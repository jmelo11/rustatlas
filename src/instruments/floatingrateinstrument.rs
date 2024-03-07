use crate::{
    cashflows::{
        cashflow::{Cashflow, Side},
        traits::InterestAccrual,
    },
    core::traits::HasCurrency,
    currencies::enums::Currency,
    rates::interestrate::RateDefinition,
    time::{date::Date, enums::Frequency},
    visitors::traits::HasCashflows,
};

use super::traits::Structure;
use crate::utils::errors::Result;

/// # FloatingRateInstrument
/// A floating rate instrument.
///
/// ## Parameters
/// * `start_date` - The start date.
/// * `end_date` - The end date.
/// * `notional` - The notional.
/// * `spread` - The spread.
/// * `side` - The side.
/// * `cashflows` - The cashflows.
/// * `payment_frequency` - The payment frequency.
/// * `rate_definition` - The rate definition.
/// * `structure` - The structure.
#[derive(Clone)]
pub struct FloatingRateInstrument {
    start_date: Date,
    end_date: Date,
    notional: f64,
    spread: f64,
    side: Side,
    cashflows: Vec<Cashflow>,
    payment_frequency: Frequency,
    rate_definition: RateDefinition,
    structure: Structure,
    currency: Currency,
    discount_curve_id: Option<usize>,
    forecast_curve_id: Option<usize>,
    id: Option<usize>,
    issue_date: Option<Date>,
}

impl FloatingRateInstrument {
    pub fn new(
        start_date: Date,
        end_date: Date,
        notional: f64,
        spread: f64,
        side: Side,
        cashflows: Vec<Cashflow>,
        payment_frequency: Frequency,
        rate_definition: RateDefinition,
        structure: Structure,
        currency: Currency,
        discount_curve_id: Option<usize>,
        forecast_curve_id: Option<usize>,
        id: Option<usize>,
        issue_date: Option<Date>,
    ) -> Self {
        FloatingRateInstrument {
            start_date,
            end_date,
            notional,
            spread,
            side,
            cashflows,
            payment_frequency,
            rate_definition,
            structure,
            currency,
            discount_curve_id,
            forecast_curve_id,
            id,
            issue_date,
        }
    }

    pub fn issue_date(&self) -> Option<Date> {
        self.issue_date
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

    pub fn spread(&self) -> f64 {
        self.spread
    }

    pub fn side(&self) -> Side {
        self.side
    }

    pub fn payment_frequency(&self) -> Frequency {
        self.payment_frequency
    }

    pub fn rate_definition(&self) -> RateDefinition {
        self.rate_definition
    }

    pub fn structure(&self) -> Structure {
        self.structure
    }

    pub fn discount_curve_id(&self) -> Option<usize> {
        self.discount_curve_id
    }

    pub fn forecast_curve_id(&self) -> Option<usize> {
        self.forecast_curve_id
    }

    pub fn set_discount_curve_id(mut self, discount_curve_id: usize) -> Self {
        self.discount_curve_id = Some(discount_curve_id);
        self.mut_cashflows()
            .iter_mut()
            .for_each(|cf| cf.set_discount_curve_id(discount_curve_id));
        self
    }

    pub fn set_forecast_curve_id(mut self, forecast_curve_id: usize) -> Self {
        self.forecast_curve_id = Some(forecast_curve_id);
        self.mut_cashflows()
            .iter_mut()
            .for_each(|cf| cf.set_forecast_curve_id(forecast_curve_id));
        self
    }
}

impl HasCurrency for FloatingRateInstrument {
    fn currency(&self) -> Result<Currency> {
        Ok(self.currency)
    }
}

impl InterestAccrual for FloatingRateInstrument {
    fn accrual_start_date(&self) -> Date {
        self.start_date
    }
    fn accrual_end_date(&self) -> Date {
        self.end_date
    }
    fn accrued_amount(&self, start_date: Date, end_date: Date) -> Result<f64> {
        let total_accrued_amount = self.cashflows.iter().fold(0.0, |acc, cf| {
            acc + cf.accrued_amount(start_date, end_date).unwrap_or(0.0)
        });
        Ok(total_accrued_amount)
    }
}

impl HasCashflows for FloatingRateInstrument {
    fn cashflows(&self) -> &[Cashflow] {
        &self.cashflows
    }

    fn mut_cashflows(&mut self) -> &mut [Cashflow] {
        &mut self.cashflows
    }
}
