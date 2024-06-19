use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::{
    core::{
        meta::MarketRequest,
        traits::{HasCurrency, HasDiscountCurveId, HasForecastCurveId, Registrable},
    },
    currencies::enums::Currency,
    time::date::Date,
    utils::errors::{AtlasError, Result},
};

use super::{
    fixedratecoupon::FixedRateCoupon,
    floatingratecoupon::FloatingRateCoupon,
    simplecashflow::SimpleCashflow,
    traits::{InterestAccrual, Payable, RequiresFixingRate},
};

/// # Side
/// Enum that represents the side of a cashflow.
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Side {
    Pay,
    Receive,
}

impl Side {
    pub fn sign(&self) -> f64 {
        match self {
            Side::Pay => -1.0,
            Side::Receive => 1.0,
        }
    }

    pub fn inverse(&self) -> Side {
        match self {
            Side::Pay => Side::Receive,
            Side::Receive => Side::Pay,
        }
    }
}

impl TryFrom<String> for Side {
    type Error = AtlasError;

    fn try_from(s: String) -> Result<Self> {
        match s.as_str() {
            "Pay" => Ok(Side::Pay),
            "Receive" => Ok(Side::Receive),
            _ => Err(AtlasError::InvalidValueErr(format!("Invalid side: {}", s))),
        }
    }
}

impl From<Side> for String {
    fn from(side: Side) -> Self {
        match side {
            Side::Pay => "Pay".to_string(),
            Side::Receive => "Receive".to_string(),
        }
    }
}

/// # Cashflow
/// Enum that represents a cashflow.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum Cashflow {
    Redemption(SimpleCashflow),
    Disbursement(SimpleCashflow),
    FixedRateCoupon(FixedRateCoupon),
    FloatingRateCoupon(FloatingRateCoupon),
}

impl Cashflow {
    pub fn set_discount_curve_id(&mut self, id: usize) {
        match self {
            Cashflow::Redemption(cashflow) => cashflow.set_discount_curve_id(id),
            Cashflow::Disbursement(cashflow) => cashflow.set_discount_curve_id(id),
            Cashflow::FixedRateCoupon(coupon) => coupon.set_discount_curve_id(id),
            Cashflow::FloatingRateCoupon(coupon) => coupon.set_discount_curve_id(id),
        }
    }

    pub fn set_forecast_curve_id(&mut self, id: usize) {
        match self {
            Cashflow::FloatingRateCoupon(coupon) => coupon.set_forecast_curve_id(id),
            _ => (),
        }
    }
}

