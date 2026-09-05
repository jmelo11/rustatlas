//! Closed-form CDS pricer.
//!
//! Prices a [`CreditDefaultSwap`] off a bootstrapped survival curve
//! ([`MarketIndex::Credit`]) and a discount curve. When a
//! [`DiscountPolicy`] is set, discounting uses the CSA collateral curve of
//! the counterparty instead of the instrument's own `discount_index`.
//!
//! Leg conventions (per unit notional, protection buyer):
//! - premium leg: `spread · Σ Δ_i · DF(d_i) · (S(d_i) + ½·(S(d_{i-1}) − S(d_i)))`
//!   (running premium on survival plus half-period accrual-on-default),
//! - protection leg: `(1 − R) · Σ DF(d_i) · (S(d_{i-1}) − S(d_i))`.
//!
//! Value = `side.sign() · notional · (protection − premium)`, so a
//! [`Side::LongReceive`](crate::core::trade::Side) trade is a protection buyer.

use crate::{
    ad::{dual::DualFwd, scalar::Scalar, tape::Tape},
    core::{
        collateral::DiscountPolicy,
        evaluationresults::{EvaluationResults, SensitivityMap},
        instrument::Instrument,
        marketdatahandling::{
            constructedelementrequest::ConstructedElementRequest,
            marketdata::{MarketData, MarketDataProvider, MarketDataRequest},
        },
        pricer::Pricer,
        pricerstate::PricerState,
        request::{HandleFairRate, HandleSensitivities, HandleValue, Request},
        trade::Trade,
    },
    instruments::credit::creditdefaultswap::CdsTrade,
    time::{date::Date, schedule::MakeSchedule},
    utils::errors::{QSError, Result},
};
use std::collections::HashSet;

/// Prices a credit default swap using survival probabilities from a
/// bootstrapped credit curve.
pub struct CdsPricer {
    discount_policy: Option<Box<dyn DiscountPolicy>>,
}

impl CdsPricer {
    /// Creates a new [`CdsPricer`].
    #[must_use]
    pub fn new() -> Self {
        Self {
            discount_policy: None,
        }
    }
}

impl Default for CdsPricer {
    fn default() -> Self {
        Self::new()
    }
}

/// State for CDS pricing.
#[derive(Default)]
struct CdsState {
    value: Option<DualFwd>,
    market_data: Option<MarketData>,
}

impl PricerState for CdsState {
    fn get_market_data_reponse(&self) -> Option<&MarketData> {
        self.market_data.as_ref()
    }

    fn get_market_data_reponse_mut(&mut self) -> Option<&mut MarketData> {
        self.market_data.as_mut()
    }
}

impl CdsPricer {
    fn resolve_discount_index(
        &self,
        trade: &CdsTrade,
    ) -> Result<crate::indices::marketindex::MarketIndex> {
        let cds = trade.instrument();
        self.discount_policy.as_ref().map_or_else(
            || Ok(cds.discount_index().clone()),
            |policy| policy.accept(cds),
        )
    }

    /// Computes the premium annuity (risky PV01 including accrual-on-default,
    /// per unit spread) and protection leg PV (per unit notional) as AD values.
    fn legs(&self, trade: &CdsTrade, state: &CdsState) -> Result<(DualFwd, DualFwd)> {
        let cds = trade.instrument();
        let discount_index = self.resolve_discount_index(trade)?;
        let discount_curve = state.get_discount_curve_element(&discount_index)?;
        let credit_curve = state.get_credit_curve_element(cds.credit_index())?;

        let schedule = MakeSchedule::new(cds.start_date(), cds.maturity_date())
            .with_frequency(cds.premium_frequency())
            .build()?;

        let discount = discount_curve.curve();
        let credit = credit_curve.curve();
        let ref_date: Date = discount.reference_date();

        let mut annuity = DualFwd::scalar(0.0);
        let mut protection = DualFwd::scalar(0.0);
        let mut s_prev: DualFwd = if schedule.dates()[0] > ref_date {
            credit.discount_factor(schedule.dates()[0])?
        } else {
            DualFwd::scalar(1.0)
        };
        for w in schedule.dates().windows(2) {
            let (d0, d1) = (w[0], w[1]);
            if d1 <= ref_date {
                continue;
            }
            let delta = cds.day_counter().year_fraction(d0, d1);
            let df: DualFwd = discount.discount_factor(d1)?;
            let s1: DualFwd = credit.discount_factor(d1)?;
            let default_prob: DualFwd = (s_prev - s1).into();
            let period_annuity: DualFwd =
                (df * (s1 + default_prob * DualFwd::scalar(0.5)) * DualFwd::scalar(delta)).into();
            annuity = (annuity + period_annuity).into();
            protection = (protection + df * default_prob).into();
            s_prev = s1;
        }
        let lgd = DualFwd::scalar(1.0 - cds.recovery());
        Ok((annuity, (protection * lgd).into()))
    }
}

