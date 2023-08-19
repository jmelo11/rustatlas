use crate::rates::interestrate::{DiscountFactor, InsterestRate};
use crate::time::date::Date;

pub trait RateAccrual {
    fn accrual_start_date(&self) -> Date;
    fn accrual_end_date(&self) -> Date;
    fn notional(&self) -> f64;
    fn rate(&self) -> &InsterestRate;
    fn accrued_amount(&self, start_date: Date, end_date: Date) -> f64 {
        return self.notional() * (self.rate().compound_factor(start_date, end_date) - 1.0);
    }
}
