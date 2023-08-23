use super::enums::Side;
use super::traits::{Expires, Payable};
use crate::core::meta::{MetaDiscountFactor, MetaExchangeRate, MetaMarketDataNode};
use crate::core::traits::Registrable;
use crate::currencies::enums::Currency;
use crate::time::date::Date;

/// # SimpleCashflow
/// A simple cashflow that is payable at a given date.
///
/// ## Example
/// ```
/// use rustatlas::cashflows::cashflow::SimpleCashflow;
/// use rustatlas::cashflows::traits::Payable;
/// use super::enums::Side;
/// use rustatlas::currencies::enums::Currency;
/// use rustatlas::time::date::Date;
///
/// let cashflow = SimpleCashflow::new(100.0, Date::from_ymd(2020, 1, 1), 0, Currency::USD, Side::Receive);
/// assert_eq!(cashflow.amount(), 100.0);
/// assert_eq!(cashflow.side(), Side::Receive);
/// assert_eq!(cashflow.payment_date(), Date::from_ymd(2020, 1, 1));
/// ```
pub struct SimpleCashflow {
    amount: f64,
    payment_date: Date,
    discount_curve_id: usize,
    currency: Currency,
    side: Side,
    registry_id: Option<usize>,
}

impl SimpleCashflow {
    pub fn new(
        amount: f64,
        payment_date: Date,
        discount_curve_id: usize,
        currency: Currency,
        side: Side,
    ) -> SimpleCashflow {
        SimpleCashflow {
            amount,
            payment_date,
            discount_curve_id,
            currency,
            side,
            registry_id: None,
        }
    }
}

impl Registrable for SimpleCashflow {
    fn registry_id(&self) -> Option<usize> {
        return self.registry_id;
    }

    fn register_id(&mut self, id: usize) {
        self.registry_id = Some(id);
    }

    fn meta_market_data(&self) -> MetaMarketDataNode {
        let id = match self.registry_id {
            Some(id) => id,
            None => panic!("SimpleCashflow has not been registered"),
        };
        let discount = MetaDiscountFactor::new(self.discount_curve_id, self.payment_date);
        let currency = MetaExchangeRate::new(self.currency, self.payment_date);
        return MetaMarketDataNode::new(id, Some(discount), None, Some(currency));
    }
}

impl Payable for SimpleCashflow {
    fn amount(&self) -> f64 {
        return self.amount;
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
