use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    ad::{dual::DualFwd, scalar::Scalar},
    core::{pillars::Pillars, pricingcontext::PricingContext},
    currencies::currency::Currency,
    indices::marketindex::MarketIndex,
    math::interpolation::interpolator::Interpolator,
    models::lgm::{
        lgmcomponents::{LgmFxModel, LgmRateModel},
        lgmmarketmodel::LgmMarketModel,
    },
    quotes::fixingstore::FixingStore,
    rates::yieldtermstructure::{
        discounttermstructure::DiscountTermStructure,
        interestratestermstructure::InterestRatesTermStructure,
    },
    time::{
        date::Date,
        daycounter::DayCounter,
        enums::{Frequency, TimeUnit},
        schedule::MakeSchedule,
    },
    utils::errors::{QSError, Result},
    xva::{
        aggregator::{CreditCurveCvaFactory, CvaFactory, FvaFactory, PfeAggregatorFactory},
        contigentclaim::ContingentClaim,
        nettingset::NettingSet,
        visitors::{
            exposureevaluator::{evaluate_with_xva, ExposureResult, ModelCallback, XvaModelSetup},
            fixingpreprocessor::FixingPreprocessor,
            marketmodel::MarketModel,
            preprocessorexecutor::{PreprocessorExecutor, SimulationRequest},
        },
    },
};

/// LGM model parameters for a single rate curve.
#[derive(Clone, Serialize, Deserialize)]
pub struct LgmModelConfig {
    pub market_index: MarketIndex,
    pub lambda: f64,
    pub sigma: f64,
}

/// FX model parameters for a single currency pair.
#[derive(Clone, Serialize, Deserialize)]
pub struct FxModelConfig {
    /// Foreign currency (domestic is always the engine's base currency).
    pub foreign_currency: Currency,
    /// FX volatility.
    pub fx_vol: f64,
    /// Correlation between domestic rate factor and FX spot.
    #[serde(default)]
    pub rho: f64,
}

/// Configuration for the XVA engine.
///
/// Contains only the simulation/model setup. Credit, funding and
/// collateral (CSA) parameters are per client and belong to each
/// [`NettingSet`]'s [`CsaTerms`](crate::xva::csa::CsaTerms).
#[derive(Clone, Serialize, Deserialize)]
pub struct XvaEngineConfig {
    /// LGM model parameters, one per rate curve.
    pub model_configs: Vec<LgmModelConfig>,
    /// FX model parameters, one per foreign currency.
    #[serde(default)]
    pub fx_configs: Vec<FxModelConfig>,
    /// Number of Monte Carlo paths.
    pub n_paths: usize,
    /// RNG seed.
    pub seed: u64,
    /// Simulation frequency (e.g. Monthly, Quarterly).
    pub frequency: Frequency,
}

/// High-level XVA engine.
///
/// Takes a fully initialised [`PricingContext`] (with bootstrapped curves)
/// and an [`XvaEngineConfig`], then runs the Savine parallel AAD loop to
/// produce exposure cubes, XVA values, and sensitivities.
///
/// # Example
/// ```ignore
/// let mut ctx = PricingContext::new()
///     .with_quote_store(quotes)
///     .with_curve_configurations(curve_specs);
/// ctx.initialize()?;
///
/// let config = XvaEngineConfig { /* ... */ };
/// let engine = XvaEngine::new(&ctx, config)?;
/// let result = engine.run(&mut trades)?;
/// ```
pub struct XvaEngine {
    setup: InternalModelSetup,
    frequency: Frequency,
    /// Snapshots of the bootstrapped credit (survival) curves, keyed by
    /// [`MarketIndex::Credit`]. Used to build per-counterparty CVA
    /// aggregators when a netting set's CSA references a credit curve.
    credit_curves: HashMap<MarketIndex, CreditCurveSnapshot>,
}

