use super::cashflow::Side;
use super::simplecashflow::SimpleCashflow;
use super::traits::{Expires, InterestAccrual, Payable};
use crate::core::meta::MarketRequest;
use crate::core::traits::Registrable;
use crate::currencies::enums::Currency;
use crate::rates::interestrate::InterestRate;
use crate::time::date::Date;

/// # FixedRateCoupon
/// A fixed rate coupon is a cashflow that pays a fixed rate of interest on a notional amount.
///
/// ## Parameters
/// * `notional` - The notional amount of the coupon
/// * `rate` - The fixed rate of interest
/// * `accrual_start_date` - The date from which the coupon accrues interest
/// * `accrual_end_date` - The date until which the coupon accrues interest
/// * `payment_date` - The date on which the coupon is paid
/// * `currency` - The currency of the coupon
/// * `side` - The side of the coupon (Pay or Receive)
#[derive(Clone, Copy)]
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
        currency: Currency,
        side: Side,
    ) -> FixedRateCoupon {
        let amount = notional * (rate.compound_factor(accrual_start_date, accrual_end_date) - 1.0);
        let cashflow = SimpleCashflow::new(payment_date, currency, side).with_amount(amount);
        FixedRateCoupon {
            notional,
            rate,
            accrual_start_date,
            accrual_end_date,
            cashflow: cashflow,
        }
    }

    pub fn with_discount_curve_id(mut self, discount_curve_id: usize) -> FixedRateCoupon {
        self.cashflow = self.cashflow.with_discount_curve_id(discount_curve_id);
        self
    }

    pub fn set_discount_curve_id(&mut self, id: usize) {
        self.cashflow.set_discount_curve_id(id);
    }
}

impl Registrable for FixedRateCoupon {
    fn registry_id(&self) -> Option<usize> {
        return self.cashflow.registry_id();
    }

    fn register_id(&mut self, id: usize) {
        self.cashflow.register_id(id);
    }

    fn market_request(&self) -> MarketRequest {
        return self.cashflow.market_request();
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

impl Expires for FixedRateCoupon {
    fn is_expired(&self, date: Date) -> bool {
        return self.cashflow.is_expired(date);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cashflows::cashflow::Side;
    use crate::currencies::enums::Currency;
    use crate::rates::enums::Compounding;
    use crate::rates::interestrate::InterestRate;
    use crate::time::date::Date;
    use crate::time::daycounter::DayCounter;
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
        let accrual_start_date = Date::new(2023, 1, 1);
        let accrual_end_date = Date::new(2023, 12, 31);
        let payment_date = Date::new(2024, 1, 1);
        let currency = Currency::JPY;

        let coupon = FixedRateCoupon::new(
            notional,
            rate,
            accrual_start_date,
            accrual_end_date,
            payment_date,
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
        let accrual_start_date = Date::new(2023, 1, 1);
        let accrual_end_date = Date::new(2023, 12, 31);
        let payment_date = Date::new(2024, 1, 1);
        let discount_curve_id = 1;
        let currency = Currency::USD;

        let mut coupon = FixedRateCoupon::new(
            notional,
            rate,
            accrual_start_date,
            accrual_end_date,
            payment_date,
            currency,
            Side::Pay,
        );

        coupon.set_discount_curve_id(discount_curve_id);

        let expected_amount =
            notional * (rate.compound_factor(accrual_start_date, accrual_end_date) - 1.0);
        assert_eq!(
            coupon.accrued_amount(accrual_start_date, accrual_end_date),
            expected_amount
        );
    }
}
