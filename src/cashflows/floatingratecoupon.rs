use crate::{
    core::{
        meta::{DiscountFactorRequest, ExchangeRateRequest, ForwardRateRequest, MarketRequest},
        traits::Registrable,
    },
    currencies::enums::Currency,
    rates::interestrate::{InterestRate, RateDefinition},
    time::date::Date,
};

use super::{
    cashflow::SimpleCashflow,
    enums::Side,
    traits::{Expires, InterestAccrual, Payable, RequiresFixingRate},
};

/// # FloatingRateCoupon
/// A floating rate coupon is a cashflow that pays a floating rate of interest on a notional amount.
///
/// ## Parameters
/// * `notional` - The notional amount of the coupon
/// * `spread` - The spread over the floating rate
/// * `accrual_start_date` - The date from which the coupon accrues interest
/// * `accrual_end_date` - The date until which the coupon accrues interest
/// * `payment_date` - The date on which the coupon is paid
/// * `fixing_date` - The date from which the floating rate is observed
/// * `rate_definition` - The definition of the floating rate
/// * `discount_curve_id` - The ID of the discount curve used to calculate the present value of the coupon
/// * `forecast_curve_id` - The ID of the forecast curve used to calculate the present value of the coupon
/// * `currency` - The currency of the coupon
/// * `side` - The side of the coupon (Pay or Receive)
pub struct FloatingRateCoupon {
    notional: f64,
    spread: f64,
    fixing_rate: Option<f64>,
    accrual_start_date: Date,
    accrual_end_date: Date,
    fixing_date: Date,
    rate_definition: RateDefinition,
    in_arrears: bool,
    forecast_curve_id: Option<usize>,
    cashflow: SimpleCashflow,
}

impl FloatingRateCoupon {
    pub fn new(
        notional: f64,
        spread: f64,
        accrual_start_date: Date,
        accrual_end_date: Date,
        payment_date: Date,
        fixing_date: Date,
        rate_definition: RateDefinition,
        currency: Currency,
        side: Side,
        in_arrears: bool,
    ) -> FloatingRateCoupon {
        FloatingRateCoupon {
            notional,
            spread,
            fixing_rate: None,
            accrual_start_date,
            accrual_end_date,
            fixing_date,
            rate_definition,
            in_arrears,
            forecast_curve_id: None,
            cashflow: SimpleCashflow::new(payment_date, currency, side),
        }
    }

    pub fn set_discount_curve_id(&mut self, id: usize) {
        self.cashflow.set_discount_curve_id(id);
    }

    pub fn set_forecast_curve_id(&mut self, id: usize) {
        self.forecast_curve_id = Some(id);
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

impl RequiresFixingRate for FloatingRateCoupon {
    fn set_fixing_rate(&mut self, fixing_rate: f64) {
        self.fixing_rate = Some(fixing_rate);
        self.cashflow
            .set_amount(self.accrued_amount(self.accrual_start_date, self.accrual_end_date));
    }
}

impl Payable for FloatingRateCoupon {
    fn amount(&self) -> f64 {
        return self.cashflow.amount();
    }
    fn side(&self) -> Side {
        return self.cashflow.side();
    }
    fn payment_date(&self) -> Date {
        return self.cashflow.payment_date();
    }
}

impl Registrable for FloatingRateCoupon {
    fn registry_id(&self) -> Option<usize> {
        return self.cashflow.registry_id();
    }

    fn register_id(&mut self, id: usize) {
        self.cashflow.register_id(id);
    }

    fn market_request(&self) -> MarketRequest {
        let id = match self.cashflow.registry_id() {
            Some(id) => id,
            None => panic!("FloatingRateCoupon has not been registered"),
        };
        let tmp = self.cashflow.market_request();
        let forecast_curve_id = match self.forecast_curve_id {
            Some(id) => id,
            None => panic!("FloatingRateCoupon does not have a forecast curve id"),
        };
        let forecast = ForwardRateRequest::new(
            forecast_curve_id,
            self.accrual_start_date,
            self.accrual_end_date,
            self.fixing_date,
            self.rate_definition.compounding(),
            self.rate_definition.frequency(),
        );
        return MarketRequest::new(id, tmp.df(), Some(forecast), tmp.fx());
    }
}

impl Expires for FloatingRateCoupon {
    fn is_expired(&self, date: Date) -> bool {
        return self.cashflow.payment_date() < date;
    }
}
