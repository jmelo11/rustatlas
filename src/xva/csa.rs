//! CSA (Credit Support Annex) terms — per-client collateral and credit configuration.
//!
//! Each counterparty (netting agreement) carries its own [`CsaTerms`]
//! describing the collateral treatment (which curve remunerates posted
//! collateral) and the credit/funding parameters used for CVA and FVA.
//! The struct is serde-deserializable so it can be loaded straight from
//! JSON configuration files.

use serde::{Deserialize, Serialize};

use crate::{
    core::collateral::SingleCurveCSADiscountPolicy, currencies::currency::Currency,
    indices::marketindex::MarketIndex,
};

/// Per-client CSA and credit configuration.
///
/// Defines the collateral treatment (discounting) and the credit/funding
/// parameters of a single netting agreement:
///
/// ```json
/// {
///     "collateral_index": "SOFR",
///     "collateral_currency": "USD",
///     "credit_spread": 0.01,
///     "recovery": 0.4,
///     "funding_spread": 0.005,
///     "credit_index": "Credit(ACME)"
/// }
/// ```
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CsaTerms {
    /// Remuneration index of the posted collateral. Trades in other
    /// currencies are discounted with a collateral-adjusted curve.
    pub collateral_index: MarketIndex,
    /// Currency of the posted collateral.
    pub collateral_currency: Currency,
    /// Counterparty credit spread used for CVA when no bootstrapped credit
    /// curve is assigned (see [`Self::credit_index`]).
    pub credit_spread: f64,
    /// Counterparty recovery rate used for CVA.
    pub recovery: f64,
    /// Funding spread used for FVA.
    #[serde(default)]
    pub funding_spread: f64,
    /// Bootstrapped credit curve of the counterparty. When set, CVA uses the
    /// survival probabilities of this curve (with sensitivities to the CDS
    /// quotes) instead of the flat `credit_spread`.
    #[serde(default)]
    pub credit_index: Option<MarketIndex>,
}

impl CsaTerms {
    /// Builds the discount policy implied by the collateral terms.
    #[must_use]
    pub fn discount_policy(&self) -> SingleCurveCSADiscountPolicy {
        SingleCurveCSADiscountPolicy::new(self.collateral_index.clone(), self.collateral_currency)
    }
}
