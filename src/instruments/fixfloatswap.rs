use super::traits::Structure;
use crate::{
    cashflows::cashflow::{Cashflow, Side},
    currencies::enums::Currency,
    rates::interestrate::{InterestRate, RateDefinition},
    time::date::Date,
    visitors::traits::HasCashflows,
};

/// # FixFloatSwap
/// A fixed vs float, single currency, swap.
pub struct FixFloatSwap {
    cashflows: Vec<Cashflow>,
    structure: Structure,
    currency: Currency,
    side: Side,
    rate: InterestRate,
    spread: f64,
    notional: f64,
    start_date: Date,
    end_date: Date,
    issue_date: Option<Date>,
    spread_rate_definition: RateDefinition,
    discount_curve_id: Option<usize>,
    forecast_curve_id: Option<usize>,
    pos: usize,
    id: Option<String>,
}

impl FixFloatSwap {
    pub fn new(
        fixed_leg: Vec<Cashflow>,
        floating_leg: Vec<Cashflow>,
        structure: Structure,
        currency: Currency,
        side: Side,
        rate: InterestRate,
        spread: f64,
        notional: f64,
        start_date: Date,
        end_date: Date,
        issue_date: Option<Date>,
        spread_rate_definition: RateDefinition,
        discount_curve_id: Option<usize>,
        forecast_curve_id: Option<usize>,
        id: Option<String>,
    ) -> Self {
        let mut cashflows = Vec::new();
        cashflows.extend_from_slice(&fixed_leg);
        cashflows.extend_from_slice(&floating_leg);
        let pos = fixed_leg.len();

        FixFloatSwap {
            cashflows,
            structure,
            currency,
            side,
            rate,
            spread,
            notional,
            start_date,
            end_date,
            issue_date,
            spread_rate_definition,
            discount_curve_id,
            forecast_curve_id,
            pos,
            id,
        }
    }

    pub fn fixed_leg(&self) -> &[Cashflow] {
        &self.cashflows[0..self.pos]
    }

    pub fn floating_leg(&self) -> &[Cashflow] {
        &self.cashflows[self.pos..]
    }

    pub fn id(&self) -> Option<String> {
        self.id.clone()
    }

    pub fn structure(&self) -> Structure {
        self.structure
    }

    pub fn currency(&self) -> Currency {
        self.currency
    }

    pub fn side(&self) -> Side {
        self.side
    }

    pub fn rate(&self) -> InterestRate {
        self.rate
    }

    pub fn spread(&self) -> f64 {
        self.spread
    }

    pub fn notional(&self) -> f64 {
        self.notional
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

    pub fn spread_rate_definition(&self) -> RateDefinition {
        self.spread_rate_definition
    }

    pub fn discount_curve_id(&self) -> Option<usize> {
        self.discount_curve_id
    }

    pub fn forecast_curve_id(&self) -> Option<usize> {
        self.forecast_curve_id
    }
}

impl HasCashflows for FixFloatSwap {
    fn cashflows(&self) -> &[Cashflow] {
        &self.cashflows
    }

    fn mut_cashflows(&mut self) -> &mut [Cashflow] {
        &mut self.cashflows
    }
}
