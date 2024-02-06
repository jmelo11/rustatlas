use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::{
    core::{
        meta::MarketRequest,
        traits::{HasCurrency, HasDiscountCurveId, HasForecastCurveId, Registrable},
    },
    currencies::enums::Currency,
    rates::interestrate::{InterestRate, RateDefinition},
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
#[derive(Clone, Copy)]
pub enum Cashflow {
    Redemption(SimpleCashflow),
    Disbursement(SimpleCashflow),
    FixedRateCoupon(FixedRateCoupon),
    FloatingRateCoupon(FloatingRateCoupon),
}

impl Serialize for Cashflow {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        SerializedCashflow::from(*self).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Cashflow {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Cashflow, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let serialized = SerializedCashflow::deserialize(deserializer)?;
        Cashflow::try_from(serialized).map_err(serde::de::Error::custom)
    }
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
                "date: {}, amount: {}, side: {:?}",
                cashflow.payment_date(),
                amount,
                cashflow.side()
            ),
            Cashflow::Disbursement(cashflow) => write!(
                f,
                "date: {}, amount: {}, side: {:?}",
                cashflow.payment_date(),
                amount,
                cashflow.side()
            ),
            Cashflow::FixedRateCoupon(coupon) => write!(
                f,
                "date: {}, amount: {}, side: {:?}",
                coupon.payment_date(),
                amount,
                coupon.side()
            ),
            Cashflow::FloatingRateCoupon(coupon) => write!(
                f,
                "date: {}, amount: {}, side: {:?}",
                coupon.payment_date(),
                amount,
                coupon.side()
            ),
        }
    }
}

