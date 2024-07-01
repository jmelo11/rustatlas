use std::collections::HashSet;

use chrono::{Datelike, NaiveDate, Weekday};

use crate::time::date::Date;

use super::traits::{ImplCalendar, IsCalendar};

/// # Market
/// Defines the relevant market for the United States calendar.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Market {
    Settlement,
    LiborImpact,
    Nyse,
    GovernmentBond,
    Nerc,
    FederalReserve,
    Sofr,
}

/// # UnitedStates
/// A calendar for the United States.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnitedStates {
    market: Market,
    added_holidays: HashSet<Date>,
    removed_holidays: HashSet<Date>,
}

impl UnitedStates {
    pub fn new(market: Market) -> Self {
        UnitedStates {
            market,
            added_holidays: HashSet::new(),
            removed_holidays: HashSet::new(),
        }
    }

    fn is_weekend(day: Weekday) -> bool {
        day == Weekday::Sat || day == Weekday::Sun
    }

    fn is_washington_birthday(day: u32, month: u32, year: i32, weekday: Weekday) -> bool {
        match year {
            y if y >= 1971 => (day >= 15 && day <= 21) && weekday == Weekday::Mon && month == 2,
            _ => {
                (day == 22
                    || (day == 23 && weekday == Weekday::Mon)
                    || (day == 21 && weekday == Weekday::Fri))
                    && month == 2
            }
        }
    }

    fn is_memorial_day(day: u32, month: u32, year: i32, weekday: Weekday) -> bool {
        match year {
            y if y >= 1971 => day >= 25 && weekday == Weekday::Mon && month == 5,
            _ => {
                (day == 30
                    || (day == 31 && weekday == Weekday::Mon)
                    || (day == 29 && weekday == Weekday::Fri))
                    && month == 5
            }
        }
    }

    fn is_independence_day(day: u32, month: u32, weekday: Weekday) -> bool {
        (day == 4 || (day == 5 && weekday == Weekday::Mon) || (day == 3 && weekday == Weekday::Fri))
            && month == 7
    }

    fn is_thanksgiving(day: u32, month: u32, weekday: Weekday) -> bool {
        (day >= 22 && day <= 28) && weekday == Weekday::Thu && month == 11
    }

    fn is_christmas(day: u32, month: u32, weekday: Weekday) -> bool {
        (day == 25
            || (day == 26 && weekday == Weekday::Mon)
            || (day == 24 && weekday == Weekday::Fri))
            && month == 12
    }

    pub fn is_business_day(&self, date: NaiveDate) -> bool {
        let weekday = date.weekday();
        let day = date.day();
        let month = date.month();
        let year = date.year();

        if UnitedStates::is_weekend(weekday) {
            return false;
        }

        match self.market {
            Market::Nyse
            | Market::Settlement
            | Market::LiborImpact
            | Market::GovernmentBond
            | Market::Nerc
            | Market::FederalReserve
            | Market::Sofr => {
                UnitedStates::is_washington_birthday(day, month, year, weekday)
                    || UnitedStates::is_memorial_day(day, month, year, weekday)
                    || UnitedStates::is_independence_day(day, month, weekday)
                    || UnitedStates::is_thanksgiving(day, month, weekday)
                    || UnitedStates::is_christmas(day, month, weekday)
            }
        }
    }
}

impl ImplCalendar for UnitedStates {
    fn impl_is_business_day(&self, date: &Date) -> bool {
        self.is_business_day(date.base_date())
    }
    fn impl_name(&self) -> String {
        format!("UnitedStates({:?})", self.market)
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

    fn holiday_list(&self, from: Date, to: Date, include_weekends: bool) -> Vec<Date> {
        let mut holidays = vec![];
        let mut d = from;
        while d <= to {
            if self.is_holiday(&d) {
                holidays.push(d);
            }
            d = d + 1;
        }
        if include_weekends {
            holidays
        } else {
            holidays
                .into_iter()
                .filter(|d| !self.is_weekend(&d.weekday()))
                .collect()
        }
    }

    fn business_day_list(&self, from: Date, to: Date) -> Vec<Date> {
        let mut business_days = vec![];
        let mut d = from;
        while d <= to {
            if self.is_business_day(d.base_date()) {
                business_days.push(d);
            }
            d = d + 1;
        }
        business_days
    }
}

impl IsCalendar for UnitedStates {}

impl Default for UnitedStates {
    fn default() -> Self {
        UnitedStates::new(Market::Sofr)
    }
}
