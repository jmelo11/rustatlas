use std::{collections::HashSet, marker};

use chrono::{Datelike, NaiveDate, Weekday};

use crate::time::date::Date;
use super::traits::{easter_monday, ImplCalendar};

/// # Chile 
/// A calendar for Chile
/// 

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Market {
    SSE, // Santiago Stock Exchange
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chile {
    market: Market,
    added_holidays: HashSet<Date>,
    removed_holidays: HashSet<Date>,
}

impl Chile {
    pub fn new(market: Market) -> Self {
        Chile {
            market,
            added_holidays: HashSet::new(),
            removed_holidays: HashSet::new(),
        }
    }

    fn is_weekend(day: Weekday) -> bool {
        day == Weekday::Sat || day == Weekday::Sun
    }

    fn is_new_years_day(day: u32, month: u32, year: i32) -> bool {
        let w = NaiveDate::from_ymd_opt(year, month, day).unwrap().weekday();
        (day == 1 && month == 1)    
            || (day == 2 && month == 1 && w == Weekday::Mon && year >= 2016)
    }
    
    fn is_good_friday(day: u32, month: u32, year: i32) -> bool {
        let easter_friday = easter_monday(year) - 3;
        let dd = Date::new(year, month, day).day_of_year();
        dd == easter_friday
    }

    fn is_easter_saturday(day: u32, month: u32, year: i32) -> bool {
        let easter_saturday = easter_monday(year) - 2;
        let dd = Date::new(year, month, day).day_of_year();
        dd == easter_saturday
    }

    fn is_labour_day(day: u32, month: u32) -> bool {
        day == 1 && month == 5
    }

    fn is_navy_day(day: u32, month: u32) -> bool {
        day == 21 && month == 5
    }

    fn is_aboriginal_peoples_day(day: u32, month: u32, year: i32) -> bool {
        day == 21 && month == 6 && year >= 2021
    }

    fn is_saint_peter_and_saint_paul_day(day: u32, month: u32) -> bool {
        let w = NaiveDate::from_ymd_opt(2001, month, day).unwrap().weekday();
        day >= 26 && day <= 29 && month == 6 && w == Weekday::Mon
            || day == 2 && month == 7 && w == Weekday::Mon 
    }

    fn is_our_lady_of_mount_carmel_day(day: u32, month: u32) -> bool {
        day == 16 && month == 7
    }

    fn is_assumption_day(day: u32, month: u32) -> bool {
        day == 15 && month == 8
    }

    fn is_independence_day(day: u32, month: u32, year: i32) -> bool {
         let w = NaiveDate::from_ymd_opt(1810, month, day).unwrap().weekday();
         (day == 17 && month == 9 && ((w == Weekday::Mon && year >= 2007) || (w == Weekday::Fri && year >= 2016)))
            || (day == 18 && month == 9 )

    }

    fn is_army_day(day: u32, month: u32,year: i32) -> bool {
        let w = NaiveDate::from_ymd_opt(1810, month, day).unwrap().weekday();
        (day == 19 && month == 9)
            || (day == 20 && month == 9  && w == Weekday::Fri && year >= 2007)
    }

    fn is_discovery_of_two_worlds(day: u32, month: u32) -> bool {
        let w = NaiveDate::from_ymd_opt(1492, month, day).unwrap().weekday();
        (day >= 9 && day <= 12 && month == 10 && w == Weekday::Mon)
            || (day == 15 && month == 10 && w == Weekday::Mon)
    }

    fn is_reformation_day(day: u32, month: u32, year: i32) -> bool {
        let w = NaiveDate::from_ymd_opt(year, month, day).unwrap().weekday();
        ((day == 27 && month == 10 && w == Weekday::Fri)
                 || (day == 31 && month == 10 && w != Weekday::Tue && w != Weekday::Wed)
                 || (day == 2 && month == 11 && w == Weekday::Fri)) 
                 && year >= 2008
    }

    fn is_all_saints_day(day: u32, month: u32) -> bool {
        day == 1 && month == 11
    }

    fn is_immaculate_conception(day: u32, month: u32) -> bool {
        day == 8 && month == 12
    }

    fn is_christmas_day(day: u32, month: u32) -> bool {
        day == 25 && month == 12
    }

    pub fn is_business_day(&self, date: NaiveDate) -> bool {
        let weekday = date.weekday();
        let day = date.day();
        let month = date.month();
        let year = date.year();
        if Chile::is_weekend(weekday) {
            return false;
        }
        
        match self.market {
            Market::SSE => {
                Chile::is_new_years_day(day, month, year)
                    || Chile::is_good_friday(day, month, year)
                    || Chile::is_easter_saturday(day, month, year)
                    || Chile::is_labour_day(day, month)
                    || Chile::is_navy_day(day, month)
                    || Chile::is_aboriginal_peoples_day(day, month, year)
                    || Chile::is_saint_peter_and_saint_paul_day(day, month)
                    || Chile::is_our_lady_of_mount_carmel_day(day, month)
                    || Chile::is_assumption_day(day, month)
                    || Chile::is_independence_day(day, month, year)
                    || Chile::is_army_day(day, month, year)
                    || Chile::is_discovery_of_two_worlds(day, month)
                    || Chile::is_reformation_day(day, month, year)
                    || Chile::is_all_saints_day(day, month)
                    || Chile::is_immaculate_conception(day, month)
                    || Chile::is_christmas_day(day, month)
            }
        }
    }
}

impl ImplCalendar for Chile {
    fn impl_is_business_day(&self, date: &Date) -> bool {
        self.is_business_day(date.base_date())
    }

    fn impl_name(&self) -> String {
        format!("Brazil({:?})", self.market)
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


impl Default for Chile {
    fn default() -> Self {
        Chile::new(Market::SSE)
    }
}