impl Payable for Cashflow {
    fn amount(&self) -> Result<f64> {
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

impl HasCurrency for Cashflow {
    fn currency(&self) -> Result<Currency> {
        match self {
            Cashflow::Redemption(cashflow) => cashflow.currency(),
            Cashflow::Disbursement(cashflow) => cashflow.currency(),
            Cashflow::FixedRateCoupon(coupon) => coupon.currency(),
            Cashflow::FloatingRateCoupon(coupon) => coupon.currency(),
        }
    }
}

impl HasDiscountCurveId for Cashflow {
    fn discount_curve_id(&self) -> Result<usize> {
        match self {
            Cashflow::Redemption(cashflow) => cashflow.discount_curve_id(),
            Cashflow::Disbursement(cashflow) => cashflow.discount_curve_id(),
            Cashflow::FixedRateCoupon(coupon) => coupon.discount_curve_id(),
            Cashflow::FloatingRateCoupon(coupon) => coupon.discount_curve_id(),
        }
    }
}

impl HasForecastCurveId for Cashflow {
    fn forecast_curve_id(&self) -> Result<usize> {
        match self {
            Cashflow::Redemption(cashflow) => cashflow.forecast_curve_id(),
            Cashflow::Disbursement(cashflow) => cashflow.forecast_curve_id(),
            Cashflow::FixedRateCoupon(coupon) => coupon.forecast_curve_id(),
            Cashflow::FloatingRateCoupon(coupon) => coupon.forecast_curve_id(),
        }
    }
}

impl Registrable for Cashflow {
    fn set_id(&mut self, id: usize) {
        match self {
            Cashflow::Redemption(cashflow) => cashflow.set_id(id),
            Cashflow::Disbursement(cashflow) => cashflow.set_id(id),
            Cashflow::FixedRateCoupon(coupon) => coupon.set_id(id),
            Cashflow::FloatingRateCoupon(coupon) => coupon.set_id(id),
        }
    }

    fn id(&self) -> Result<usize> {
        match self {
            Cashflow::Redemption(cashflow) => cashflow.id(),
            Cashflow::Disbursement(cashflow) => cashflow.id(),
            Cashflow::FixedRateCoupon(coupon) => coupon.id(),
            Cashflow::FloatingRateCoupon(coupon) => coupon.id(),
        }
    }

    fn market_request(&self) -> Result<MarketRequest> {
        match self {
            Cashflow::Redemption(cashflow) => cashflow.market_request(),
            Cashflow::Disbursement(cashflow) => cashflow.market_request(),
            Cashflow::FixedRateCoupon(coupon) => coupon.market_request(),
            Cashflow::FloatingRateCoupon(coupon) => coupon.market_request(),
        }
    }
}

impl InterestAccrual for Cashflow {
    fn accrual_end_date(&self) -> Result<Date> {
        match self {
            Cashflow::FixedRateCoupon(coupon) => coupon.accrual_end_date(),
            Cashflow::FloatingRateCoupon(coupon) => coupon.accrual_end_date(),
            Cashflow::Disbursement(_) | Cashflow::Redemption(_) => {
                Err(AtlasError::InvalidValueErr(
                    "Disbursement and Redemption cashflows do not have an accrual end date"
                        .to_string(),
                ))
            }
        }
    }

    fn accrual_start_date(&self) -> Result<Date> {
        match self {
            Cashflow::FixedRateCoupon(coupon) => coupon.accrual_start_date(),
            Cashflow::FloatingRateCoupon(coupon) => coupon.accrual_start_date(),
            Cashflow::Disbursement(_) | Cashflow::Redemption(_) => {
                Err(AtlasError::InvalidValueErr(
                    "Disbursement and Redemption cashflows do not have an accrual start date"
                        .to_string(),
                ))
            }
        }
    }

    fn accrued_amount(&self, start_date: Date, end_date: Date) -> Result<f64> {
        match self {
            Cashflow::FixedRateCoupon(coupon) => coupon.accrued_amount(start_date, end_date),
            Cashflow::FloatingRateCoupon(coupon) => coupon.accrued_amount(start_date, end_date),
            _ => Ok(0.0),
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
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let amount = self.amount().unwrap_or(0.0);
        match self {
            Cashflow::Redemption(cashflow) => write!(
                f,
                "date: {}, amount: {}, side: {:?}, type: redemption",
                cashflow.payment_date(),
                amount,
                cashflow.side()
            ),
            Cashflow::Disbursement(cashflow) => write!(
                f,
                "date: {}, amount: {}, side: {:?}, type: disbursement",
                cashflow.payment_date(),
                amount,
                cashflow.side()
            ),
            Cashflow::FixedRateCoupon(coupon) => write!(
                f,
                "date: {}, amount: {}, side: {:?}, type: fixed rate coupon",
                coupon.payment_date(),
                amount,
                coupon.side()
            ),
            Cashflow::FloatingRateCoupon(coupon) => write!(
                f,
                "date: {}, amount: {}, side: {:?}, type: floating rate coupon",
                coupon.payment_date(),
                amount,
                coupon.side()
            ),
        }
    }
}

/// # CashflowType
/// Enum that represents the type of a cashflow.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CashflowType {
    Redemption,
    Disbursement,
    FixedRateCoupon,
    FloatingRateCoupon,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn serialization_test() {
        let cashflow = Cashflow::Redemption(SimpleCashflow::new(
            Date::new(2024, 1, 1),
            Currency::USD,
            Side::Receive,
        ));
        let serialized = serde_json::to_string(&cashflow).unwrap();
        println!("{}", serialized);

        let deserialized: Cashflow = serde_json::from_str(&serialized).unwrap();
        assert_eq!(cashflow, deserialized);
    }
}