impl XvaEngine {
    /// Creates a new engine from an initialised [`PricingContext`].
    ///
    /// Snapshots the f64 curve data from every discount curve referenced
    /// in `config.model_configs`. The curves must already be bootstrapped
    /// in the context.
    ///
    /// # Errors
    /// Returns an error if a required discount curve is missing or has no nodes.
    pub fn new(context: &PricingContext, config: XvaEngineConfig) -> Result<Self> {
        let store = context.constructed_elements();

        let mut curves = HashMap::new();
        let mut model_configs = HashMap::new();

        for mc in &config.model_configs {
            let elem = store.discount_curve(&mc.market_index).ok_or_else(|| {
                QSError::NotFoundErr(format!(
                    "Discount curve not found for index {:?}",
                    mc.market_index
                ))
            })?;

            // Snapshot f64 data from the already-bootstrapped curve.
            let borrowed = elem.curve();
            let nodes = borrowed
                .nodes()
                .ok_or_else(|| QSError::NotFoundErr("Curve has no nodes".into()))?;
            let dates: Vec<Date> = nodes.iter().map(|(d, _)| *d).collect();
            let dfs: Vec<f64> = nodes.iter().map(|(_, v)| v.value()).collect();
            let pillar_labels = borrowed.pillar_labels().unwrap_or_default();
            let pillar_values: Vec<f64> = borrowed
                .pillars()
                .unwrap_or_default()
                .iter()
                .map(|(_, v)| v.value())
                .collect();
            let dc = borrowed.day_counter().unwrap_or(DayCounter::Actual365);

            curves.insert(
                mc.market_index.clone(),
                CurveSnapshot {
                    dates,
                    discount_factors: dfs,
                    day_counter: dc,
                    interpolator: Interpolator::LogLinear,
                    pillar_labels,
                    pillar_values,
                },
            );
            model_configs.insert(mc.market_index.clone(), mc.clone());
        }

        // Snapshot FX spots from the FxStore.
        let mut fx_spots = HashMap::new();
        let fx_store = context.fx_store();
        let domestic = context.base_currency();
        for fx_cfg in &config.fx_configs {
            // Get rate as: 1 foreign = X domestic (e.g. 1 CLP = 0.00111 USD)
            let rate = fx_store
                .get_fx_rate(fx_cfg.foreign_currency, domestic)
                .map_err(|_| {
                    QSError::NotFoundErr(format!(
                        "FX spot not found for {}/{}",
                        fx_cfg.foreign_currency, domestic
                    ))
                })?;
            fx_spots.insert(fx_cfg.foreign_currency, rate.value());
        }

        // Snapshot bootstrapped credit (survival) curves for per-counterparty CVA.
        let mut credit_curves = HashMap::new();
        for (index, element) in store.credit_curves() {
            let borrowed = element.curve();
            let nodes = borrowed.nodes().ok_or_else(|| {
                QSError::NotFoundErr(format!("Credit curve {index} has no nodes"))
            })?;
            // Node 0 is the reference date with S = 1; pillars follow.
            let pillar_dates: Vec<Date> = nodes.iter().skip(1).map(|(d, _)| *d).collect();
            let survivals: Vec<f64> = nodes.iter().skip(1).map(|(_, v)| v.value()).collect();
            let mut labels = borrowed.pillar_labels().unwrap_or_default();
            if labels.len() != pillar_dates.len() {
                labels = (0..pillar_dates.len())
                    .map(|i| format!("{index}.pillar_{i}"))
                    .collect();
            }
            let dc = borrowed.day_counter().unwrap_or(DayCounter::Actual365);
            credit_curves.insert(
                index.clone(),
                CreditCurveSnapshot {
                    pillar_dates,
                    survivals,
                    labels,
                    day_counter: dc,
                },
            );
        }

        Ok(Self {
            setup: InternalModelSetup {
                curves,
                model_configs,
                fx_configs: config.fx_configs,
                fx_spots,
                domestic_currency: domestic,
                domestic_index: context.base_index().clone(),
                reference_date: context.evaluation_date(),
                day_counter: DayCounter::Actual365,
                n_paths: config.n_paths,
                seed: config.seed,
                requests: Vec::new(), // filled in run()
                fixing_store: context.fixing_store().clone(),
            },
            frequency: config.frequency,
            credit_curves,
        })
    }