impl HandleValue<CdsTrade, CdsState> for CdsPricer {
    fn handle_value(&self, trade: &CdsTrade, state: &mut CdsState) -> Result<f64> {
        let cds = trade.instrument();

        Tape::start_recording_fwd();
        Tape::set_mark_fwd();
        state.put_pillars_on_tape()?;

        let (annuity, protection) = self.legs(trade, state)?;
        let premium: DualFwd = (annuity * DualFwd::scalar(cds.spread())).into();
        let sign = trade.side().sign();
        let value: DualFwd =
            ((protection - premium) * DualFwd::scalar(sign * trade.notional())).into();
        state.value = Some(value);

        Tape::stop_recording_fwd();
        Ok(value.value())
    }
}

impl HandleFairRate<CdsTrade, CdsState> for CdsPricer {
    fn handle_fair_rate(&self, trade: &CdsTrade, state: &mut CdsState) -> Result<f64> {
        Tape::start_recording_fwd();
        Tape::set_mark_fwd();
        state.put_pillars_on_tape()?;
        let (annuity, protection) = self.legs(trade, state)?;
        Tape::stop_recording_fwd();
        // Restarting the tape invalidates any previously cached value node.
        state.value = None;
        if annuity.value().abs() < 1e-16 {
            return Err(QSError::InvalidValueErr(
                "CDS risky annuity is zero; cannot compute par spread".into(),
            ));
        }
        Ok(protection.value() / annuity.value())
    }
}

impl HandleSensitivities<CdsTrade, CdsState> for CdsPricer {
    fn handle_sensitivities(
        &self,
        trade: &CdsTrade,
        state: &mut CdsState,
    ) -> Result<SensitivityMap> {
        let value = if let Some(v) = state.value {
            v
        } else {
            let _ = self.handle_value(trade, state)?;
            state.value.ok_or_else(|| {
                QSError::UnexpectedErr(
                    "State does not contain price after value computation.".into(),
                )
            })?
        };

        value.backward_to_mark()?;

        let cds = trade.instrument();
        let discount_index = self.resolve_discount_index(trade)?;

        let mut ids = Vec::new();
        let mut exposures = Vec::new();

        // Discount curve sensitivities.
        for (label, pillar) in state
            .get_discount_curve_element(&discount_index)?
            .curve()
            .pillars()
            .unwrap_or_default()
        {
            ids.push(label);
            exposures.push(pillar.adjoint()?.value());
        }

        // Credit curve sensitivities (to the CDS quotes via IFT).
        for (label, pillar) in state
            .get_credit_curve_element(cds.credit_index())?
            .curve()
            .pillars()
            .unwrap_or_default()
        {
            ids.push(label);
            exposures.push(pillar.adjoint()?.value());
        }

        Ok(SensitivityMap::default()
            .with_instrument_keys(&ids)
            .with_exposure(&exposures)
            .aggregate())
    }
}

impl Pricer for CdsPricer {
    type Item = CdsTrade;
    type Policy = dyn DiscountPolicy;

    fn evaluate(
        &self,
        trade: &CdsTrade,
        requests: &[Request],
        ctx: &impl MarketDataProvider,
    ) -> Result<EvaluationResults> {
        let eval_date = ctx.evaluation_date();
        let identifier = trade.instrument().identifier();

        let md_request = self
            .market_data_request(trade)
            .ok_or_else(|| QSError::InvalidValueErr("Missing market data request".into()))?;

        let mut results = EvaluationResults::new(eval_date, identifier);
        let mut state = CdsState {
            value: None,
            market_data: Some(ctx.handle_request(&md_request)?),
        };

        for request in requests {
            match request {
                Request::Value => {
                    let price = self.handle_value(trade, &mut state)?;
                    results = results.with_price(price);
                }
                Request::FairRate => {
                    let fair_rate = self.handle_fair_rate(trade, &mut state)?;
                    results = results.with_fair_rate(fair_rate);
                }
                Request::Sensitivities => {
                    let sensitivities = self.handle_sensitivities(trade, &mut state)?;
                    results = results.with_sensitivities(sensitivities);
                }
                _ => {}
            }
        }

        Ok(results)
    }

    fn market_data_request(&self, trade: &CdsTrade) -> Option<MarketDataRequest> {
        let cds = trade.instrument();
        let mut elements = vec![
            ConstructedElementRequest::DiscountCurve {
                market_index: cds.discount_index().clone(),
            },
            ConstructedElementRequest::CreditCurve {
                market_index: cds.credit_index().clone(),
            },
        ];

        let mut seen_indices = HashSet::new();
        seen_indices.insert(cds.discount_index().clone());

        if let Some(policy) = &self.discount_policy {
            for policy_index in policy.discount_indices() {
                if seen_indices.insert(policy_index.clone()) {
                    elements.push(ConstructedElementRequest::DiscountCurve {
                        market_index: policy_index,
                    });
                }
            }
        }

        let request = MarketDataRequest::default().with_constructed_elements_request(elements);
        Some(request)
    }

    fn set_discount_policy(&mut self, policy: Box<Self::Policy>) {
        self.discount_policy = Some(policy);
    }

    fn discount_policy(&self) -> Option<&Self::Policy> {
        self.discount_policy.as_deref()
    }
}
