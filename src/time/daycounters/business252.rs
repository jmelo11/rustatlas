use std::collections::HashMap;

use crate::time::{calendar::Calendar, calendars::traits::ImplCalendar, date::Date};

/// # Business252
/// Business/252 day count convention.



#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Business252{
    calendar: Calendar,
    cache: HashMap<(i32, u32), i32>,
    outer_cache : HashMap<i32, i32>,
}


impl Business252 {
    pub fn new(calendar: Calendar) -> Self {
        Business252 {
            calendar,
            cache: HashMap::new(),
            outer_cache: HashMap::new(),
        }
    }

    //fn business_days_monthly(&mut self, year: i32, month: u32) -> i32 {
    //    if let Some(&days) = self.cache.get(&(year, month)) {
    //        return days;
    //    } else {
    //        let d1 = Date::new(year, month, 1);
    //        let d2 = d1 + Period::new(1, TimeUnit::Months); 
    //        let days = self.calendar.business_day_list(d1, d2).len() as i32;
    //        self.cache.insert((year, month), days);
    //        return days;
    //    }
    //}

    //fn business_days_yearly(&mut self, year: i32) -> i32 {
    //    if let Some(&days) = self.outer_cache.get(&year) {
    //        return days;
    //    } else {
    //        let mut total = 0;
    //        for month in 1..=12 {
    //            total += self.business_days_monthly(year, month);
    //        }
    //        self.outer_cache.insert(year, total);
    //        return total;
    //    }
    //}

    pub fn day_count(start: Date, end: Date, calendar: Calendar) -> i64 {
        if end < start {
            return  -(calendar.business_day_list(start, end).len() as i64);
        }
        else {
            return calendar.business_day_list(start, end).len() as i64
        }
    }

    pub fn year_fraction(start: Date, end: Date, calendar: Calendar) -> f64 {
        Self::day_count(start, end, calendar) as f64 / 252.0
    }
}
