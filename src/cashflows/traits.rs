use crate::time::date::Date;

use super::cashflow::Side;

/// # InterestAccrual
/// A trait that defines the accrual period of an instrument.
pub trait InterestAccrual {
    fn accrual_start_date(&self) -> Date;
    fn accrual_end_date(&self) -> Date;
    fn accrued_amount(&self, start_date: Date, end_date: Date) -> f64;
    fn relevant_accrual_dates(&self, start_date: Date, end_date: Date) -> (Date, Date) {
        let first_date = if start_date < self.accrual_start_date() {
            self.accrual_start_date()
        } else {
            start_date
        };
        let last_date = if end_date > self.accrual_end_date() {
            self.accrual_end_date()
        } else {
            end_date
        };
        return (first_date, last_date);
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
    fn amount(&self) -> f64;
    fn side(&self) -> Side;
    fn payment_date(&self) -> Date;
}

/// # Expires
/// A trait that defines if an instrument expires.
pub trait Expires {
    fn is_expired(&self, date: Date) -> bool;
}
