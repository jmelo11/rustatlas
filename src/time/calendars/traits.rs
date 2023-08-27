use crate::{
    time::date::Date,
    time::enums::{BusinessDayConvention, TimeUnit, Weekday},
    time::period::Period,
};

use std::collections::HashSet;

pub trait ImplCalendar {
    fn impl_name(&self) -> String;

    fn added_holidays(&self) -> HashSet<Date>;

    fn impl_is_business_day(&self, date: &Date) -> bool;

    fn removed_holidays(&self) -> HashSet<Date>;

    fn add_holiday(&mut self, date: Date);

    fn remove_holiday(&mut self, date: Date);

    fn holiday_list(&self, from: Date, to: Date, include_weekends: bool) -> Vec<Date>;

    fn business_day_list(&self, from: Date, to: Date) -> Vec<Date>;

    fn is_weekend(&self, weekday: &Weekday) -> bool {
        weekday == &Weekday::Saturday || weekday == &Weekday::Sunday
    }
}

pub trait IsCalendar: ImplCalendar {
    fn name(&self) -> String {
        self.impl_name()
    }

    fn is_business_day(&self, date: &Date) -> bool {
        if !self.added_holidays().is_empty() && self.added_holidays().contains(date) {
            return false;
        }
        if !self.removed_holidays().is_empty() && self.removed_holidays().contains(date) {
            return true;
        }
        self.impl_is_business_day(date)
    }

    fn end_of_month(&self, date: Date) -> Date {
        self.adjust(
            Date::end_of_month(date),
            Some(BusinessDayConvention::Preceding),
        )
    }

    fn is_end_of_month(&self, date: &Date) -> bool {
        let d1 = self.adjust(*date + 1, None);
        d1.month() != date.month()
    }

    fn is_holiday(&self, date: &Date) -> bool {
        !self.is_business_day(date)
    }

    fn business_days_between(
        &self,
        from: Date,
        to: Date,
        include_first: bool,
        include_last: bool,
    ) -> i64 {
        if from < to {
            self.impl_days_between(from, to, include_first, include_last)
        } else if from > to {
            -self.impl_days_between(to, from, include_last, include_first)
        } else {
            if include_first && include_last && self.is_business_day(&from) {
                1
            } else {
                0
            }
        }
    }

    fn adjust(&self, date: Date, convention: Option<BusinessDayConvention>) -> Date {
        assert!(date != Date::empty(), "null date");

        let conv = match convention {
            Some(convention) => convention,
            None => BusinessDayConvention::Following,
        };

        let mut d1 = date;
        match conv {
            BusinessDayConvention::Unadjusted => return date,
            BusinessDayConvention::Following
            | BusinessDayConvention::ModifiedFollowing
            | BusinessDayConvention::HalfMonthModifiedFollowing => {
                while self.is_holiday(&d1) {
                    d1 += 1;
                }
                if let BusinessDayConvention::ModifiedFollowing
                | BusinessDayConvention::HalfMonthModifiedFollowing = conv
                {
                    if d1.month() != date.month() {
                        return self.adjust(date, Some(BusinessDayConvention::Preceding));
                    }
                    if let BusinessDayConvention::HalfMonthModifiedFollowing = conv {
                        if date.day() <= 15 && d1.day() > 15 {
                            return self.adjust(date, Some(BusinessDayConvention::Preceding));
                        }
                    }
                }
            }
            BusinessDayConvention::Preceding | BusinessDayConvention::ModifiedPreceding => {
                while self.is_holiday(&d1) {
                    d1 -= 1;
                }
                if let BusinessDayConvention::ModifiedPreceding = conv {
                    if d1.month() != date.month() {
                        return self.adjust(date, Some(BusinessDayConvention::Following));
                    }
                }
            }
            BusinessDayConvention::Nearest => {
                let mut d2 = date;
                while self.is_holiday(&d1) && self.is_holiday(&d2) {
                    d1 += 1;
                    d2 -= 1;
                }
                if self.is_holiday(&d1) {
                    return d2;
                } else {
                    return d1;
                }
            }
        }
        d1
    }

    fn impl_days_between(
        &self,
        from: Date,
        to: Date,
        include_first: bool,
        include_last: bool,
    ) -> i64 {
        let mut res = if include_last && self.is_business_day(&to) {
            1
        } else {
            0
        };
        let mut d = if include_first { from } else { from + 1 };
        while d < to {
            if self.is_business_day(&d) {
                res += 1;
            }
            d += 1;
        }
        res
    }

    fn advance(
        &self,
        date: Date,
        period: Period,
        convention: Option<BusinessDayConvention>,
        end_of_month: bool,
    ) -> Date {
        assert!(date != Date::empty(), "null date");

        let mut d1 = date;
        match period.units() {
            TimeUnit::Days => {
                let mut n = period.length();
                if n > 0 {
                    while n > 0 {
                        d1 += 1;
                        while self.is_holiday(&d1) {
                            d1 += 1;
                        }
                        n -= 1;
                    }
                } else {
                    while n < 0 {
                        d1 -= 1;
                        while self.is_holiday(&d1) {
                            d1 -= 1;
                        }
                        n += 1;
                    }
                }
            }
            TimeUnit::Weeks => {
                d1 = d1 + period;
                d1 = self.adjust(d1, convention);
            }
            _ => {
                d1 = d1 + period;
                if end_of_month && self.is_end_of_month(&date) {
                    d1 = Date::end_of_month(date);
                }
                d1 = self.adjust(d1, convention);
            }
        }
        d1
    }
}
