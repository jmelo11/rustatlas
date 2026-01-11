use serde::{Deserialize, Serialize};

use super::instrument::RateType;
use super::traits::Structure;
use crate::{
    cashflows::cashflow::{Cashflow, Side},
    core::traits::HasCurrency,
    currencies::enums::Currency,
    rates::interestrate::RateDefinition,
    time::{date::Date, enums::Frequency},
    utils::errors::{AtlasError, Result},
    visitors::traits::HasCashflows,
};
/// A hybrid rate instrument that combines fixed and floating rate components.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HybridRateInstrument {
    start_date: Date,
    end_date: Date,
    notional: f64,
    payment_frequency: Frequency,
    structure: Structure,
    rate_type: RateType,
    side: Option<Side>,
    currency: Option<Currency>,
    id: Option<String>,
    issue_date: Option<Date>,
    first_rate_definition: Option<RateDefinition>,
    first_rate: Option<f64>,
    second_rate_definition: Option<RateDefinition>,
    second_rate: Option<f64>,
    forecast_curve_id: Option<usize>,
    discount_curve_id: Option<usize>,
    cashflows: Vec<Cashflow>,
}

impl HybridRateInstrument {
    /// Creates a new hybrid rate instrument.
    #[allow(clippy::missing_const_for_fn)]
    #[must_use]
    pub fn new(
        start_date: Date,
        end_date: Date,
        notional: f64,
        payment_frequency: Frequency,
        structure: Structure,
        side: Option<Side>,
        currency: Option<Currency>,
        id: Option<String>,
        issue_date: Option<Date>,
        rate_type: RateType,
        first_rate_definition: Option<RateDefinition>,
        first_rate: Option<f64>,
        second_rate_definition: Option<RateDefinition>,
        second_rate: Option<f64>,
        forecast_curve_id: Option<usize>,
        discount_curve_id: Option<usize>,
        cashflows: Vec<Cashflow>,
    ) -> Self {
        HybridRateInstrument {
            start_date,
            end_date,
            notional,
            payment_frequency,
            structure,
            side,
            currency,
            id,
            issue_date,
            rate_type,
            first_rate_definition,
            first_rate,
            second_rate_definition,
            second_rate,
            forecast_curve_id,
            discount_curve_id,
            cashflows,
        }
    }

    /// Returns the notional amount of the instrument.
    #[must_use]
    pub const fn notional(&self) -> f64 {
        self.notional
    }

    /// Returns the payment frequency of the instrument.
    #[must_use]
    pub const fn payment_frequency(&self) -> Frequency {
        self.payment_frequency
    }

    /// Returns the structure of the instrument.
    #[must_use]
    pub const fn structure(&self) -> Structure {
        self.structure
    }

    /// Returns the side of the instrument.
    #[must_use]
    pub const fn side(&self) -> Option<Side> {
        self.side
    }

    /// Returns the identifier of the instrument.
    #[must_use]
    pub fn id(&self) -> Option<String> {
        self.id.clone()
    }

    /// Returns the forecast curve identifier.
    #[must_use]
    pub const fn forecast_curve_id(&self) -> Option<usize> {
        self.forecast_curve_id
    }

    /// Returns the discount curve identifier.
    #[must_use]
    pub const fn discount_curve_id(&self) -> Option<usize> {
        self.discount_curve_id
    }

    /// Returns the start date of the instrument.
    #[must_use]
    pub const fn start_date(&self) -> Date {
        self.start_date
    }

    /// Returns the end date of the instrument.
    #[must_use]
    pub const fn end_date(&self) -> Date {
        self.end_date
    }

    /// Returns the issue date of the instrument.
    #[must_use]
    pub const fn issue_date(&self) -> Option<Date> {
        self.issue_date
    }

    /// Returns the rate type of the instrument.
    #[must_use]
    pub const fn rate_type(&self) -> RateType {
        self.rate_type
    }

    /// Returns the first rate definition of the instrument.
    #[must_use]
    pub const fn first_rate_definition(&self) -> Option<RateDefinition> {
        self.first_rate_definition
    }

    /// Returns the first rate of the instrument.
    #[must_use]
    pub const fn first_rate(&self) -> Option<f64> {
        self.first_rate
    }

    /// Returns the second rate definition of the instrument.
    #[must_use]
    pub const fn second_rate_definition(&self) -> Option<RateDefinition> {
        self.second_rate_definition
    }

    /// Returns the second rate of the instrument.
    #[must_use]
    pub const fn second_rate(&self) -> Option<f64> {
        self.second_rate
    }

    /// Sets the discount curve identifier.
    #[must_use]
    pub const fn set_discount_curve_id(mut self, discount_curve_id: usize) -> Self {
        self.discount_curve_id = Some(discount_curve_id);
        self
    }

    /// Sets the forecast curve identifier.
    #[must_use]
    pub const fn set_forecast_curve_id(mut self, forecast_curve_id: usize) -> Self {
        self.forecast_curve_id = Some(forecast_curve_id);
        self
    }
}

impl HasCurrency for HybridRateInstrument {
    fn currency(&self) -> Result<Currency> {
        match self.currency {
            Some(currency) => Ok(currency),
            None => Err(AtlasError::NotFoundErr("Currency not found".to_string())),
        }
    }
}

impl HasCashflows for HybridRateInstrument {
    fn cashflows(&self) -> &[Cashflow] {
        &self.cashflows
    }

    fn mut_cashflows(&mut self) -> &mut [Cashflow] {
        &mut self.cashflows
    }
}
