use serde::{Deserialize, Serialize};

use crate::{
    core::{
        meta::{ForwardRateRequest, MarketRequest, Numeric},
        traits::{HasCurrency, HasDiscountCurveId, HasForecastCurveId, Registrable},
    },
    currencies::enums::Currency,
    rates::interestrate::{InterestRate, RateDefinition},
    time::date::Date,
    utils::errors::{AtlasError, Result},
};

use super::{
    cashflow::Side,
    simplecashflow::SimpleCashflow,
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
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct FloatingRateCoupon {
    notional: Numeric,
    spread: Numeric,
    accrual_start_date: Date,
    accrual_end_date: Date,
    fixing_date: Option<Date>,
    rate_definition: RateDefinition,
    cashflow: SimpleCashflow,
    fixing_rate: Option<Numeric>,
    forecast_curve_id: Option<usize>,
}

impl FloatingRateCoupon {
    pub fn new(
        notional: Numeric,
        spread: Numeric,
        accrual_start_date: Date,
        accrual_end_date: Date,
        payment_date: Date,
        fixing_date: Option<Date>,
        rate_definition: RateDefinition,
        currency: Currency,
        side: Side,
    ) -> FloatingRateCoupon {
        FloatingRateCoupon {
            notional,
            spread,
            fixing_rate: None,
            accrual_start_date,
            accrual_end_date,
            fixing_date,
            rate_definition,
            forecast_curve_id: None,
            cashflow: SimpleCashflow::new(payment_date, currency, side),
        }
    }

    pub fn with_discount_curve_id(self, id: usize) -> FloatingRateCoupon {
        self.cashflow.with_discount_curve_id(id);
        self
    }

    pub fn with_forecast_curve_id(mut self, id: usize) -> FloatingRateCoupon {
        self.forecast_curve_id = Some(id);
        self
    }

    pub fn set_discount_curve_id(&mut self, id: usize) {
        self.cashflow.set_discount_curve_id(id);
    }

    pub fn set_forecast_curve_id(&mut self, id: usize) {
        self.forecast_curve_id = Some(id);
    }

    pub fn set_spread(&mut self, spread: Numeric) {
        self.spread = spread;
        // if fixing rate is set, update the cashflow
        match self.fixing_rate {
            Some(fixing_rate) => {
                self.set_fixing_rate(fixing_rate);
            },
            None => {}
        }
    }

    pub fn set_notional(&mut self, notional: Numeric) {
        self.notional = notional;
    }

    pub fn notional(&self) -> Numeric {
        self.notional
    }

    pub fn spread(&self) -> Numeric {
        self.spread
    }

    pub fn rate_definition(&self) -> RateDefinition {
        self.rate_definition
    }

    pub fn fixing_date(&self) -> Date {
        match self.fixing_date {
            Some(date) => date,
            None => self.accrual_start_date,
        }
    }

    pub fn fixing_rate(&self) -> Option<Numeric> {
        self.fixing_rate
    }
}

impl InterestAccrual for FloatingRateCoupon {
    fn accrual_start_date(&self) -> Result<Date> {
        return Ok(self.accrual_start_date);
    }
    fn accrual_end_date(&self) -> Result<Date> {
        return Ok(self.accrual_end_date);
    }
    fn accrued_amount(&self, start_date: Date, end_date: Date) -> Result<Numeric> {
        let fixing = self
            .fixing_rate
            .ok_or(AtlasError::ValueNotSetErr("Fixing rate".to_string()))?;
        let rate = InterestRate::from_rate_definition(fixing + self.spread, self.rate_definition);

        let (d1, d2) = self.relevant_accrual_dates(self.accrual_start_date, end_date)?;
        let acc_1 = self.notional * (rate.compound_factor(d1, d2) - 1.0);

        let (d1, d2) = self.relevant_accrual_dates(self.accrual_start_date, start_date)?;
        let acc_2 = self.notional * (rate.compound_factor(d1, d2) - 1.0);

        return Ok(acc_1 - acc_2);
    }
}

impl RequiresFixingRate for FloatingRateCoupon {
    fn set_fixing_rate(&mut self, fixing_rate: Numeric) {
        self.fixing_rate = Some(fixing_rate);
        let accrual = self
            .accrued_amount(self.accrual_start_date, self.accrual_end_date)
            .unwrap();
        self.cashflow = self.cashflow.with_amount(accrual);
    }
}

impl Payable for FloatingRateCoupon {
    fn amount(&self) -> Result<Numeric> {
        return self.cashflow.amount();
    }
    fn side(&self) -> Side {
        return self.cashflow.side();
    }
    fn payment_date(&self) -> Date {
        return self.cashflow.payment_date();
    }
}

impl HasCurrency for FloatingRateCoupon {
    fn currency(&self) -> Result<Currency> {
        self.cashflow.currency()
    }
}

impl HasDiscountCurveId for FloatingRateCoupon {
    fn discount_curve_id(&self) -> Result<usize> {
        self.cashflow.discount_curve_id()
    }
}

impl HasForecastCurveId for FloatingRateCoupon {
    fn forecast_curve_id(&self) -> Result<usize> {
        self.forecast_curve_id
            .ok_or(AtlasError::ValueNotSetErr("Forecast curve id".to_string()))
    }
}

impl Registrable for FloatingRateCoupon {
    fn id(&self) -> Result<usize> {
        self.cashflow.id()
    }

    fn set_id(&mut self, id: usize) {
        self.cashflow.set_id(id);
    }

    fn market_request(&self) -> Result<MarketRequest> {
        let tmp = self.cashflow.market_request()?;
        let forecast_curve_id = self.forecast_curve_id()?;
        let fixing_date = self.fixing_date();
        let forecast = ForwardRateRequest::new(
            forecast_curve_id,
            fixing_date,
            self.accrual_start_date,
            self.accrual_end_date,
            self.rate_definition.compounding(),
            self.rate_definition.frequency(),
        );
        Ok(MarketRequest::new(
            tmp.id(),
            tmp.df(),
            Some(forecast),
            tmp.fx(),
        ))
    }
}

impl Expires for FloatingRateCoupon {
    fn is_expired(&self, date: Date) -> bool {
        self.cashflow.payment_date() < date
    }
}
