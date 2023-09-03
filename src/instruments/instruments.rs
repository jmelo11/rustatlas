use crate::cashflows::cashflow::*;
use crate::rates::interestrate::*;
use crate::time::date::*;

pub struct CouponBearingInstrument {
    start_date: Date,
    end_date: Date,
    notional: f64,
    cashflows: Vec<Cashflow>,
}

impl CouponBearingInstrument {
    pub fn new(
        start_date_: Date,
        end_date_: Date,
        notional_: f64,
        cashflows_: Vec<Cashflow>,
    ) -> CouponBearingInstrument {
        CouponBearingInstrument {
            start_date: start_date_,
            end_date: end_date_,
            notional: notional_,
            cashflows: cashflows_,
        }
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

    pub fn cashflows(&self) -> &[Cashflow] {
        &self.cashflows
    }
}

trait HasFixedRate {
    fn rate(&self) -> InterestRate;
}

trait HasFloatingRate {
    fn spread(&self) -> f64;
}

trait HasMixedRate: HasFixedRate + HasFloatingRate {}
