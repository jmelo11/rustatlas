use super::instrument::RateType;
use super::traits::Structure;
use crate::cashflows::cashflow::{Cashflow, Side};
use crate::cashflows::traits::InterestAccrual;
use crate::core::traits::HasCurrency;
use crate::currencies::enums::Currency;
use crate::rates::interestrate::RateDefinition;
use crate::time::date::Date;
use crate::time::enums::Frequency;
use crate::utils::errors::Result;
use crate::visitors::traits::HasCashflows;

pub struct HybridRateInstrument {
    start_date: Date,
    end_date: Date,
    notional: f64,
    payment_frequency: Frequency,
    structure: Structure,
    side: Side,
    currency: Currency,
    id: Option<usize>,
    issue_date: Option<Date>,

    rate_type: RateType,
    first_rate_definition: RateDefinition,
    first_rate: f64,

    second_rate_definition: RateDefinition,
    second_rate: f64,

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
        cashflows: Vec<Cashflow>,
        structure: Structure,
        side: Side,
        currency: Currency,
        id: Option<usize>,
        issue_date: Option<Date>,

        rate_type: RateType,
        first_rate_definition: RateDefinition,
        first_rate: f64,

        second_rate_definition: RateDefinition,
        second_rate: f64,

        forecast_curve_id: Option<usize>,
        discount_curve_id: Option<usize>,
    ) -> Self {
        Self {
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

    pub fn id(&self) -> Option<usize> {
        self.id
    }

    pub fn start_date(&self) -> Date {
        self.start_date
    }

    pub fn end_date(&self) -> Date {
        self.end_date
    }

    pub fn notional(&self) -> f64 {
        self.notional
    }

    pub fn structure(&self) -> Structure {
        self.structure
    }

    pub fn payment_frequency(&self) -> Frequency {
        self.payment_frequency
    }

    pub fn side(&self) -> Side {
        self.side
    }

    pub fn issue_date(&self) -> Option<Date> {
        self.issue_date
    }

    pub fn rate_type(&self) -> RateType {
        self.rate_type
    }

    pub fn first_rate_definition(&self) -> RateDefinition {
        self.first_rate_definition
    }

    pub fn first_rate(&self) -> f64 {
        self.first_rate
    }

    pub fn second_rate_definition(&self) -> RateDefinition {
        self.second_rate_definition
    }

    pub fn second_rate(&self) -> f64 {
        self.second_rate
    }

    pub fn forecast_curve_id(&self) -> Option<usize> {
        self.forecast_curve_id
    }

    pub fn discount_curve_id(&self) -> Option<usize> {
        self.discount_curve_id
    }
}

impl HasCurrency for HybridRateInstrument {
    fn currency(&self) -> Result<Currency> {
        Ok(self.currency)
    }
}

impl InterestAccrual for HybridRateInstrument {
    fn accrued_amount(&self, start_date: Date, end_date: Date) -> Result<f64> {
        let total_accrued_amount = self.cashflows.iter().fold(0.0, |acc, cf| {
            acc + cf.accrued_amount(start_date, end_date).unwrap_or(0.0)
        });
        Ok(total_accrued_amount)
    }

    fn accrual_start_date(&self) -> Date {
        self.start_date
    }
    fn accrual_end_date(&self) -> Date {
        self.end_date
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
