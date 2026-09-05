//! Pfe aggregator trait and implementations.
//!
//! An [`PfeAggregator`] takes per-date NPVs from a single MC path and
//! combines them into a single Pfe contribution scalar that can be used
//! as the backward-pass root.

use crate::{
    ad::{dual::DualFwd, scalar::Scalar},
    time::{date::Date, daycounter::DayCounter},
};

/// Aggregates per-date NPVs from one MC path into a single Pfe contribution.
///
/// Implementations are constructed **before** `set_mark` so that their
/// `T`-typed fields (credit spread, recovery, survival probabilities, etc.)
/// become pre-mark tape leaves.  [`aggregate_path`](Self::aggregate_path) is
/// called **after** the mark inside each path iteration.
pub trait PfeAggregator<T: Scalar>: Send + Sync {
    /// Human-readable name for this measure (e.g. `"CVA"`, `"DVA"`).
    fn name(&self) -> &'static str;

    /// Combine per-date NPVs into a single Pfe contribution for one path.
    ///
    /// `npvs[d]` is the portfolio NPV at `dates[d]`.
    /// The returned `T` is the root for `backward_to_mark()`.
    fn aggregate_path(&self, npvs: &[T], dates: &[Date]) -> T;
}

/// Bundle returned by an [`PfeAggregatorFactory`]: the aggregator together
/// with tracked `DualFwd` leaves whose adjoints carry sensitivities after
/// the backward pass.
pub struct AggregatorBundle {
    /// The aggregator instance (lives on the current thread's tape).
    pub aggregator: Box<dyn PfeAggregator<DualFwd>>,
    /// Tracked leaves: `(label, leaf)` pairs whose `.adjoint()` can be read
    /// after `propagate_mark_to_start`.
    pub leaves: Vec<(String, DualFwd)>,
}

/// Factory for creating per-thread [`PfeAggregator`] instances.
///
/// Each rayon thread calls [`create_aggregator`](Self::create_aggregator) to
/// build its own aggregator with `DualFwd` leaves on the thread-local tape
/// (pre-mark).
pub trait PfeAggregatorFactory: Send + Sync {
    /// Human-readable name for the Pfe measure (e.g. `"CVA"`).
    fn name(&self) -> &'static str;

    /// Creates an aggregator and its tracked leaves on the current thread's
    /// tape.  Must be called **before** `set_mark`.
    fn create_aggregator(&self, ref_date: Date, dates: &[Date]) -> AggregatorBundle;
}

/// Unilateral CVA aggregator.
pub struct CvaAggregator<T: Scalar> {
    /// Loss-given-default: `1 − R`.
    lgd: T,
    /// Survival probabilities at each simulation date: `S(t_d) = exp(−λ t_d)`.
    survival_probs: Vec<T>,
    /// System-curve discount factors `DF(0, t_d)` at each simulation date.
    /// When absent, exposures are aggregated undiscounted.
    system_discounts: Option<Vec<f64>>,
    /// `1 / n_paths`.
    inv_n: f64,
}

impl<T: Scalar> CvaAggregator<T> {
    /// Creates a new CVA aggregator.
    ///
    /// If `credit_spread` and `recovery` are `DualFwd` leaves on the tape,
    /// credit sensitivities propagate automatically.  For no credit
    /// sensitivities, pass `T::scalar(val)` constants.
    pub fn new(
        credit_spread: T,
        recovery: T,
        n_paths: usize,
        ref_date: Date,
        dates: &[Date],
    ) -> Self {
        let lgd = T::one().sub_val(recovery);
        let hazard_rate = credit_spread.div_val(lgd);
        let dc = DayCounter::Actual365;
        let survival_probs: Vec<T> = dates
            .iter()
            .map(|d| {
                let t = dc.year_fraction(ref_date, *d);
                hazard_rate.neg_val().mul_val(T::scalar(t)).exp()
            })
            .collect();
        Self {
            lgd,
            survival_probs,
            system_discounts: None,
            inv_n: 1.0 / f64::from(u32::try_from(n_paths).unwrap_or(u32::MAX)),
        }
    }

