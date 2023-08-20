use super::enums::Compounding;
use crate::time::{date::Date, enums::Frequency};

/// # YieldProvider
/// Implement this trait for a struct that provides yield information.
/// # Example
/// ```
/// use rustatlas::time::date::Date;
/// use rustatlas::rates::enums::Compounding;
/// use rustatlas::time::enums::Frequency;
/// use rustatlas::rates::interestrate::InterestRate;
/// use rustatlas::time::daycounters::enums::DayCounter;
/// use rustatlas::time::daycounters::traits::DayCountProvider;
/// let start_date = Date::from_ymd(2020, 1, 1);
/// let end_date = Date::from_ymd(2020, 2, 1);
/// let day_count = DayCounter::Thirty360;
/// let compounding = Compounding::Continuous;
/// let frequency = Frequency::Annual;
/// let rate = InterestRate::new(0.05, day_count, compounding, frequency);
/// assert_eq!(rate.compound_factor(start_date, end_date), 1.0041666666666667);
/// assert_eq!(rate.discount_factor(start_date, end_date), 0.9958514173998045);
/// assert_eq!(rate.forward_rate(start_date, end_date, compounding, frequency), 0.05);
/// ```
pub trait YieldProvider {
    fn compound_factor(&self, start: Date, end: Date) -> f64;
    fn discount_factor(&self, start: Date, end: Date) -> f64;
    fn forward_rate(
        &self,
        start_date: Date,
        end_date: Date,
        comp: Compounding,
        freq: Frequency,
    ) -> f64;
}
