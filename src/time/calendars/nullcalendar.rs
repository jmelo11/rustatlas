use std::collections::HashSet;

use crate::time::date::Date;

use super::calendar::{ImplCalendar, IsCalendar};

pub struct NullCalendar {
    added_holidays: HashSet<Date>,
    removed_holidays: HashSet<Date>,
}

impl ImplCalendar for NullCalendar {
    fn impl_name(&self) -> String {
        "NullCalendar".to_string()
    }

    fn impl_is_business_day(&self, _date: &Date) -> bool {
        true
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

impl IsCalendar for NullCalendar {}

mod tests {
    use super::*;
    use crate::time::calendars::calendar::IsCalendar;
    use crate::time::calendars::nullcalendar::NullCalendar;
    use crate::time::date::Date;
    use crate::time::period::Period;

    #[test]
    fn test_nullcalendar() {
        let mut cal = NullCalendar {
            added_holidays: HashSet::new(),
            removed_holidays: HashSet::new(),
        };
        assert_eq!(cal.name(), "NullCalendar");
        assert_eq!(cal.is_business_day(&Date::new(2021, 1, 1)), true);
        assert_eq!(cal.is_business_day(&Date::new(2021, 1, 2)), true);
    }
}
