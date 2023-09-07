use std::fmt::{Display, Formatter, Result};

use crate::{
    core::{meta::MarketRequest, traits::Registrable},
    time::date::Date,
};

use super::{
    fixedratecoupon::FixedRateCoupon,
    floatingratecoupon::FloatingRateCoupon,
    simplecashflow::SimpleCashflow,
    traits::{InterestAccrual, Payable, RequiresFixingRate},
};

/// # Side
/// Enum that represents the side of a cashflow.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Side {
    Pay,
    Receive,
}

/// # Cashflow
/// Enum that represents a cashflow.
#[derive(Clone, Copy)]
pub enum Cashflow {
    Redemption(SimpleCashflow),
    Disbursement(SimpleCashflow),
    FixedRateCoupon(FixedRateCoupon),
    FloatingRateCoupon(FloatingRateCoupon),
}

impl Cashflow {
    pub fn set_discount_curve_id(&mut self, id: Option<usize>) {
        match self {
            Cashflow::Redemption(cashflow) => cashflow.set_discount_curve_id(id),
            Cashflow::Disbursement(cashflow) => cashflow.set_discount_curve_id(id),
            Cashflow::FixedRateCoupon(coupon) => coupon.set_discount_curve_id(id),
            Cashflow::FloatingRateCoupon(coupon) => coupon.set_discount_curve_id(id),
        }
    }

    pub fn set_forecast_curve_id(&mut self, id: Option<usize>) {
        match self {
            Cashflow::FloatingRateCoupon(coupon) => coupon.set_forecast_curve_id(id),
            _ => (),
        }
    }
}

impl Payable for Cashflow {
    fn amount(&self) -> f64 {
        match self {
            Cashflow::Redemption(cashflow) => cashflow.amount(),
            Cashflow::Disbursement(cashflow) => cashflow.amount(),
            Cashflow::FixedRateCoupon(coupon) => coupon.amount(),
            Cashflow::FloatingRateCoupon(coupon) => coupon.amount(),
        }
    }

    fn side(&self) -> Side {
        match self {
            Cashflow::Redemption(cashflow) => cashflow.side(),
            Cashflow::Disbursement(cashflow) => cashflow.side(),
            Cashflow::FixedRateCoupon(coupon) => coupon.side(),
            Cashflow::FloatingRateCoupon(coupon) => coupon.side(),
        }
    }

    fn payment_date(&self) -> Date {
        match self {
            Cashflow::Redemption(cashflow) => cashflow.payment_date(),
            Cashflow::Disbursement(cashflow) => cashflow.payment_date(),
            Cashflow::FixedRateCoupon(coupon) => coupon.payment_date(),
            Cashflow::FloatingRateCoupon(coupon) => coupon.payment_date(),
        }
    }
}

impl Registrable for Cashflow {
    fn register_id(&mut self, id: usize) {
        match self {
            Cashflow::Redemption(cashflow) => cashflow.register_id(id),
            Cashflow::Disbursement(cashflow) => cashflow.register_id(id),
            Cashflow::FixedRateCoupon(coupon) => coupon.register_id(id),
            Cashflow::FloatingRateCoupon(coupon) => coupon.register_id(id),
        }
    }

    fn registry_id(&self) -> Option<usize> {
        match self {
            Cashflow::Redemption(cashflow) => cashflow.registry_id(),
            Cashflow::Disbursement(cashflow) => cashflow.registry_id(),
            Cashflow::FixedRateCoupon(coupon) => coupon.registry_id(),
            Cashflow::FloatingRateCoupon(coupon) => coupon.registry_id(),
        }
    }

    fn market_request(&self) -> MarketRequest {
        match self {
            Cashflow::Redemption(cashflow) => cashflow.market_request(),
            Cashflow::Disbursement(cashflow) => cashflow.market_request(),
            Cashflow::FixedRateCoupon(coupon) => coupon.market_request(),
            Cashflow::FloatingRateCoupon(coupon) => coupon.market_request(),
        }
    }
}

impl InterestAccrual for Cashflow {
    fn accrual_end_date(&self) -> Date {
        match self {
            Cashflow::FixedRateCoupon(coupon) => coupon.accrual_end_date(),
            Cashflow::FloatingRateCoupon(coupon) => coupon.accrual_end_date(),
            _ => panic!("Not implemented"),
        }
    }

    fn accrual_start_date(&self) -> Date {
        match self {
            Cashflow::FixedRateCoupon(coupon) => coupon.accrual_start_date(),
            Cashflow::FloatingRateCoupon(coupon) => coupon.accrual_start_date(),
            _ => panic!("Not implemented"),
        }
    }

    fn accrued_amount(&self, start_date: Date, end_date: Date) -> f64 {
        match self {
            Cashflow::FixedRateCoupon(coupon) => coupon.accrued_amount(start_date, end_date),
            Cashflow::FloatingRateCoupon(coupon) => coupon.accrued_amount(start_date, end_date),
            _ => 0.0,
        }
    }
}

impl RequiresFixingRate for Cashflow {
    fn set_fixing_rate(&mut self, fixing_rate: f64) {
        match self {
            Cashflow::FloatingRateCoupon(coupon) => coupon.set_fixing_rate(fixing_rate),
            _ => (),
        }
    }
}

impl Display for Cashflow {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Cashflow::Redemption(cashflow) => write!(
                f,
                "date: {}, amount: {}, side: {:?}",
                cashflow.payment_date(),
                cashflow.amount(),
                cashflow.side()
            ),
            Cashflow::Disbursement(cashflow) => write!(
                f,
                "date: {}, amount: {}, side: {:?}",
                cashflow.payment_date(),
                cashflow.amount(),
                cashflow.side()
            ),
            Cashflow::FixedRateCoupon(coupon) => write!(
                f,
                "date: {}, amount: {}, side: {:?}",
                coupon.payment_date(),
                coupon.amount(),
                coupon.side()
            ),
            Cashflow::FloatingRateCoupon(coupon) => write!(
                f,
                "date: {}, amount: {}, side: {:?}",
                coupon.payment_date(),
                coupon.amount(),
                coupon.side()
            ),
        }
    }
}
