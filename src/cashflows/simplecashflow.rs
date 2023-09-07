use super::cashflow::Side;
use super::traits::{Expires, Payable};
use crate::core::meta::*;
use crate::core::traits::Registrable;
use crate::currencies::enums::Currency;
use crate::time::date::Date;

/// # SimpleCashflow
/// A simple cashflow that is payable at a given date.
///
/// ## Example
/// ```
/// use rustatlas::prelude::*;
/// let payment_date = Date::new(2020, 1, 1);
/// let cashflow = SimpleCashflow::new(payment_date, Currency::USD, Side::Receive).with_amount(100.0);
/// assert_eq!(cashflow.amount(), 100.0);
/// assert_eq!(cashflow.side(), Side::Receive);
/// assert_eq!(cashflow.payment_date(), payment_date);
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SimpleCashflow {
    payment_date: Date,
    currency: Currency,
    side: Side,
    amount: Option<f64>,
    discount_curve_id: Option<usize>,
    registry_id: Option<usize>,
}

impl SimpleCashflow {
    pub fn new(payment_date: Date, currency: Currency, side: Side) -> SimpleCashflow {
        SimpleCashflow {
            payment_date,
            currency,
            side,
            amount: None,
            discount_curve_id: None,
            registry_id: None,
        }
    }

    pub fn with_amount(mut self, amount: f64) -> SimpleCashflow {
        self.amount = Some(amount);
        self
    }

    pub fn with_discount_curve_id(mut self, discount_curve_id: Option<usize>) -> SimpleCashflow {
        self.discount_curve_id = discount_curve_id;
        self
    }

    pub fn with_registry_id(mut self, registry_id: usize) -> SimpleCashflow {
        self.registry_id = Some(registry_id);
        self
    }

    pub fn set_discount_curve_id(&mut self, id: Option<usize>) {
        self.discount_curve_id = id;
    }

    pub fn set_amount(&mut self, amount: f64) {
        self.amount = Some(amount);
    }
}

impl Registrable for SimpleCashflow {
    fn registry_id(&self) -> Option<usize> {
        return self.registry_id;
    }

    fn register_id(&mut self, id: usize) {
        self.registry_id = Some(id);
    }

    fn market_request(&self) -> MarketRequest {
        let id = self
            .registry_id
            .expect("SimpleCashflow does not have a registry id");
        let discount_curve_id = self
            .discount_curve_id
            .expect("SimpleCashflow does not have a discount curve id");
        let discount = DiscountFactorRequest::new(discount_curve_id, self.payment_date);
        let currency = ExchangeRateRequest::new(self.currency, None, None);
        return MarketRequest::new(id, Some(discount), None, Some(currency));
    }
}

impl Payable for SimpleCashflow {
    fn amount(&self) -> f64 {
        return self
            .amount
            .expect("SimpleCashflow does not have an defined amount");
    }
    fn side(&self) -> Side {
        return self.side;
    }

    fn payment_date(&self) -> Date {
        return self.payment_date;
    }
}

impl Expires for SimpleCashflow {
    fn is_expired(&self, date: Date) -> bool {
        return self.payment_date < date;
    }
}
