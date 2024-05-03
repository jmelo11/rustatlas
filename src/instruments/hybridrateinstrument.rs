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

    pub fn notional(&self) -> f64 {
        self.notional
    }

    pub fn payment_frequency(&self) -> Frequency {
        self.payment_frequency
    }

    pub fn structure(&self) -> Structure {
        self.structure
    }

    pub fn side(&self) -> Option<Side> {
        self.side
    }

    pub fn id(&self) -> Option<String> {
        self.id.clone()
    }

    pub fn forecast_curve_id(&self) -> Option<usize> {
        self.forecast_curve_id
    }

    pub fn discount_curve_id(&self) -> Option<usize> {
        self.discount_curve_id
    }

    pub fn start_date(&self) -> Date {
        self.start_date
    }

    pub fn end_date(&self) -> Date {
        self.end_date
    }

    pub fn issue_date(&self) -> Option<Date> {
        self.issue_date
    }

    pub fn rate_type(&self) -> RateType {
        self.rate_type
    }

    pub fn first_rate_definition(&self) -> Option<RateDefinition> {
        self.first_rate_definition
    }

    pub fn first_rate(&self) -> Option<f64> {
        self.first_rate
    }

    pub fn second_rate_definition(&self) -> Option<RateDefinition> {
        self.second_rate_definition
    }

    pub fn second_rate(&self) -> Option<f64> {
        self.second_rate
    }

    pub fn set_discount_curve_id(mut self, discount_curve_id: usize) -> Self {
        self.discount_curve_id = Some(discount_curve_id);
        self
    }

    pub fn set_forecast_curve_id(mut self, forecast_curve_id: usize) -> Self {
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
