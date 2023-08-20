use super::traits::Payable;
use crate::core::enums::Side;
use crate::core::meta::{MetaDiscountFactor, MetaExchangeRate, MetaMarketData};
use crate::core::registry::Registrable;
use crate::currencies::enums::Currency;
use crate::time::date::Date;

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

    fn meta_market_data(&self) -> MetaMarketData {
        let id = match self.registry_id {
            Some(id) => id,
            None => panic!("SimpleCashflow has not been registered"),
        };
        let discount = MetaDiscountFactor::new(self.discount_curve_id, self.payment_date);
        let currency = MetaExchangeRate::new(self.currency.numeric_code(), self.payment_date);
        return MetaMarketData::new(id, Some(discount), None, Some(currency));
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
