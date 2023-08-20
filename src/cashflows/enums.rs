use std::fmt::{Display, Formatter, Result};

use crate::{
    core::{enums::Side, meta::MetaMarketData, registry::Registrable},
    time::date::Date,
};

use super::{
    cashflow::SimpleCashflow,
    fixedratecoupon::FixedRateCoupon,
    floatingratecoupon::{FloatingRateCoupon, RequiresFixingRate},
    traits::{InterestAccrual, Payable},
};
pub enum Cashflow {
    Redemption(SimpleCashflow),
    Disbursement(SimpleCashflow),
    FixedRateCoupon(FixedRateCoupon),
    FloatingRateCoupon(FloatingRateCoupon),
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

    fn meta_market_data(&self) -> MetaMarketData {
        match self {
            Cashflow::Redemption(cashflow) => cashflow.meta_market_data(),
            Cashflow::Disbursement(cashflow) => cashflow.meta_market_data(),
            Cashflow::FixedRateCoupon(coupon) => coupon.meta_market_data(),
            Cashflow::FloatingRateCoupon(coupon) => coupon.meta_market_data(),
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
