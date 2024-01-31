use crate::time::date::Date;
use std::collections::HashSet;

use super::traits::{ImplCalendar, IsCalendar};

/// # WeekendsOnly
/// A calendar that considers only weekends as business days.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WeekendsOnly {
    added_holidays: HashSet<Date>,
    removed_holidays: HashSet<Date>,
}

impl WeekendsOnly {
    pub fn new() -> Self {
        WeekendsOnly {
            added_holidays: HashSet::new(),
            removed_holidays: HashSet::new(),
        }
    }
}

impl ImplCalendar for WeekendsOnly {
    fn impl_is_business_day(&self, date: &Date) -> bool {
        !self.is_weekend(&date.weekday())
    }

    fn impl_name(&self) -> String {
        "WeekendsOnly".to_string()
    }

    fn added_holidays(&self) -> HashSet<Date> {
        self.added_holidays.clone()
    }

    fn removed_holidays(&self) -> HashSet<Date> {
        self.removed_holidays.clone()
    }

    fn add_holiday(&mut self, date: Date) {
        self.added_holidays.insert(date);
    }

    fn remove_holiday(&mut self, date: Date) {
        self.removed_holidays.insert(date);
    }

    fn holiday_list(&self, _from: Date, _to: Date, _include_weekends: bool) -> Vec<Date> {
        vec![]
    }

    fn business_day_list(&self, _from: Date, _to: Date) -> Vec<Date> {
        vec![]
    }
}

impl IsCalendar for WeekendsOnly {}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::time::{
        calendars::{traits::IsCalendar, weekendsonly::WeekendsOnly},
        date::Date,
    };

    #[test]
    fn test_weekendsonly() {
        let cal = WeekendsOnly {
            added_holidays: HashSet::new(),
            removed_holidays: HashSet::new(),
        };
        assert_eq!(cal.name(), "WeekendsOnly");
        assert_eq!(cal.is_business_day(&Date::new(2023, 8, 23)), true);
        assert_eq!(cal.is_business_day(&Date::new(2023, 8, 25)), true);
    }
}
