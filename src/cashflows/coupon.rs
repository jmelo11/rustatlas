use super::cashflow::Cashflow;
use super::traits::RateAccrual;
use crate::core::enums::Side;
use crate::core::meta::MetaMarketData;
use crate::core::registry::Registrable;
use crate::currencies::enums::Currency;
use crate::rates::interestrate::{DiscountFactor, InsterestRate};
use crate::time::date::Date;

pub struct Coupon {
    notional: f64,
    rate: InsterestRate,
    accrual_start_date: Date,
    accrual_end_date: Date,
    cashflow: Cashflow,
}

impl Coupon {
    pub fn new(
        notional: f64,
        rate: InsterestRate,
        accrual_start_date: Date,
        accrual_end_date: Date,
        payment_date: Date,
        discount_curve_id: u16,
        currency: Currency,
        side: Side,
    ) -> Coupon {
        let amount = notional * (rate.compound_factor(accrual_start_date, accrual_end_date) - 1.0);
        Coupon {
            notional,
            rate,
            accrual_start_date,
            accrual_end_date,
            cashflow: Cashflow::new(amount, payment_date, discount_curve_id, currency, side),
        }
    }

    pub fn notional(&self) -> f64 {
        return self.notional;
    }

    pub fn rate(&self) -> &InsterestRate {
        return &self.rate;
    }

    pub fn accrual_start_date(&self) -> Date {
        return self.accrual_start_date;
    }

    pub fn accrual_end_date(&self) -> Date {
        return self.accrual_end_date;
    }

    pub fn payment_date(&self) -> Date {
        return self.cashflow.payment_date();
    }

    pub fn amount(&self) -> f64 {
        return self.cashflow.amount();
    }
}

impl Registrable for Coupon {
    fn registry_id(&self) -> Option<u64> {
        return self.cashflow.registry_id();
    }

    fn register_id(&mut self, id: u64) {
        self.cashflow.register_id(id);
    }

    fn meta_market_data(&self) -> MetaMarketData {
        return self.cashflow.meta_market_data();
    }
}

impl RateAccrual for Coupon {
    fn accrual_start_date(&self) -> Date {
        return self.accrual_start_date;
    }
    fn accrual_end_date(&self) -> Date {
        return self.accrual_end_date;
    }
    fn notional(&self) -> f64 {
        return self.notional;
    }
    fn rate(&self) -> &InsterestRate {
        return &self.rate;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::enums::Side;
    use crate::currencies::enums::Currency;
    use crate::rates::enums::Compounding;
    use crate::rates::interestrate::InsterestRate;
    use crate::time::date::Date;
    use crate::time::daycounters::enums::DayCounter;
    use crate::time::enums::Frequency;
    use std::sync::Arc;

    #[test]
    fn test_coupon_creation() {
        let notional = 1000.0;
        let rate = InsterestRate::new(
            0.05,
            Compounding::Compounded,
            Frequency::Annual,
            DayCounter::Thirty360,
        );
        let accrual_start_date = Date::from_ymd(2023, 1, 1);
        let accrual_end_date = Date::from_ymd(2023, 12, 31);
        let payment_date = Date::from_ymd(2024, 1, 1);
        let discount_curve_id = 1;
        let currency = Currency::JPY;

        let coupon = Coupon::new(
            notional,
            rate,
            accrual_start_date,
            accrual_end_date,
            payment_date,
            discount_curve_id,
            currency,
            Side::Pay,
        );

        assert_eq!(coupon.notional(), notional);
        assert_eq!(coupon.accrual_start_date(), accrual_start_date);
        assert_eq!(coupon.accrual_end_date(), accrual_end_date);
        assert_eq!(coupon.payment_date(), payment_date);
    }

    #[test]
    fn test_amount_calculation() {
        let notional = 1000.0;
        let rate = InsterestRate::new(
            0.05,
            Compounding::Compounded,
            Frequency::Annual,
            DayCounter::Actual360,
        );
        let accrual_start_date = Date::from_ymd(2023, 1, 1);
        let accrual_end_date = Date::from_ymd(2023, 12, 31);
        let payment_date = Date::from_ymd(2024, 1, 1);
        let discount_curve_id = 1;
        let currency = Currency::USD;

        let coupon = Coupon::new(
            notional,
            rate,
            accrual_start_date,
            accrual_end_date,
            payment_date,
            discount_curve_id,
            currency,
            Side::Pay,
        );

        let expected_amount =
            notional * (rate.compound_factor(accrual_start_date, accrual_end_date) - 1.0);
        assert_eq!(coupon.amount(), expected_amount);
    }
}
