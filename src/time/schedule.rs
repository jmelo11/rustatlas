use super::date::Date;
use super::enums::Frequency;
use super::period::Period;

/// # Schedule
/// Schedule struct.
/// # Example
/// ```
/// use rustatlas::time::schedule::Schedule;
/// use rustatlas::time::date::Date;
/// use rustatlas::time::enums::TimeUnit;
/// use rustatlas::time::period::Period;
///
/// let start_date = Date::from_ymd(2022, 1, 1);
/// let end_date = Date::from_ymd(2022, 3, 1);
/// let period = Period::new(1, TimeUnit::Months);
/// let schedule = Schedule::generate_schedule_with_tenor(start_date, end_date, period);
/// assert_eq!(
///    schedule.dates(),
///   &vec![
///       Date::from_ymd(2022, 1, 1),
///      Date::from_ymd(2022, 2, 1),
///     Date::from_ymd(2022, 3, 1),
/// ]
/// );
/// ```
pub struct Schedule {
    dates: Vec<Date>,
    current_index: usize,
}

impl Schedule {
    pub fn new(dates: Vec<Date>) -> Schedule {
        Schedule {
            dates,
            current_index: 0,
        }
    }

    pub fn generate_schedule_with_tenor(
        start_date: Date,
        end_date: Date,
        period: Period,
    ) -> Schedule {
        let mut dates = Vec::new();
        let mut current_date = start_date;
        while current_date <= end_date {
            dates.push(current_date);
            current_date = current_date + period;
        }
        if dates.last().unwrap() != &end_date {
            dates.push(end_date);
        }
        return Schedule::new(dates);
    }

    pub fn generate_schedule_with_frequency(
        start_date: Date,
        end_date: Date,
        frequency: Frequency,
    ) -> Schedule {
        let period = match Period::from_frequency(frequency) {
            Ok(p) => p,
            Err(_) => panic!("Invalid frequency"),
        };

        let mut dates = Vec::new();
        let mut current_date = start_date;
        while current_date <= end_date {
            dates.push(current_date);
            current_date = current_date + period;
        }
        if dates.last().unwrap() != &end_date {
            dates.push(end_date);
        }
        return Schedule::new(dates);
    }

    pub fn dates(&self) -> &Vec<Date> {
        return &self.dates;
    }
}

/// # DatePairProvider
/// Iterator over a schedule that returns a pair of dates.
/// # Example
/// ```
/// use rustatlas::time::schedule::{Schedule, DatePairProvider};
/// use rustatlas::time::date::Date;
/// use rustatlas::time::enums::TimeUnit;
/// use rustatlas::time::period::Period;
///
/// let start_date = Date::from_ymd(2022, 1, 1);
/// let end_date = Date::from_ymd(2022, 3, 1);
/// let period = Period::new(1, TimeUnit::Months);
/// let mut schedule = Schedule::generate_schedule_with_tenor(start_date, end_date, period);
/// assert_eq!(schedule.next_pair(), Some((Date::from_ymd(2022, 1, 1), Date::from_ymd(2022, 2, 1))));
/// assert_eq!(schedule.next_pair(), Some((Date::from_ymd(2022, 2, 1), Date::from_ymd(2022, 3, 1))));
/// assert_eq!(schedule.next_pair(), None);
/// ```
pub trait DatePairProvider {
    fn next_pair(&mut self) -> Option<(Date, Date)>;
}

impl DatePairProvider for Schedule {
    fn next_pair(&mut self) -> Option<(Date, Date)> {
        if self.current_index > self.dates.len() - 2 {
            self.current_index = 0;
            return None;
        }
        let first = self.dates[self.current_index];
        let second = self.dates[self.current_index + 1];
        self.current_index += 1;
        return Some((first, second));
    }
}

#[cfg(test)]
mod tests {
    use super::super::enums::{Frequency, TimeUnit};
    use super::super::period::Period;
    use super::*;
    #[test]
    fn test_new_schedule() {
        let dates = vec![Date::from_ymd(2022, 1, 1), Date::from_ymd(2022, 2, 1)];
        let schedule = Schedule::new(dates.clone());

        assert_eq!(schedule.dates(), &dates);
    }

    #[test]
    fn test_generate_schedule_with_tenor() {
        let start_date = Date::from_ymd(2022, 1, 1);
        let end_date = Date::from_ymd(2022, 3, 1);
        let period = Period::new(1, TimeUnit::Months); // Assuming you have a TimeUnit::Months

        let schedule = Schedule::generate_schedule_with_tenor(start_date, end_date, period);
        assert_eq!(
            schedule.dates(),
            &vec![
                Date::from_ymd(2022, 1, 1),
                Date::from_ymd(2022, 2, 1),
                Date::from_ymd(2022, 3, 1),
            ]
        );
    }

    #[test]
    fn test_generate_schedule_with_frequency() {
        let start_date = Date::from_ymd(2022, 1, 1);
        let end_date = Date::from_ymd(2023, 6, 1);
        let frequency = Frequency::Semiannual; // Assuming Frequency::Quarterly corresponds to 3 months

        let schedule = Schedule::generate_schedule_with_frequency(start_date, end_date, frequency);
        assert_eq!(
            schedule.dates(),
            &vec![
                Date::from_ymd(2022, 1, 1),
                Date::from_ymd(2022, 7, 1),
                Date::from_ymd(2023, 1, 1),
                Date::from_ymd(2023, 6, 1)
            ]
        );
    }

    #[test]
    fn test_dates_method() {
        let dates = vec![Date::from_ymd(2022, 1, 1), Date::from_ymd(2022, 2, 1)];
        let schedule = Schedule::new(dates.clone());
        assert_eq!(schedule.dates(), &dates);
    }
}
