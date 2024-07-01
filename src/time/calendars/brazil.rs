use std::collections::HashSet;

use chrono::{Datelike, NaiveDate, Weekday};

use crate::time::date::Date;
use super::traits::{easter_monday, ImplCalendar, IsCalendar};

/// # Brazil     
/// A calendar for Brazil 
/// 

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Market {
    Settlement,
    Exchange,
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Brazil {
    market: Market,
    added_holidays: HashSet<Date>,
    removed_holidays: HashSet<Date>,
}


impl Brazil {
    pub fn new(market: Market) -> Self {
        Brazil {
            market,
            added_holidays: HashSet::new(),
            removed_holidays: HashSet::new(),
        }
    } 

    fn is_weekend(day: Weekday) -> bool {
        day == Weekday::Sat || day == Weekday::Sun
    }

    fn is_new_years_day(day: u32, month: u32) -> bool {
        day == 1 && month == 1
    }

    fn is_sao_paulo_city_day(day: u32, month: u32) -> bool {
        day == 25 && month == 1
    }

    fn is_tiradentes_day(day: u32, month: u32) -> bool {
        day == 21 && month == 4
    }

    fn is_labor_day(day: u32, month: u32) -> bool {
        day == 1 && month == 5
    }

    fn is_revolution_day(day: u32, month: u32) -> bool {
        day == 9 && month == 7
    }

    fn is_independence_day(day: u32, month: u32) -> bool {
        day == 7 && month == 9
    }

    fn is_nossa_senhora_aparecida_day(day: u32, month: u32) -> bool {
        day == 12 && month == 10
    }

    fn is_all_souls_day(day: u32, month: u32) -> bool {
        day == 2 && month == 11
    }

    fn is_republic_day(day: u32, month: u32) -> bool {
        day == 15 && month == 11
    }

    fn is_black_consciousness_day(day: u32, month: u32, year: i32) -> bool {
        day == 20 && month == 11 && year >= 2007
    }

    fn is_christmas_eve(day: u32, month: u32) -> bool {
        day == 24 && month == 12
    }

    fn is_christmas(day: u32, month: u32) -> bool {
        day == 25 && month == 12
    }

    fn is_passion_of_christ(day: u32, month: u32, year: i32) -> bool {
        let em = easter_monday(year);
        let dd = Date::new(year, month, day).day_of_year();
        if em-3 == dd {
            return true;
        }
        false
    }

    fn is_carnival(day: u32, month: u32, year: i32) -> bool {
        let em = easter_monday(year);
        let dd = Date::new(year, month, day).day_of_year();
        em-49 == dd || em-48 == dd
    }

    fn is_corpus_christi(day: u32, month: u32, year: i32) -> bool {
        let em = easter_monday(year);
        let dd = Date::new(year, month, day).day_of_year();
        em+59 == dd 
    }

    fn is_last_business_day_of_year(day: u32, month: u32, year: i32) -> bool {
        let w = NaiveDate::from_ymd_opt(year, month, day).unwrap().weekday();
        month == 12 && (day == 31 || (day >= 29 && w == Weekday::Fri )) 
        
    }

    pub fn is_business_day(&self, date: NaiveDate) -> bool {
        let weekday = date.weekday();
        let day = date.day();
        let month = date.month();
        let year = date.year();
        if Brazil::is_weekend(weekday) {
            return false;
        }

        match self.market {
            Market::Settlement => {
                if Brazil::is_new_years_day( day, month) 
                || Brazil::is_tiradentes_day(day, month)
                || Brazil::is_labor_day(day, month)
                || Brazil::is_independence_day(day, month)
                || Brazil::is_nossa_senhora_aparecida_day(day, month)
                || Brazil::is_all_souls_day(day, month)
                || Brazil::is_republic_day(day, month)
                || Brazil::is_christmas(day, month)
                || Brazil::is_passion_of_christ(day, month, year)
                || Brazil::is_carnival(day, month, year)
                || Brazil::is_corpus_christi(day, month, year)
                {
                    return false;
                }
                return true;
            }
            Market::Exchange => {
                if Brazil::is_new_years_day( day, month) 
                || Brazil::is_sao_paulo_city_day(day, month)
                || Brazil::is_tiradentes_day(day, month)
                || Brazil::is_labor_day(day, month)
                || Brazil::is_revolution_day(day, month)
                || Brazil::is_independence_day(day, month)
                || Brazil::is_nossa_senhora_aparecida_day(day, month)
                || Brazil::is_all_souls_day(day, month)
                || Brazil::is_republic_day(day, month)
                || Brazil::is_black_consciousness_day(day, month, year)
                || Brazil::is_christmas_eve(day, month)
                || Brazil::is_christmas(day, month)
                || Brazil::is_passion_of_christ(day, month, year)
                || Brazil::is_carnival(day, month, year)
                || Brazil::is_corpus_christi(day, month, year)
                || Brazil::is_last_business_day_of_year(day, month, year)
                {
                    return false;
                }
                return true;
            }

        }
    }

}

impl ImplCalendar for Brazil {
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

    fn holiday_list(&self, from: Date, to: Date, include_weekends: bool) -> Vec<Date> {
        let mut holidays = vec![];
        let mut d = from;
        while d <= to {
            if self.is_business_day(d.base_date()) {
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


impl IsCalendar for Brazil {}

impl Default for Brazil {
    fn default() -> Self {
        Brazil::new(Market::Settlement)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::time::date::Date;

    #[test]
    fn test_brazil_settlement() {
        let cal = Brazil::new(Market::Settlement);
        let expected_hol = vec![
            Date::new(2005,2,7),
            Date::new(2005,2,8),
            Date::new(2005,3,25),
            Date::new(2005,4,21),
            Date::new(2005,5,26),
            Date::new(2005,9,7),
            Date::new(2005,10,12),
            Date::new(2005,11,2),
            Date::new(2005,11,15),
            Date::new(2006,2,27),
            Date::new(2006,2,28),
            Date::new(2006,4,14),
            Date::new(2006,4,21),
            Date::new(2006,5,1),
            Date::new(2006,6,15),
            Date::new(2006,9,7),
            Date::new(2006,10,12),
            Date::new(2006,11,2),
            Date::new(2006,11,15),
        ];

        for d in expected_hol {
            assert_eq!(cal.is_business_day(d.base_date()), false);
        }
    }
}