    /// Creates a CVA aggregator directly from survival probabilities at each
    /// simulation date (e.g. interpolated from a bootstrapped credit curve).
    pub fn from_survival_probs(recovery: T, survival_probs: Vec<T>, n_paths: usize) -> Self {
        Self {
            lgd: T::one().sub_val(recovery),
            survival_probs,
            system_discounts: None,
            inv_n: 1.0 / f64::from(u32::try_from(n_paths).unwrap_or(u32::MAX)),
        }
    }

    /// Sets the system-curve discount factors `DF(0, t_d)` applied to the
    /// exposures so that CVA is expressed in present-value terms.
    #[must_use]
    pub fn with_system_discounts(mut self, discounts: Vec<f64>) -> Self {
        self.system_discounts = Some(discounts);
        self
    }
}

impl<T: Scalar> PfeAggregator<T> for CvaAggregator<T> {
    fn name(&self) -> &'static str {
        "CVA"
    }

    fn aggregate_path(&self, npvs: &[T], dates: &[Date]) -> T {
        let mut c_p = T::zero();
        let n = dates.len().min(npvs.len());
        for (d, npv) in npvs.iter().enumerate().take(n).skip(1) {
            let mut exposure = npv.max_val(T::zero());
            if let Some(dfs) = &self.system_discounts {
                exposure = exposure.mul_val(T::scalar(dfs[d]));
            }
            let delta_pd = self.survival_probs[d - 1].sub_val(self.survival_probs[d]);
            c_p = c_p.add_val(exposure.mul_val(delta_pd));
        }
        c_p.mul_val(self.lgd).mul_val(T::scalar(self.inv_n))
    }
}

/// Unilateral DVA aggregator (own-default).
pub struct DvaAggregator<T: Scalar> {
    lgd: T,
    survival_probs: Vec<T>,
    system_discounts: Option<Vec<f64>>,
    inv_n: f64,
}

impl<T: Scalar> DvaAggregator<T> {
    pub fn new(
        own_spread: T,
        own_recovery: T,
        n_paths: usize,
        ref_date: Date,
        dates: &[Date],
    ) -> Self {
        let lgd = T::one().sub_val(own_recovery);
        let hazard_rate = own_spread.div_val(lgd);
        let dc = DayCounter::Actual365;
        let survival_probs: Vec<T> = dates
            .iter()
            .map(|d| {
                let t = dc.year_fraction(ref_date, *d);
                hazard_rate.neg_val().mul_val(T::scalar(t)).exp()
            })
            .collect();
        Self {
            lgd,
            survival_probs,
            system_discounts: None,
            inv_n: 1.0 / f64::from(u32::try_from(n_paths).unwrap_or(u32::MAX)),
        }
    }

    /// Sets the system-curve discount factors `DF(0, t_d)` applied to the
    /// exposures so that DVA is expressed in present-value terms.
    #[must_use]
    pub fn with_system_discounts(mut self, discounts: Vec<f64>) -> Self {
        self.system_discounts = Some(discounts);
        self
    }
}

impl<T: Scalar> PfeAggregator<T> for DvaAggregator<T> {
    fn name(&self) -> &'static str {
        "DVA"
    }

    fn aggregate_path(&self, npvs: &[T], dates: &[Date]) -> T {
        let mut d_p = T::zero();
        let n = dates.len().min(npvs.len());
        for (d, npv) in npvs.iter().enumerate().take(n).skip(1) {
            let mut exposure = npv.neg_val().max_val(T::zero());
            if let Some(dfs) = &self.system_discounts {
                exposure = exposure.mul_val(T::scalar(dfs[d]));
            }
            let delta_pd = self.survival_probs[d - 1].sub_val(self.survival_probs[d]);
            d_p = d_p.add_val(exposure.mul_val(delta_pd));
        }
        d_p.mul_val(self.lgd).mul_val(T::scalar(self.inv_n))
    }
}

/// Funding valuation adjustment aggregator.
pub struct FvaAggregator<T: Scalar> {
    funding_spread: T,
    system_discounts: Option<Vec<f64>>,
    inv_n: f64,
}

impl<T: Scalar> FvaAggregator<T> {
    pub fn new(funding_spread: T, n_paths: usize) -> Self {
        Self {
            funding_spread,
            system_discounts: None,
            inv_n: 1.0 / f64::from(u32::try_from(n_paths).unwrap_or(u32::MAX)),
        }
    }

