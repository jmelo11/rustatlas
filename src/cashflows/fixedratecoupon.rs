use super::cashflow::SimpleCashflow;
use super::traits::{InterestAccrual, Payable};
use crate::core::enums::Side;
use crate::core::meta::MetaMarketData;
use crate::core::registry::Registrable;
use crate::currencies::enums::Currency;
use crate::rates::interestrate::InterestRate;
use crate::rates::traits::YieldProvider;
use crate::time::date::Date;

pub struct FixedRateCoupon {
    notional: f64,
    rate: InterestRate,
    accrual_start_date: Date,
    accrual_end_date: Date,
    cashflow: SimpleCashflow,
}

impl FixedRateCoupon {
    pub fn new(
        notional: f64,
        rate: InterestRate,
        accrual_start_date: Date,
        accrual_end_date: Date,
        payment_date: Date,
        discount_curve_id: usize,
        currency: Currency,
        side: Side,
    ) -> FixedRateCoupon {
        let amount = notional * (rate.compound_factor(accrual_start_date, accrual_end_date) - 1.0);
        FixedRateCoupon {
            notional,
            rate,
            accrual_start_date,
            accrual_end_date,
            cashflow: SimpleCashflow::new(amount, payment_date, discount_curve_id, currency, side),
        }
    }
}

impl Registrable for FixedRateCoupon {
    fn registry_id(&self) -> Option<usize> {
        return self.cashflow.registry_id();
    }

    fn register_id(&mut self, id: usize) {
        self.cashflow.register_id(id);
    }

    fn meta_market_data(&self) -> MetaMarketData {
        return self.cashflow.meta_market_data();
    }
}

impl InterestAccrual for FixedRateCoupon {
    fn accrual_start_date(&self) -> Date {
        return self.accrual_start_date;
    }
    fn accrual_end_date(&self) -> Date {
        return self.accrual_end_date;
    }
    fn accrued_amount(&self, start_date: Date, end_date: Date) -> f64 {
        let (d1, d2) = self.relevant_accrual_dates(start_date, end_date);
        return self.notional * (self.rate.compound_factor(d1, d2) - 1.0);
    }
}

impl Payable for FixedRateCoupon {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::enums::Side;
    use crate::currencies::enums::Currency;
    use crate::rates::enums::Compounding;
    use crate::rates::interestrate::InterestRate;
    use crate::time::date::Date;
    use crate::time::daycounters::enums::DayCounter;
    use crate::time::enums::Frequency;

    #[test]
    fn test_fixed_rate_coupon_creation() {
        let notional = 1000.0;
        let rate = InterestRate::new(
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

        let coupon = FixedRateCoupon::new(
            notional,
            rate,
            accrual_start_date,
            accrual_end_date,
            payment_date,
            discount_curve_id,
            currency,
            Side::Pay,
        );

        assert_eq!(coupon.accrual_start_date(), accrual_start_date);
        assert_eq!(coupon.accrual_end_date(), accrual_end_date);
    }

    #[test]
    fn test_amount_calculation() {
        let notional = 1000.0;
        let rate = InterestRate::new(
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

        let coupon = FixedRateCoupon::new(
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
        assert_eq!(
            coupon.accrued_amount(accrual_start_date, accrual_end_date),
            expected_amount
        );
    }
}
