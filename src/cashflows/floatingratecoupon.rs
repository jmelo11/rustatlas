use crate::{
    core::{
        enums::Side,
        meta::{MetaDiscountFactor, MetaExchangeRate, MetaForwardRate, MetaMarketData},
        registry::Registrable,
    },
    currencies::enums::Currency,
    rates::{interestrate::{InterestRate, RateDefinition}, traits::YieldProvider},
    time::date::Date,
};

use super::traits::{InterestAccrual, Payable};

pub struct FloatingRateCoupon {
    notional: f64,
    spread: f64,
    fixing_rate: Option<f64>,
    amount: Option<f64>,

    accrual_start_date: Date,
    accrual_end_date: Date,
    payment_date: Date,
    fixing_start_date: Date,
    fixing_end_date: Date,

    rate_definition: RateDefinition,
    discount_curve_id: usize,
    forecast_curve_id: usize,
    currency: Currency,
    side: Side,
    registry_id: Option<usize>,
}

impl FloatingRateCoupon {
    pub fn new(
        notional: f64,
        spread: f64,
        accrual_start_date: Date,
        accrual_end_date: Date,
        payment_date: Date,
        fixing_start_date: Date,
        fixing_end_date: Date,
        rate_definition: RateDefinition,
        discount_curve_id: usize,
        forecast_curve_id: usize,
        currency: Currency,
        side: Side,
    ) -> FloatingRateCoupon {
        FloatingRateCoupon {
            notional,
            spread,
            fixing_rate: None,
            amount: None,
            accrual_start_date,
            accrual_end_date,
            payment_date,
            fixing_start_date,
            fixing_end_date,
            rate_definition,
            discount_curve_id,
            forecast_curve_id,
            currency,
            side,
            registry_id: None,
        }
    }
}

impl InterestAccrual for FloatingRateCoupon {
    fn accrual_start_date(&self) -> Date {
        return self.accrual_start_date;
    }
    fn accrual_end_date(&self) -> Date {
        return self.accrual_end_date;
    }
    fn accrued_amount(&self, start_date: Date, end_date: Date) -> f64 {
        let fixing = match self.fixing_rate {
            Some(fixing) => fixing,
            None => panic!("No fixing rate has been set"),
        };
        let rate = InterestRate::from_rate_definition(fixing + self.spread, self.rate_definition);
        let (d1, d2) = self.relevant_accrual_dates(start_date, end_date);
        return self.notional * (rate.compound_factor(d1, d2) - 1.0);
    }
}

pub trait RequiresFixingRate: InterestAccrual {
    fn set_fixing_rate(&mut self, fixing_rate: f64);
}

impl RequiresFixingRate for FloatingRateCoupon {
    fn set_fixing_rate(&mut self, fixing_rate: f64) {
        self.fixing_rate = Some(fixing_rate);
        self.amount = Some(self.accrued_amount(self.accrual_start_date, self.accrual_end_date));
    }
}

impl Payable for FloatingRateCoupon {
    fn amount(&self) -> f64 {
        return match self.amount {
            Some(amount) => amount,
            None => panic!("No amount has been set"),
        };
    }
    fn side(&self) -> Side {
        return self.side;
    }
    fn payment_date(&self) -> Date {
        return self.payment_date;
    }
}

impl Registrable for FloatingRateCoupon {
    fn registry_id(&self) -> Option<usize> {
        return self.registry_id;
    }

    fn register_id(&mut self, id: usize) {
        self.registry_id = Some(id);
    }

    fn meta_market_data(&self) -> MetaMarketData {
        let id = match self.registry_id {
            Some(id) => id,
            None => panic!("FloatingRateCoupon has not been registered"),
        };
        let discount = MetaDiscountFactor::new(self.discount_curve_id, self.payment_date);
        let forecast = MetaForwardRate::new(
            self.forecast_curve_id,
            self.fixing_start_date,
            self.fixing_end_date,
        );
        let currency = MetaExchangeRate::new(self.currency.numeric_code(), self.payment_date);
        return MetaMarketData::new(id, Some(discount), Some(forecast), Some(currency));
    }
}
