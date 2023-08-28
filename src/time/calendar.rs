use super::calendars::nullcalendar::NullCalendar;
use super::calendars::target::TARGET;
use super::calendars::traits::{ImplCalendar, IsCalendar};
use super::calendars::weekendsonly::WeekendsOnly;
use crate::time::date::Date;
use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Calendar {
    NullCalendar(NullCalendar),
    WeekendsOnly(WeekendsOnly),
    TARGET(TARGET),
}

impl ImplCalendar for Calendar {
    fn impl_name(&self) -> String {
        match self {
            Calendar::NullCalendar(cal) => cal.impl_name(),
            Calendar::WeekendsOnly(cal) => cal.impl_name(),
            Calendar::TARGET(cal) => cal.impl_name(),
        }
    }

    fn impl_is_business_day(&self, date: &Date) -> bool {
        match self {
            Calendar::NullCalendar(cal) => cal.impl_is_business_day(date),
            Calendar::WeekendsOnly(cal) => cal.impl_is_business_day(date),
            Calendar::TARGET(cal) => cal.impl_is_business_day(date),
        }
    }

    fn added_holidays(&self) -> HashSet<Date> {
        match self {
            Calendar::NullCalendar(cal) => cal.added_holidays(),
            Calendar::WeekendsOnly(cal) => cal.added_holidays(),
            Calendar::TARGET(cal) => cal.added_holidays(),
        }
    }

    fn removed_holidays(&self) -> HashSet<Date> {
        match self {
            Calendar::NullCalendar(cal) => cal.removed_holidays(),
            Calendar::WeekendsOnly(cal) => cal.removed_holidays(),
            Calendar::TARGET(cal) => cal.removed_holidays(),
        }
    }

    fn add_holiday(&mut self, date: Date) {
        match self {
            Calendar::NullCalendar(cal) => cal.add_holiday(date),
            Calendar::WeekendsOnly(cal) => cal.add_holiday(date),
            Calendar::TARGET(cal) => cal.add_holiday(date),
        }
    }

    fn remove_holiday(&mut self, date: Date) {
        match self {
            Calendar::NullCalendar(cal) => cal.remove_holiday(date),
            Calendar::WeekendsOnly(cal) => cal.remove_holiday(date),
            Calendar::TARGET(cal) => cal.remove_holiday(date),
        }
    }

    fn holiday_list(&self, from: Date, to: Date, include_weekends: bool) -> Vec<Date> {
        match self {
            Calendar::NullCalendar(cal) => cal.holiday_list(from, to, include_weekends),
            Calendar::WeekendsOnly(cal) => cal.holiday_list(from, to, include_weekends),
            Calendar::TARGET(cal) => cal.holiday_list(from, to, include_weekends),
        }
    }

    fn business_day_list(&self, from: Date, to: Date) -> Vec<Date> {
        match self {
            Calendar::NullCalendar(cal) => cal.business_day_list(from, to),
            Calendar::WeekendsOnly(cal) => cal.business_day_list(from, to),
            Calendar::TARGET(cal) => cal.business_day_list(from, to),
        }
    }
}

impl IsCalendar for Calendar {}