    /// Runs the full XVA pipeline.
    ///
    /// Builds a [`PreprocessorExecutor`] with preprocessing steps (fixing resolution),
    /// runs it on all netting sets, then launches the Savine
    /// parallel AAD evaluation loop.
    ///
    /// Each [`NettingSet`] carries its own [`CsaTerms`](crate::xva::csa::CsaTerms)
    /// (collateral discounting plus credit/funding parameters), from which the
    /// per-client CVA and FVA aggregators are built.
    ///
    /// # Errors
    /// Returns an error if any netting set lacks CSA terms, or if simulation
    /// or evaluation fails.
    pub fn run(
        &mut self,
        netting_sets: &mut HashMap<String, NettingSet>,
    ) -> Result<ExposureResult> {
        // 1. Build preprocessor pipeline.
        let fixing_pp = FixingPreprocessor::new(
            self.setup.reference_date,
            self.setup.day_counter,
            self.setup.fixing_store.clone(),
        );

        let mut inspector = PreprocessorExecutor::new().with_preprocessor(Box::new(fixing_pp));

        // 2. Visit all netting sets in-place, assigning global indices.
        inspector.visit(netting_sets.values_mut());
        self.setup.requests = inspector.requests().to_vec();

        // 2b. Validate that every resolved discount index has a simulated
        // curve. The LGM model silently skips unknown curves, which would
        // otherwise produce missing discounts downstream.
        for req in &self.setup.requests {
            if let Some(discount_request) = &req.discount_request {
                let index = discount_request.market_index();
                if !self.setup.curves.contains_key(&index) {
                    return Err(QSError::NotFoundErr(format!(
                        "Discount curve {index} resolved by a CSA discount policy is not \
                         configured in the XVA engine; add an LgmModelConfig (and bootstrap \
                         the curve) for it"
                    )));
                }
            }
        }

        // 3. Build simulation dates.
        let max_maturity = netting_sets
            .values()
            .flat_map(|ns| ns.claims().iter())
            .map(ContingentClaim::payment_date)
            .max()
            .unwrap_or_else(|| self.setup.reference_date.advance(1, TimeUnit::Years));

        let schedule = MakeSchedule::new(self.setup.reference_date, max_maturity)
            .with_frequency(self.frequency)
            .build()?;
        let sim_dates = schedule.dates().clone();

        // 3b. System-curve discount factors DF(0, t_d) at the simulation
        // dates, taken from the engine's base (domestic) curve. Exposures at
        // future dates are values as of t_d; multiplying by these
        // deterministic DFs expresses every XVA in present-value terms on the
        // system curve. (Deterministic approximation: no rate sensitivity is
        // propagated through this discounting term.)
        let system_curve = self
            .setup
            .curves
            .get(&self.setup.domestic_index)
            .ok_or_else(|| {
                QSError::NotFoundErr(format!(
                    "System (base) discount curve {} not configured in the XVA engine",
                    self.setup.domestic_index
                ))
            })?;
        let system_ts = DiscountTermStructure::<f64>::new(
            system_curve.dates.clone(),
            system_curve.discount_factors.clone(),
            system_curve.day_counter,
            system_curve.interpolator,
            true,
        )?;
        let system_dfs: Vec<f64> = sim_dates
            .iter()
            .map(|d| system_ts.discount_factor(*d))
            .collect::<Result<Vec<f64>>>()?;

        // 4. Per-netting-set aggregator factories from each client's CSA terms.
        let mut factories: HashMap<String, Vec<Box<dyn PfeAggregatorFactory>>> = HashMap::new();
        for (id, ns) in netting_sets.iter() {
            let csa = ns.csa_terms().ok_or_else(|| {
                QSError::InvalidValueErr(format!(
                    "Netting set '{id}' has no CSA terms; build it with NettingSet::with_csa_terms"
                ))
            })?;

            // CVA: bootstrapped credit curve when assigned, flat spread otherwise.
            let cva_factory: Box<dyn PfeAggregatorFactory> =
                if let Some(credit_index) = &csa.credit_index {
                    let snapshot = self.credit_curves.get(credit_index).ok_or_else(|| {
                        QSError::NotFoundErr(format!(
                            "Netting set '{id}' references credit curve {credit_index}, but it \
                             was not bootstrapped in the pricing context"
                        ))
                    })?;
                    Box::new(CreditCurveCvaFactory {
                        pillar_dates: snapshot.pillar_dates.clone(),
                        pillar_survivals: snapshot.survivals.clone(),
                        pillar_labels: snapshot.labels.clone(),
                        recovery: csa.recovery,
                        n_paths: self.setup.n_paths,
                        day_counter: snapshot.day_counter,
                        system_dfs: Some(system_dfs.clone()),
                    })
                } else {
                    Box::new(CvaFactory {
                        credit_spread: csa.credit_spread,
                        recovery: csa.recovery,
                        n_paths: self.setup.n_paths,
                        system_dfs: Some(system_dfs.clone()),
                    })
                };

            factories.insert(
                id.clone(),
                vec![
                    cva_factory,
                    Box::new(FvaFactory {
                        funding_spread: csa.funding_spread,
                        n_paths: self.setup.n_paths,
                        system_dfs: Some(system_dfs.clone()),
                    }),
                ],
            );
        }

        // 5. Build netting-set slice map.
        let ns_slices: HashMap<String, &[_]> = netting_sets
            .iter()
            .map(|(id, ns)| (id.clone(), ns.claims()))
            .collect();

        // 6. Run.
        evaluate_with_xva(&sim_dates, &ns_slices, &factories, &self.setup)
    }
}

