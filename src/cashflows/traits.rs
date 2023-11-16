use crate::{time::date::Date, utils::errors::Result};

use super::cashflow::Side;

/// # InterestAccrual
/// A trait that defines the accrual period of an instrument.
pub trait InterestAccrual {
    fn accrual_start_date(&self) -> Date;
    fn accrual_end_date(&self) -> Date;
    fn accrued_amount(&self, start_date: Date, end_date: Date) -> Result<f64>;

    fn relevant_accrual_dates(&self, start_date: Date, end_date: Date) -> (Date, Date) {
        let accrual_start = self.accrual_start_date();
        let accrual_end = self.accrual_end_date();

        // Check if the ranges intersect
        if start_date <= accrual_end && end_date >= accrual_start {
            // The ranges intersect, so we find the relevant accrual dates
            let relevant_start = if accrual_start >= start_date {
                accrual_start
            } else {
                start_date
            };

            let relevant_end = if accrual_end <= end_date {
                accrual_end
            } else {
                end_date
            };

            (relevant_start, relevant_end)
        } else {
            // The ranges do not intersect, so return Date::empty()
            (Date::empty(), Date::empty())
        }
    }

    fn delta_accrued_amount(&self, start_date: Date, end_date: Date) -> Result<f64> {
        let acc_1 = self.accrued_amount(self.accrual_start_date(), start_date)?;
        let acc_2 = self.accrued_amount(self.accrual_start_date(), end_date)?;
        return Ok(acc_2 - acc_1);
    }
}

/// # RequiresFixingRate
/// A trait that defines if an instrument requires a fixing rate.
pub trait RequiresFixingRate: InterestAccrual {
    fn set_fixing_rate(&mut self, fixing_rate: f64);
}

/// # Payable
/// A trait that defines the payment of an instrument.
pub trait Payable {
    fn amount(&self) -> Result<f64>;
    fn side(&self) -> Side;
    fn payment_date(&self) -> Date;
}

/// # Expires
/// A trait that defines if an instrument expires.
pub trait Expires {
    fn is_expired(&self, date: Date) -> bool;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::{
        Compounding, Currency, Date, DayCounter, FixedRateCoupon, Frequency, InterestRate, Side,
    };

    #[test]
    fn test_delta_accrued_amount_simple() {
        let notional = 10000.0;
        let rate = InterestRate::new(
            0.05,
            Compounding::Simple,
            Frequency::Annual,
            DayCounter::Thirty360,
        );
        let accrual_start_date = Date::new(2023, 1, 1);
        let accrual_end_date = Date::new(2023, 3, 31);
        let payment_date = Date::new(2023, 3, 31);
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

        let mut start_date = Date::new(2023, 1, 1);
        let mut end_date = Date::new(2023, 3, 31);
        let mut accrued_amount = coupon.delta_accrued_amount(start_date, end_date).unwrap();
        assert!((accrued_amount - 125.0).abs() < 0.00001);

        start_date = Date::new(2023, 1, 15);
        end_date = Date::new(2023, 1, 16);
        accrued_amount = coupon.delta_accrued_amount(start_date, end_date).unwrap();
        assert!((accrued_amount - 125.0 / 90.0).abs() < 0.00001);
    }

    #[test]
    fn test_delta_accrued_amount_compounded() {
        let notional = 10000.0;
        let rate = InterestRate::new(
            0.05,
            Compounding::Compounded,
            Frequency::Annual,
            DayCounter::Thirty360,
        );
        let accrual_start_date = Date::new(2023, 1, 30);
        let accrual_end_date = Date::new(2023, 3, 31);
        let payment_date = Date::new(2023, 3, 31);
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

        let start_date = Date::new(2023, 1, 30);
        let end_date = Date::new(2023, 3, 31);
        let accrued_amount = coupon
            .clone()
            .delta_accrued_amount(start_date, end_date)
            .unwrap();

        assert!(accrued_amount - 122.72234429 < 0.00001);

        // start_date = Date::new(2023, 2, 15);
        // end_date = Date::new(2023, 2, 16);
        // accrued_amount = coupon
        //     .clone()
        //     .delta_accrued_amount(start_date, end_date)
        //     .unwrap();
        // //println!("accrued_amount = {}", accrued_amount);

        // start_date = Date::new(2023, 2, 15);
        // end_date = Date::new(2023, 2, 16);
        // accrued_amount = coupon
        //     .clone()
        //     .delta_accrued_amount(start_date, end_date)
        //     .unwrap();
        // //println!("accrued_amount = {}", accrued_amount);

        // start_date = Date::new(2023, 1, 30);
        // end_date = Date::new(2023, 1, 31);
        // accrued_amount = coupon
        //     .clone()
        //     .delta_accrued_amount(start_date, end_date)
        //     .unwrap();
        // //println!("accrued_amount = {}", accrued_amount);
        // assert_eq!(accrued_amount, 0.0);
    }
}