    /// Sets the system-curve discount factors `DF(0, t_d)` applied to the
    /// funding cost so that FVA is expressed in present-value terms.
    #[must_use]
    pub fn with_system_discounts(mut self, discounts: Vec<f64>) -> Self {
        self.system_discounts = Some(discounts);
        self
    }
}

impl<T: Scalar> PfeAggregator<T> for FvaAggregator<T> {
    fn name(&self) -> &'static str {
        "FVA"
    }

    fn aggregate_path(&self, npvs: &[T], dates: &[Date]) -> T {
        let dc = DayCounter::Actual365;
        let mut f_p = T::zero();
        for d in 1..dates.len().min(npvs.len()) {
            let dt = dc.year_fraction(dates[d - 1], dates[d]);
            let mut term = npvs[d].mul_val(self.funding_spread).mul_val(T::scalar(dt));
            if let Some(dfs) = &self.system_discounts {
                term = term.mul_val(T::scalar(dfs[d]));
            }
            f_p = f_p.add_val(term);
        }
        f_p.mul_val(T::scalar(self.inv_n))
    }
}

/// Factory for [`CvaAggregator`].
pub struct CvaFactory {
    pub credit_spread: f64,
    pub recovery: f64,
    pub n_paths: usize,
    /// System-curve discount factors `DF(0, t_d)` at the simulation dates.
    pub system_dfs: Option<Vec<f64>>,
}

impl PfeAggregatorFactory for CvaFactory {
    fn name(&self) -> &'static str {
        "CVA"
    }

    fn create_aggregator(&self, ref_date: Date, dates: &[Date]) -> AggregatorBundle {
        let cs = DualFwd::new(self.credit_spread);
        let rec = DualFwd::new(self.recovery);
        let mut agg = CvaAggregator::new(cs, rec, self.n_paths, ref_date, dates);
        if let Some(dfs) = &self.system_dfs {
            agg = agg.with_system_discounts(dfs.clone());
        }
        AggregatorBundle {
            aggregator: Box::new(agg),
            leaves: vec![
                ("CVA.credit_spread".to_string(), cs),
                ("CVA.recovery".to_string(), rec),
            ],
        }
    }
}

/// Factory for a CVA aggregator driven by a bootstrapped credit curve.
///
/// Survival probabilities at the simulation dates are log-linearly
/// interpolated between the curve pillars (flat-hazard extrapolation beyond
/// the last pillar). The pillar survivals become tracked tape leaves so that
/// the backward pass yields CVA sensitivities to the original CDS quotes
/// (labels are prefixed with `"CVA."`).
pub struct CreditCurveCvaFactory {
    /// Pillar dates of the credit curve (strictly after the reference date).
    pub pillar_dates: Vec<Date>,
    /// Survival probabilities at the pillar dates.
    pub pillar_survivals: Vec<f64>,
    /// Labels of the pillar quotes (e.g. CDS quote identifiers).
    pub pillar_labels: Vec<String>,
    /// Counterparty recovery rate.
    pub recovery: f64,
    /// Number of Monte Carlo paths.
    pub n_paths: usize,
    /// Day counter of the credit curve.
    pub day_counter: DayCounter,
    /// System-curve discount factors `DF(0, t_d)` at the simulation dates.
    pub system_dfs: Option<Vec<f64>>,
}

