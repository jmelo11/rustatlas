use crate::core::enums::Side;
use crate::core::meta::{MetaDiscountFactor, MetaExchangeRateMeta, MetaMarketData};
use crate::core::registry::Registrable;
use crate::currencies::enums::Currency;
use crate::time::date::Date;

pub struct Cashflow {
    amount: f64,
    payment_date: Date,
    discount_curve_id: u16,
    currency: Currency,
    side: Side,
    registry_id: Option<u64>,
}

impl Cashflow {
    pub fn new(
        amount: f64,
        payment_date: Date,
        discount_curve_id: u16,
        currency: Currency,
        side: Side,
    ) -> Cashflow {
        Cashflow {
            amount,
            payment_date,
            discount_curve_id,
            currency,
            side,
            registry_id: None,
        }
    }

    pub fn currency(&self) -> Currency {
        return self.currency;
    }

    pub fn discount_curve_id(&self) -> u16 {
        return self.discount_curve_id;
    }

    pub fn side(&self) -> Side {
        return self.side;
    }

    pub fn amount(&self) -> f64 {
        return self.amount;
    }

    pub fn payment_date(&self) -> Date {
        return self.payment_date;
    }
}

impl Registrable for Cashflow {
    fn registry_id(&self) -> Option<u64> {
        return self.registry_id;
    }

    fn register_id(&mut self, id: u64) {
        self.registry_id = Some(id);
    }

    fn meta_market_data(&self) -> MetaMarketData {
        let discount = MetaDiscountFactor::new(self.discount_curve_id, self.payment_date);
        let currency = MetaExchangeRateMeta::new(self.currency.numeric_code(), self.payment_date);
        return MetaMarketData::new(Some(discount), None, Some(currency));
    }
}
