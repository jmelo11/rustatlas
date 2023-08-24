use super::enums::Side;
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
/// let cashflow = SimpleCashflow::new(100.0, Date::from_ymd(2020, 1, 1), 0, Currency::USD, Side::Receive);
/// assert_eq!(cashflow.amount(), 100.0);
/// assert_eq!(cashflow.side(), Side::Receive);
/// assert_eq!(cashflow.payment_date(), Date::from_ymd(2020, 1, 1));
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

    pub fn new_with_amount(
        amount: f64,
        payment_date: Date,
        currency: Currency,
        side: Side,
    ) -> SimpleCashflow {
        SimpleCashflow {
            payment_date,
            currency,
            side,
            amount: Some(amount),
            discount_curve_id: None,
            registry_id: None,
        }
    }

    pub fn set_amount(&mut self, amount: f64) {
        self.amount = Some(amount);
    }

    pub fn set_discount_curve_id(&mut self, id: usize) {
        self.discount_curve_id = Some(id);
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
        let id = match self.registry_id {
            Some(id) => id,
            None => panic!("SimpleCashflow has not been registered"),
        };
        let discount_curve_id = match self.discount_curve_id {
            Some(id) => id,
            None => panic!("SimpleCashflow does not have a discount curve id"),
        };
        let discount = DiscountFactorRequest::new(discount_curve_id, self.payment_date);
        let currency = ExchangeRateRequest::new(self.currency, None, None);
        return MarketRequest::new(id, Some(discount), None, Some(currency));
    }
}

impl Payable for SimpleCashflow {
    fn amount(&self) -> f64 {
        return match self.amount {
            Some(amount) => amount,
            None => panic!("SimpleCashflow does not have an amount"),
        };
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


