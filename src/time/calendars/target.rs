use std::collections::HashSet;

use crate::time::date::Date;

use super::traits::{ImplCalendar, IsCalendar};

/// # TARGET
/// The TARGET calendar is the calendar for the European Union and is used for many EUR denominated
/// bonds. It is also the basis for the Euro Overnight Index Average (EONIA) rate.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TARGET {
    added_holidays: HashSet<Date>,
    removed_holidays: HashSet<Date>,
}

impl TARGET {
    pub fn new() -> TARGET {
        TARGET {
            added_holidays: HashSet::new(),
            removed_holidays: HashSet::new(),
        }
    }
}

impl ImplCalendar for TARGET {
    fn impl_name(&self) -> String {
        "TARGET".to_string()
    }

    fn impl_is_business_day(&self, date: &Date) -> bool {
        let w = date.weekday();
        let d = date.day();
        let dd = date.day_of_year();
        let m = date.month();
        let y = date.year();
        let em = self.easter_monday(y);
        if self.is_weekend(&w)
            || (d == 1 && m == 1)
            || (dd == em - 3 && y >= 2000)
            || (dd == em && y >= 2000)
            || (d == 1 && m == 5 && y >= 2000)
            || (d == 25 && m == 12)
            || (d == 26 && m == 12 && y >= 2000)
            || (d == 31 && m == 12 && (y == 1998 || y == 1999 || y == 2001))
        {
            return false;
        }
        return true;
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
            if self.is_business_day(&d) {
                business_days.push(d);
            }
            d = d + 1;
        }
        business_days
    }
}

impl IsCalendar for TARGET {}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::time::date::Date;
    use crate::time::enums::*;
    use crate::time::period::*;

    #[test]
    fn test_is_business_day() {
        let cal = TARGET::new();
        assert_eq!(cal.is_business_day(&Date::new(2021, 1, 1)), false);
        assert_eq!(cal.is_business_day(&Date::new(2021, 1, 2)), false);
        assert_eq!(cal.is_business_day(&Date::new(2021, 1, 3)), false);
        assert_eq!(cal.is_business_day(&Date::new(2021, 1, 4)), true);
        assert_eq!(cal.is_business_day(&Date::new(2021, 1, 5)), true);
        assert_eq!(cal.is_business_day(&Date::new(2021, 1, 6)), true);
        assert_eq!(cal.is_business_day(&Date::new(2021, 1, 7)), true);
        assert_eq!(cal.is_business_day(&Date::new(2021, 1, 8)), true);
        assert_eq!(cal.is_business_day(&Date::new(2021, 1, 9)), false);
    }

    #[test]
    fn test_advance_date() {
        let date = Date::new(2013, 3, 28);
        let cal = TARGET::new();
        let new_date = cal.advance(
            date,
            Period::new(1, TimeUnit::Years),
            Some(BusinessDayConvention::Unadjusted),
            true,
        );
        let tmpd = date+1;
        assert!(cal.is_business_day(&tmpd)==false);
        assert_eq!(cal.adjust(tmpd, None).month(), 4);

        assert_eq!(new_date, Date::new(2014, 3, 31));

        let new_date = cal.advance(
            date,
            Period::new(1, TimeUnit::Years),
            Some(BusinessDayConvention::Unadjusted),
            false,
        );

        assert_eq!(new_date, Date::new(2014, 3, 28));
    }
}