/// Snapshot of f64 curve data extracted from the `PricingContext`.
/// Each rayon thread uses this to build a thread-local `DualFwd` curve.
#[derive(Clone)]
struct CurveSnapshot {
    dates: Vec<Date>,
    discount_factors: Vec<f64>,
    day_counter: DayCounter,
    interpolator: Interpolator,
    pillar_labels: Vec<String>,
    pillar_values: Vec<f64>,
}

/// Snapshot of a bootstrapped credit (survival) curve. The reference-date
/// node (`S = 1`) is excluded.
#[derive(Clone)]
struct CreditCurveSnapshot {
    pillar_dates: Vec<Date>,
    survivals: Vec<f64>,
    labels: Vec<String>,
    day_counter: DayCounter,
}

impl CurveSnapshot {
    /// Build a `DiscountTermStructure<DualFwd>` on the current thread's tape.
    ///
    /// # Errors
    /// Returns an error if curve construction or pillar assignment fails.
    fn build_dualfwd_curve(&self) -> Result<DiscountTermStructure<DualFwd>> {
        let dfs: Vec<DualFwd> = self
            .discount_factors
            .iter()
            .map(|&v| DualFwd::scalar(v))
            .collect();
        let pvs: Vec<DualFwd> = self
            .pillar_values
            .iter()
            .map(|&v| DualFwd::scalar(v))
            .collect();

        let mut curve = DiscountTermStructure::<DualFwd>::new(
            self.dates.clone(),
            dfs,
            self.day_counter,
            self.interpolator,
            true,
        )?
        .with_pillar_values(pvs)?
        .with_pillar_labels(self.pillar_labels.clone())?;

        curve.put_pillars_on_tape();
        Ok(curve)
    }
}

/// Internal model setup implementing `XvaModelSetup`.
struct InternalModelSetup {
    curves: HashMap<MarketIndex, CurveSnapshot>,
    model_configs: HashMap<MarketIndex, LgmModelConfig>,
    fx_configs: Vec<FxModelConfig>,
    fx_spots: HashMap<Currency, f64>,
    domestic_currency: Currency,
    domestic_index: MarketIndex,
    reference_date: Date,
    day_counter: DayCounter,
    n_paths: usize,
    seed: u64,
    requests: Vec<SimulationRequest>,
    fixing_store: FixingStore,
}

// Safety: all fields are owned plain data (Vec, HashMap, f64, etc.). No Rc/RefCell.
unsafe impl Send for InternalModelSetup {}
unsafe impl Sync for InternalModelSetup {}

impl XvaModelSetup for InternalModelSetup {
    fn n_paths(&self) -> usize {
        self.n_paths
    }