impl CreditCurveCvaFactory {
    /// Log-linear survival interpolation at time `t` (year fraction from the
    /// reference date), with flat-hazard extrapolation beyond the last pillar.
    fn survival_at(t: f64, pillar_times: &[f64], leaves: &[DualFwd]) -> DualFwd {
        let n = pillar_times.len();
        if t <= 0.0 || n == 0 {
            return DualFwd::scalar(1.0);
        }
        // Before or at the first pillar: interpolate between (0, S=1) and t_1.
        if t <= pillar_times[0] {
            let w = t / pillar_times[0];
            return leaves[0].ln().mul_val(DualFwd::scalar(w)).exp();
        }
        // Between pillars.
        for k in 1..n {
            if t <= pillar_times[k] {
                let w = (t - pillar_times[k - 1]) / (pillar_times[k] - pillar_times[k - 1]);
                let ln_s = leaves[k - 1].ln().add_val(
                    leaves[k]
                        .ln()
                        .sub_val(leaves[k - 1].ln())
                        .mul_val(DualFwd::scalar(w)),
                );
                return ln_s.exp();
            }
        }
        // Beyond the last pillar: flat hazard from the last bucket.
        let last = n - 1;
        let (t_prev, ln_prev) = if n >= 2 {
            (pillar_times[last - 1], leaves[last - 1].ln())
        } else {
            (0.0, DualFwd::scalar(0.0))
        };
        let dt_bucket = pillar_times[last] - t_prev;
        let hazard = ln_prev
            .sub_val(leaves[last].ln())
            .div_val(DualFwd::scalar(dt_bucket));
        leaves[last]
            .ln()
            .sub_val(hazard.mul_val(DualFwd::scalar(t - pillar_times[last])))
            .exp()
    }
}

impl PfeAggregatorFactory for CreditCurveCvaFactory {
    fn name(&self) -> &'static str {
        "CVA"
    }

    fn create_aggregator(&self, ref_date: Date, dates: &[Date]) -> AggregatorBundle {
        // Tracked leaves: one per pillar survival + recovery.
        let pillar_leaves: Vec<DualFwd> = self
            .pillar_survivals
            .iter()
            .map(|s| DualFwd::new(*s))
            .collect();
        let rec = DualFwd::new(self.recovery);

        let pillar_times: Vec<f64> = self
            .pillar_dates
            .iter()
            .map(|d| self.day_counter.year_fraction(ref_date, *d))
            .collect();

        let survival_probs: Vec<DualFwd> = dates
            .iter()
            .map(|d| {
                let t = self.day_counter.year_fraction(ref_date, *d);
                Self::survival_at(t, &pillar_times, &pillar_leaves)
            })
            .collect();

        let mut agg = CvaAggregator::from_survival_probs(rec, survival_probs, self.n_paths);
        if let Some(dfs) = &self.system_dfs {
            agg = agg.with_system_discounts(dfs.clone());
        }

        let mut leaves: Vec<(String, DualFwd)> = self
            .pillar_labels
            .iter()
            .zip(&pillar_leaves)
            .map(|(label, leaf)| (format!("CVA.{label}"), *leaf))
            .collect();
        leaves.push(("CVA.recovery".to_string(), rec));

        AggregatorBundle {
            aggregator: Box::new(agg),
            leaves,
        }
    }
}

/// Factory for [`DvaAggregator`].
pub struct DvaFactory {
    pub own_spread: f64,
    pub own_recovery: f64,
    pub n_paths: usize,
}

impl PfeAggregatorFactory for DvaFactory {
    fn name(&self) -> &'static str {
        "DVA"
    }

    fn create_aggregator(&self, ref_date: Date, dates: &[Date]) -> AggregatorBundle {
        let sp = DualFwd::new(self.own_spread);
        let rec = DualFwd::new(self.own_recovery);
        let agg = DvaAggregator::new(sp, rec, self.n_paths, ref_date, dates);
        AggregatorBundle {
            aggregator: Box::new(agg),
            leaves: vec![
                ("DVA.own_spread".to_string(), sp),
                ("DVA.own_recovery".to_string(), rec),
            ],
        }
    }
}

/// Factory for [`FvaAggregator`].
pub struct FvaFactory {
    pub funding_spread: f64,
    pub n_paths: usize,
    /// System-curve discount factors `DF(0, t_d)` at the simulation dates.
    pub system_dfs: Option<Vec<f64>>,
}

impl PfeAggregatorFactory for FvaFactory {
    fn name(&self) -> &'static str {
        "FVA"
    }

    fn create_aggregator(&self, _ref_date: Date, _dates: &[Date]) -> AggregatorBundle {
        let fs = DualFwd::new(self.funding_spread);
        let mut agg = FvaAggregator::new(fs, self.n_paths);
        if let Some(dfs) = &self.system_dfs {
            agg = agg.with_system_discounts(dfs.clone());
        }
        AggregatorBundle {
            aggregator: Box::new(agg),
            leaves: vec![("FVA.funding_spread".to_string(), fs)],
        }
    }
}
