use crate::{time::date::Date, utils::errors::Result};

use super::cashflow::Side;

/// # InterestAccrual
/// A trait that defines the accrual period of an instrument.
pub trait InterestAccrual {
    fn accrual_start_date(&self) -> Date;
    fn accrual_end_date(&self) -> Date;
    fn accrued_amount(&self, start_date: Date, end_date: Date) -> f64;

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
