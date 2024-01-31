use super::instrument::RateType;
use super::traits::Structure;
use crate::cashflows::cashflow::{Cashflow, Side};
use crate::cashflows::traits::InterestAccrual;
use crate::core::traits::HasCurrency;
use crate::currencies::enums::Currency;
use crate::rates::interestrate::{InterestRate, RateDefinition};
use crate::time::date::Date;
use crate::time::enums::Frequency;
use crate::utils::errors::Result;
use crate::visitors::traits::HasCashflows;


pub struct MixedRateInstrument {
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

    first_part_rate_type: RateType,
    first_part_rate_definition: RateDefinition,
    first_part_rate: f64, 

    second_part_rate_type: RateType,
    second_part_rate_definition: RateDefinition,
    second_part_rate: f64,

    forecast_curve_id: Option<usize>,
    discount_curve_id: Option<usize>,
}

impl MixedRateInstrument {
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

        first_part_rate_type: RateType,
        first_part_rate_definition: RateDefinition,
        first_part_rate: f64, 

        second_part_rate_type: RateType,
        second_part_rate_definition: RateDefinition,
        second_part_rate: f64,
        
        forecast_curve_id: Option<usize>,
        discount_curve_id: Option<usize>,
    ) -> Self {
        Self {
            start_date,
            end_date,
            notional,
            payment_frequency,
            cashflows,
            structure,
            side,
            currency,
            id,
            issue_date,
            first_part_rate_type,
            first_part_rate_definition,
            first_part_rate, 
            second_part_rate_type,
            second_part_rate_definition,
            second_part_rate,
            forecast_curve_id,
            discount_curve_id,
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

}


impl HasCurrency for MixedRateInstrument {
    fn currency(&self) -> Result<Currency> {
        Ok(self.currency)
    }
}

impl InterestAccrual for MixedRateInstrument {
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

impl HasCashflows for MixedRateInstrument {
    fn cashflows(&self) -> &[Cashflow] {
        &self.cashflows
    }

    fn mut_cashflows(&mut self) -> &mut [Cashflow] {
        &mut self.cashflows
    }
}