    fn with_model<R>(&self, dates: &[Date], callback: &mut ModelCallback<'_, R>) -> Result<R> {
        // 1. Build DualFwd curves and collect leaves.
        let mut built_curves: Vec<(MarketIndex, DiscountTermStructure<DualFwd>)> = Vec::new();
        let mut all_leaves: Vec<(String, DualFwd)> = Vec::new();

        for (idx, snap) in &self.curves {
            let curve = snap.build_dualfwd_curve()?;
            let leaves: Vec<(String, DualFwd)> = curve
                .pillars()
                .unwrap_or_default()
                .into_iter()
                .map(|(label, &val)| (label, val))
                .collect();
            all_leaves.extend(leaves);
            built_curves.push((idx.clone(), curve));
        }

        // 2. Build rate models for curve_models (moved into the market model).
        //    Also build separate rate model instances for FX model references.
        let mut fx_rate_models: Vec<(MarketIndex, LgmRateModel<'_, DualFwd>)> = Vec::new();

        let mut model = LgmMarketModel::new(
            self.domestic_currency,
            self.domestic_index.clone(),
            self.reference_date,
            self.day_counter,
        )
        .with_n_paths(self.n_paths)
        .with_seed(self.seed);

        for (idx, curve) in &built_curves {
            let cfg = self.model_configs.get(idx).ok_or_else(|| {
                QSError::NotFoundErr(format!("Model config missing for curve {idx:?}"))
            })?;
            let rate_model = LgmRateModel::new(
                DualFwd::scalar(cfg.lambda),
                DualFwd::scalar(cfg.sigma),
                curve,
            );
            model.add_curve_model(idx.clone(), rate_model);

            // If any FX config references this curve's currency, build an extra
            // rate model for the FX model to borrow.
            if !self.fx_configs.is_empty() {
                let fx_rate = LgmRateModel::new(
                    DualFwd::scalar(cfg.lambda),
                    DualFwd::scalar(cfg.sigma),
                    curve,
                );
                fx_rate_models.push((idx.clone(), fx_rate));
            }
        }

        // 3. Build FX models from the separate rate model instances.
        //    Find domestic and foreign rate models by index.
        let find_fx_rate = |idx: &MarketIndex| -> Option<usize> {
            fx_rate_models.iter().position(|(i, _)| i == idx)
        };

        for fx_cfg in &self.fx_configs {
            let dom_pos = find_fx_rate(&self.domestic_index).ok_or_else(|| {
                QSError::NotFoundErr("Domestic rate model not found for FX".into())
            })?;
            // Find the foreign index by currency
            let foreign_index = self
                .model_configs
                .iter()
                .find(|(_, mc)| {
                    mc.market_index
                        .rate_index_details()
                        .is_ok_and(|d| d.currency() == fx_cfg.foreign_currency)
                })
                .map(|(_, mc)| &mc.market_index)
                .ok_or_else(|| {
                    QSError::NotFoundErr("Foreign rate model not found for FX".into())
                })?;
            let for_pos = find_fx_rate(foreign_index).ok_or_else(|| {
                QSError::NotFoundErr("Foreign rate model not found for FX".into())
            })?;

            // SAFETY: dom_pos != for_pos (domestic != foreign currency).
            // We need two simultaneous immutable borrows from the Vec.
            let (dom_rate, for_rate) = if dom_pos < for_pos {
                let (left, right) = fx_rate_models.split_at(for_pos);
                (&left[dom_pos].1, &right[0].1)
            } else {
                let (left, right) = fx_rate_models.split_at(dom_pos);
                (&right[0].1, &left[for_pos].1)
            };

            let spot = *self.fx_spots.get(&fx_cfg.foreign_currency).ok_or_else(|| {
                QSError::NotFoundErr(format!(
                    "FX spot missing for currency {}",
                    fx_cfg.foreign_currency
                ))
            })?;

            let fx_model = LgmFxModel::new(
                dom_rate,
                for_rate,
                DualFwd::scalar(fx_cfg.fx_vol),
                DualFwd::scalar(spot),
                DualFwd::scalar(fx_cfg.rho),
            );
            model.add_fx_model(fx_cfg.foreign_currency, fx_model);
        }

        model.set_evaluation_dates(dates.to_vec());
        model.set_requests(self.requests.clone());

        callback(&model, &all_leaves)
    }
}