/// # CashflowType
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CashflowType {
    Redemption,
    Disbursement,
    FixedRateCoupon,
    FloatingRateCoupon,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct SerializedCashflow {
    cashflow_type: CashflowType,
    payment_date: Date,
    notional: Option<f64>,
    side: Side,
    rate: Option<f64>,
    rate_definition: Option<RateDefinition>,
    currency: Currency,
    amount: Option<f64>,
    accrual_start_date: Option<Date>,
    accrual_end_date: Option<Date>,
}

impl SerializedCashflow {
    pub fn new(
        cashflow_type: CashflowType,
        payment_date: Date,
        notional: Option<f64>,
        side: Side,
        rate: Option<f64>,
        rate_definition: Option<RateDefinition>,
        currency: Currency,
        amount: Option<f64>,
        accrual_start_date: Option<Date>,
        accrual_end_date: Option<Date>,
    ) -> Self {
        SerializedCashflow {
            cashflow_type,
            payment_date,
            notional,
            side,
            rate,
            rate_definition,
            currency,
            amount,
            accrual_start_date,
            accrual_end_date,
        }
    }

    pub fn cashflow_type(&self) -> CashflowType {
        self.cashflow_type
    }

    pub fn payment_date(&self) -> Date {
        self.payment_date
    }

    pub fn notional(&self) -> Option<f64> {
        self.notional
    }

    pub fn side(&self) -> Side {
        self.side
    }

    pub fn currency(&self) -> Currency {
        self.currency
    }

    pub fn amount(&self) -> Option<f64> {
        self.amount
    }

    pub fn accrual_start_date(&self) -> Option<Date> {
        self.accrual_start_date
    }

    pub fn accrual_end_date(&self) -> Option<Date> {
        self.accrual_end_date
    }

    pub fn rate(&self) -> Option<f64> {
        self.rate
    }

    pub fn rate_definition(&self) -> Option<RateDefinition> {
        self.rate_definition
    }
}

impl From<Cashflow> for SerializedCashflow {
    fn from(cashflow: Cashflow) -> Self {
        match cashflow {
            Cashflow::Redemption(cashflow) => SerializedCashflow {
                cashflow_type: CashflowType::Redemption,
                payment_date: cashflow.payment_date(),
                notional: None,
                side: cashflow.side(),
                amount: Some(cashflow.amount().unwrap()),
                accrual_start_date: None,
                accrual_end_date: None,
                currency: cashflow.currency().unwrap(),
                rate: None,
                rate_definition: None,
            },
            Cashflow::Disbursement(cashflow) => SerializedCashflow {
                cashflow_type: CashflowType::Disbursement,
                payment_date: cashflow.payment_date(),
                notional: None,
                side: cashflow.side(),
                amount: Some(cashflow.amount().unwrap()),
                accrual_start_date: None,
                accrual_end_date: None,
                currency: cashflow.currency().unwrap(),
                rate: None,
                rate_definition: None,
            },
            Cashflow::FixedRateCoupon(coupon) => SerializedCashflow {
                cashflow_type: CashflowType::FixedRateCoupon,
                payment_date: coupon.payment_date(),
                notional: Some(coupon.notional()),
                side: coupon.side(),
                amount: Some(coupon.amount().unwrap()),
                accrual_start_date: Some(coupon.accrual_start_date()),
                accrual_end_date: Some(coupon.accrual_end_date()),
                currency: cashflow.currency().unwrap(),
                rate: Some(coupon.rate().rate()),
                rate_definition: Some(coupon.rate().rate_definition().clone()),
            },
            Cashflow::FloatingRateCoupon(coupon) => SerializedCashflow {
                cashflow_type: CashflowType::FloatingRateCoupon,
                payment_date: coupon.payment_date(),
                notional: Some(coupon.notional()),
                side: coupon.side(),
                amount: None,
                accrual_start_date: Some(coupon.accrual_start_date()),
                accrual_end_date: Some(coupon.accrual_end_date()),
                currency: cashflow.currency().unwrap(),
                rate: Some(coupon.spread()),
                rate_definition: Some(coupon.rate_definition().clone()),
            },
        }
    }
}

impl TryFrom<SerializedCashflow> for Cashflow {
    type Error = AtlasError;

    fn try_from(serialized: SerializedCashflow) -> Result<Self> {
        match serialized.cashflow_type {
            CashflowType::Redemption => Ok(Cashflow::Redemption(
                SimpleCashflow::new(
                    serialized.payment_date,
                    serialized.currency,
                    serialized.side,
                )
                .with_amount(serialized.amount.unwrap()),
            )),
            CashflowType::Disbursement => Ok(Cashflow::Disbursement(
                SimpleCashflow::new(
                    serialized.payment_date,
                    serialized.currency,
                    serialized.side,
                )
                .with_amount(serialized.amount.unwrap()),
            )),
            CashflowType::FixedRateCoupon => Ok(Cashflow::FixedRateCoupon(FixedRateCoupon::new(
                serialized
                    .notional()
                    .ok_or(AtlasError::InvalidValueErr("Notional not set".to_string()))?,
                InterestRate::from_rate_definition(
                    serialized
                        .rate()
                        .ok_or(AtlasError::InvalidValueErr("Rate not set".to_string()))?,
                    serialized.rate_definition().unwrap(),
                ),
                serialized
                    .accrual_start_date()
                    .ok_or(AtlasError::InvalidValueErr(
                        "Accrual start date not set".to_string(),
                    ))?,
                serialized
                    .accrual_end_date
                    .ok_or(AtlasError::InvalidValueErr(
                        "Accrual end date not set".to_string(),
                    ))?,
                serialized.payment_date(),
                serialized.currency(),
                serialized.side(),
            ))),
            CashflowType::FloatingRateCoupon => {
                Ok(Cashflow::FloatingRateCoupon(FloatingRateCoupon::new(
                    serialized
                        .notional()
                        .ok_or(AtlasError::InvalidValueErr("Notional not set".to_string()))?,
                    serialized
                        .rate()
                        .ok_or(AtlasError::InvalidValueErr("Spread not set".to_string()))?,
                    serialized
                        .accrual_start_date()
                        .ok_or(AtlasError::InvalidValueErr(
                            "Accrual start date not set".to_string(),
                        ))?,
                    serialized
                        .accrual_end_date
                        .ok_or(AtlasError::InvalidValueErr(
                            "Accrual end date not set".to_string(),
                        ))?,
                    serialized.payment_date(),
                    serialized
                        .rate_definition()
                        .ok_or(AtlasError::InvalidValueErr(
                            "Rate definition not set".to_string(),
                        ))?,
                    serialized.currency(),
                    serialized.side(),
                )))
            }
        }
    }
}
