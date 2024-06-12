use crate::utils::errors::{AtlasError, Result};

use super::calendar::*;
use super::calendars::nullcalendar::NullCalendar;
use super::calendars::traits::*;
use super::date::Date;
use super::enums::*;
use super::imm::IMM;
use super::period::Period;

fn next_twentieth(date: Date, rule: DateGenerationRule) -> Date {
    let mut result = Date::new(date.year(), date.month(), 20);
    if result < date {
        result = result + Period::new(1, TimeUnit::Months);
    }
    if rule == DateGenerationRule::TwentiethIMM
        || rule == DateGenerationRule::OldCDS
        || rule == DateGenerationRule::CDS
        || rule == DateGenerationRule::CDS2015
    {
        let m = result.month();
        if m % 3 != 0 {
            let skip = 3 - m % 3;
            result = result + Period::new(skip as i32, TimeUnit::Months);
        }
    }
    return result;
}

fn previous_twentieth(date: Date, rule: DateGenerationRule) -> Date {
    let mut result = Date::new(date.year(), date.month(), 20);
    if result > date {
        result = result - Period::new(1, TimeUnit::Months);
    }
    if rule == DateGenerationRule::TwentiethIMM
        || rule == DateGenerationRule::OldCDS
        || rule == DateGenerationRule::CDS
        || rule == DateGenerationRule::CDS2015
    {
        let m = result.month();
        if m % 3 != 0 {
            let skip = 3 - m % 3;
            result = result - Period::new(skip as i32, TimeUnit::Months);
        }
    }
    return result;
}

/// # Schedule
/// A schedule is a sequence of dates. It is defined by an effective date, a termination date and
/// a tenor.
///
/// ## Parameters
/// * `tenor` - The tenor of the schedule
/// * `calendar` - The calendar of the schedule
/// * `convention` - The business day convention of the schedule
/// * `termination_date_convention` - The business day convention of the termination date
/// * `rule` - The date generation rule
/// * `end_of_month` - The end of month flag
/// * `first_date` - The first date of the schedule
/// * `next_to_last_date` - The next to last date of the schedule
/// * `dates` - The dates of the schedule
/// * `is_regular` - The regularity of the schedule
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Schedule {
    tenor: Period,
    calendar: Calendar,
    convention: BusinessDayConvention,
    termination_date_convention: BusinessDayConvention,
    rule: DateGenerationRule,
    end_of_month: bool,
    first_date: Date,
    next_to_last_date: Date,
    dates: Vec<Date>,
    is_regular: Vec<bool>,
}

impl Schedule {
    pub fn new(
        tenor: Period,
        calendar: Calendar,
        convention: BusinessDayConvention,
        termination_date_convention: BusinessDayConvention,
        rule: DateGenerationRule,
        end_of_month: bool,
        first_date: Date,
        next_to_last_date: Date,
        dates: Vec<Date>,
        is_regular: Vec<bool>,
    ) -> Schedule {
        Schedule {
            tenor,
            calendar,
            convention,
            termination_date_convention,
            rule,
            end_of_month,
            first_date,
            next_to_last_date,
            dates,
            is_regular,
        }
    }

    pub fn empty() -> Schedule {
        Schedule {
            tenor: Period::empty(),
            calendar: Calendar::NullCalendar(NullCalendar::new()),
            convention: BusinessDayConvention::Unadjusted,
            termination_date_convention: BusinessDayConvention::Unadjusted,
            rule: DateGenerationRule::Backward,
            end_of_month: false,
            first_date: Date::empty(),
            next_to_last_date: Date::empty(),
            dates: Vec::new(),
            is_regular: Vec::new(),
        }
    }

    pub fn dates(&self) -> &Vec<Date> {
        return &self.dates;
    }

    pub fn is_regular(&self) -> &Vec<bool> {
        return &self.is_regular;
    }

    pub fn tenor(&self) -> Period {
        return self.tenor;
    }

    pub fn calendar(&self) -> Calendar {
        return self.calendar.clone();
    }

    pub fn convention(&self) -> BusinessDayConvention {
        return self.convention;
    }

    pub fn termination_date_convention(&self) -> BusinessDayConvention {
        return self.termination_date_convention;
    }

    pub fn rule(&self) -> DateGenerationRule {
        return self.rule;
    }

    pub fn end_of_month(&self) -> bool {
        return self.end_of_month;
    }

    pub fn first_date(&self) -> Date {
        return self.first_date;
    }

    pub fn next_to_last_date(&self) -> Date {
        return self.next_to_last_date;
    }
}

/// # MakeSchedule
/// This struct is used to build a schedule.
///
/// ## Example
///
/// ```
/// use rustatlas::prelude::*;
///
/// let from = Date::new(2022, 1, 1);
/// let to = Date::new(2022, 6, 1);
/// let tenor = Period::new(1, TimeUnit::Months);
///
/// let schedule = MakeSchedule::new(from, to).with_tenor(tenor).build().unwrap();
///
/// let dates = vec![
///    Date::new(2022, 1, 1),
///    Date::new(2022, 2, 1),
///    Date::new(2022, 3, 1),
///    Date::new(2022, 4, 1),
///    Date::new(2022, 5, 1),
///    Date::new(2022, 6, 1),
/// ];
///
/// assert_eq!(schedule.dates(), &dates);
/// ```
pub struct MakeSchedule {
    effective_date: Date,
    termination_date: Date,
    tenor: Period,
    calendar: Calendar,
    convention: BusinessDayConvention,
    termination_date_convention: BusinessDayConvention,
    rule: DateGenerationRule,
    end_of_month: bool,
    first_date: Date,
    next_to_last_date: Date,
    is_regular: Vec<bool>,
    dates: Vec<Date>,
}

/// Constructor, setters and getters
impl MakeSchedule {
    /// Returns a new instance of MakeSchedule.
    pub fn new(from: Date, to: Date) -> MakeSchedule {
        MakeSchedule {
            effective_date: from,
            termination_date: to,
            tenor: Period::empty(),
            calendar: Calendar::NullCalendar(NullCalendar::new()),
            convention: BusinessDayConvention::Unadjusted,
            termination_date_convention: BusinessDayConvention::Unadjusted,
            rule: DateGenerationRule::Backward,
            end_of_month: false,
            first_date: Date::empty(),
            next_to_last_date: Date::empty(),
            dates: Vec::new(),
            is_regular: Vec::new(),
        }
    }

    /// Sets the tenor.
    pub fn with_tenor(mut self, tenor: Period) -> MakeSchedule {
        self.tenor = tenor;
        self
    }

    /// Sets the frequency.
    pub fn with_frequency(mut self, frequency: Frequency) -> MakeSchedule {
        self.tenor = Period::from_frequency(frequency).expect("Invalid frequency");
        self
    }

    /// Sets the calendar.
    pub fn with_calendar(mut self, calendar: Calendar) -> MakeSchedule {
        self.calendar = calendar;
        self
    }

    /// Sets the convention. weekday correccions are applied.
    pub fn with_convention(mut self, convention: BusinessDayConvention) -> MakeSchedule {
        self.convention = convention;
        self
    }

    /// Sets the termination date convention.
    pub fn with_termination_date_convention(
        mut self,
        termination_date_convention: BusinessDayConvention,
    ) -> MakeSchedule {
        self.termination_date_convention = termination_date_convention;
        return self;
    }

    /// Sets the rule.
    pub fn with_rule(mut self, rule: DateGenerationRule) -> MakeSchedule {
        self.rule = rule;
        return self;
    }

    /// Sets the end of month flag.
    pub fn forwards(mut self) -> MakeSchedule {
        self.rule = DateGenerationRule::Forward;
        return self;
    }

    /// Sets the date generation rule to backward.
    pub fn backwards(mut self) -> MakeSchedule {
        self.rule = DateGenerationRule::Backward;
        return self;
    }

    /// Sets the end of month flag.
    pub fn end_of_month(mut self, flag: bool) -> MakeSchedule {
        self.end_of_month = flag;
        return self;
    }

    /// Sets the first date.
    pub fn with_first_date(mut self, first_date: Date) -> MakeSchedule {
        self.first_date = first_date;
        return self;
    }

    /// Sets the next to last date.
    pub fn with_next_to_last_date(mut self, next_to_last_date: Date) -> MakeSchedule {
        self.next_to_last_date = next_to_last_date;
        return self;
    }
}

/// Build method
impl MakeSchedule {
    pub fn build(&mut self) -> Result<Schedule> {
        if self.tenor.length() < 0 {
            return Err(AtlasError::MakeScheduleErr(format!(
                "non positive tenor ({})",
                self.tenor.length()
            )));
        }
        if self.tenor.length() == 0 {
            self.rule = DateGenerationRule::Zero;
        }

        if self.first_date != Date::empty() {
            match self.rule {
                DateGenerationRule::Backward | DateGenerationRule::Forward => {
                    if self.first_date <= self.effective_date
                        || self.first_date > self.termination_date
                    {
                        return Err(AtlasError::MakeScheduleErr(
                            "first date out of effective-termination date range".to_string(),
                        ));
                    }
                }
                DateGenerationRule::ThirdWednesday => {
                    if !IMM::is_imm_date(self.first_date, false) {
                        return Err(AtlasError::MakeScheduleErr(
                            "first date is not an IMM date".to_string(),
                        ));
                    }
                }
                DateGenerationRule::Zero
                | DateGenerationRule::Twentieth
                | DateGenerationRule::TwentiethIMM
                | DateGenerationRule::OldCDS
                | DateGenerationRule::CDS
                | DateGenerationRule::CDS2015 => {
                    return Err(AtlasError::MakeScheduleErr(
                        "first date incompatible with date generation rule".to_string(),
                    ));
                }
                _ => {
                    return Err(AtlasError::MakeScheduleErr("unknown rule".to_string()));
                }
            }
        }

        if self.next_to_last_date != Date::empty() {
            match self.rule {
                DateGenerationRule::Backward | DateGenerationRule::Forward => {
                    if self.next_to_last_date <= self.effective_date
                        || self.next_to_last_date >= self.termination_date
                    {
                        return Err(AtlasError::MakeScheduleErr(
                            "next to last date out of effective-termination date range".to_string(),
                        ));
                    }
                }
                DateGenerationRule::ThirdWednesday => {
                    if !IMM::is_imm_date(self.next_to_last_date, false) {
                        return Err(AtlasError::MakeScheduleErr(
                            "next to last date is not an IMM date".to_string(),
                        ));
                    }
                }
                DateGenerationRule::Zero
                | DateGenerationRule::Twentieth
                | DateGenerationRule::TwentiethIMM
                | DateGenerationRule::OldCDS
                | DateGenerationRule::CDS
                | DateGenerationRule::CDS2015 => {
                    return Err(AtlasError::MakeScheduleErr(
                        "next to last date incompatible with date generation rule".to_string(),
                    ));
                }
                _ => {
                    return Err(AtlasError::MakeScheduleErr("unknown rule".to_string()));
                }
            }
        }

        let null_calendar = Calendar::NullCalendar(NullCalendar::new());
        let mut periods = 1;
        let mut seed = Date::empty();

        match self.rule {
            DateGenerationRule::Zero => {
                self.tenor = Period::new(0, TimeUnit::Years);
                self.dates.push(self.effective_date);
                self.dates.push(self.termination_date);
                self.is_regular.push(true);
            }
            DateGenerationRule::Backward => {
                self.dates.push(self.termination_date);

                seed = self.termination_date;
                if self.next_to_last_date != Date::empty() {
                    self.dates.insert(0, self.next_to_last_date);
                    let temp = null_calendar.advance(
                        seed,
                        self.tenor * -periods,
                        Some(self.convention),
                        self.end_of_month,
                    );
                    if temp != self.next_to_last_date {
                        self.is_regular.insert(0, false);
                    } else {
                        self.is_regular.insert(0, true);
                    }
                    seed = self.next_to_last_date;
                }

                let mut exit_date = self.effective_date;
                if self.first_date != Date::empty() {
                    exit_date = self.first_date;
                }

                loop {
                    let temp = null_calendar.advance(
                        seed,
                        self.tenor * -periods,
                        Some(self.convention),
                        self.end_of_month,
                    );
                    if temp < exit_date {
                        if self.first_date != Date::empty()
                            && (self.calendar.adjust(self.dates[0], Some(self.convention))
                                != self.calendar.adjust(self.first_date, Some(self.convention)))
                        {
                            self.dates.insert(0, self.first_date);
                            self.is_regular.insert(0, false);
                        }
                        break;
                    } else {
                        // skip dates that would result in duplicates
                        // after adjustment
                        if self.calendar.adjust(self.dates[0], Some(self.convention))
                            != self.calendar.adjust(temp, Some(self.convention))
                        {
                            self.dates.insert(0, temp);
                            self.is_regular.insert(0, true);
                        }
                        periods += 1;
                    }
                }

                if self.calendar.adjust(self.dates[0], Some(self.convention))
                    != self
                        .calendar
                        .adjust(self.effective_date, Some(self.convention))
                {
                    self.dates.insert(0, self.effective_date);
                    self.is_regular.insert(0, false);
                }
            }
            DateGenerationRule::Twentieth
            | DateGenerationRule::TwentiethIMM
            | DateGenerationRule::ThirdWednesday
            | DateGenerationRule::ThirdWednesdayInclusive
            | DateGenerationRule::OldCDS
            | DateGenerationRule::CDS
            | DateGenerationRule::CDS2015
            | DateGenerationRule::Forward => {
                if self.rule != DateGenerationRule::Forward {
                    // assert!(
                    //     self.end_of_month == false,
                    //     "endOfMonth convention incompatible with {:?} date generation rule",
                    //     self.rule
                    // );
                    if self.end_of_month == true {
                        //panic!("endOfMonth convention incompatible with {:?} date generation rule", self.rule);
                        return Err(AtlasError::MakeScheduleErr(
                            "endOfMonth convention incompatible with date generation rule"
                                .to_string(),
                        ));
                    }
                }

                if self.rule == DateGenerationRule::CDS || self.rule == DateGenerationRule::CDS2015
                {
                    let prev20th = previous_twentieth(self.effective_date, self.rule);
                    if self.calendar.adjust(prev20th, Some(self.convention)) > self.effective_date {
                        self.dates.push(prev20th - Period::new(3, TimeUnit::Months));
                        self.is_regular.push(true);
                    }
                    self.dates.push(prev20th);
                } else {
                    self.dates.push(self.effective_date);
                }

                seed = self.dates.last().unwrap().clone();

                if self.first_date != Date::empty() {
                    self.dates.push(self.first_date);
                    let temp = self.calendar.advance(
                        seed,
                        self.tenor * periods,
                        Some(self.convention),
                        self.end_of_month,
                    );
                    if temp != self.first_date {
                        self.is_regular.push(false);
                    } else {
                        self.is_regular.push(true);
                    }
                    seed = self.first_date;
                } else if self.rule == DateGenerationRule::Twentieth
                    || self.rule == DateGenerationRule::TwentiethIMM
                    || self.rule == DateGenerationRule::OldCDS
                    || self.rule == DateGenerationRule::CDS
                    || self.rule == DateGenerationRule::CDS2015
                {
                    let mut next20th = next_twentieth(self.effective_date, self.rule);
                    if self.rule == DateGenerationRule::OldCDS {
                        // distance rule inforced in natural days
                        let stub_days = 30;
                        if next20th - self.effective_date < stub_days {
                            // +1 will skip this one and get the next
                            next20th = next_twentieth(next20th + 1, self.rule);
                        }
                    }
                    if next20th != self.effective_date {
                        self.dates.push(next20th);
                        self.is_regular.push(
                            self.rule == DateGenerationRule::CDS
                                || self.rule == DateGenerationRule::CDS2015,
                        );
                        seed = next20th;
                    }
                }

                let mut exit_date = self.termination_date;

                if self.next_to_last_date != Date::empty() {
                    exit_date = self.next_to_last_date;
                }

                loop {
                    let temp = null_calendar.advance(
                        seed,
                        self.tenor * periods,
                        Some(self.convention),
                        self.end_of_month,
                    );
                    if temp > exit_date {
                        if self.next_to_last_date != Date::empty()
                            && (self
                                .calendar
                                .adjust(self.dates.last().unwrap().clone(), Some(self.convention))
                                != self
                                    .calendar
                                    .adjust(self.next_to_last_date, Some(self.convention)))
                        {
                            self.dates.push(self.next_to_last_date);
                            self.is_regular.push(false);
                        }
                        break;
                    } else {
                        // skip dates that would result in duplicates
                        // after adjustment
                        if self
                            .calendar
                            .adjust(self.dates.last().unwrap().clone(), Some(self.convention))
                            != self.calendar.adjust(temp, Some(self.convention))
                        {
                            self.dates.push(temp);
                            self.is_regular.push(true);
                        }
                        periods += 1;
                    }
                }

                if self.calendar.adjust(
                    self.dates.last().unwrap().clone(),
                    Some(self.termination_date_convention),
                ) != self.calendar.adjust(
                    self.termination_date,
                    Some(self.termination_date_convention),
                ) {
                    if self.rule == DateGenerationRule::Twentieth
                        || self.rule == DateGenerationRule::TwentiethIMM
                        || self.rule == DateGenerationRule::OldCDS
                        || self.rule == DateGenerationRule::CDS
                        || self.rule == DateGenerationRule::CDS2015
                    {
                        self.dates
                            .push(next_twentieth(self.termination_date, self.rule));
                        self.is_regular.push(true);
                    } else {
                        self.dates.push(self.termination_date);
                        self.is_regular.push(false);
                    }
                }
            }
        }

        if self.rule == DateGenerationRule::ThirdWednesday {
            for i in 1..self.dates.len() - 1 {
                self.dates[i] = Date::nth_weekday(
                    3,
                    Weekday::Wednesday,
                    self.dates[i].month(),
                    self.dates[i].year(),
                );
            }
        } else if self.rule == DateGenerationRule::ThirdWednesdayInclusive {
            for date in self.dates.iter_mut() {
                *date = Date::nth_weekday(3, Weekday::Wednesday, date.month(), date.year());
            }
        }

        if self.convention != BusinessDayConvention::Unadjusted
            && self.rule != DateGenerationRule::OldCDS
        {
            self.dates[0] = self.calendar.adjust(self.dates[0], Some(self.convention));
        }

        if self.termination_date_convention != BusinessDayConvention::Unadjusted
            && self.rule != DateGenerationRule::CDS
            && self.rule != DateGenerationRule::CDS2015
        {
            let len = self.dates.len();
            self.dates[len - 1] = self
                .calendar
                .adjust(self.dates[len - 1], Some(self.termination_date_convention));
        }

        if self.end_of_month && self.calendar.is_end_of_month(&seed) {
            if self.convention == BusinessDayConvention::Unadjusted {
                for i in 1..self.dates.len() - 1 {
                    self.dates[i] = Date::end_of_month(self.dates[i]);
                }
            } else {
                for i in 1..self.dates.len() - 1 {
                    self.dates[i] = self.calendar.end_of_month(self.dates[i]);
                }
            }
        } else {
            for i in 1..self.dates.len() - 1 {
                self.dates[i] = self.calendar.adjust(self.dates[i], Some(self.convention));
            }
        }

        if self.dates.len() >= 2
            && self.dates[self.dates.len() - 2] >= self.dates.last().unwrap().clone()
        {
            let is_regular_len = self.is_regular.len();
            let dates_len = self.dates.len();
            if self.is_regular.len() >= 2 {
                self.is_regular[is_regular_len - 2] =
                    self.dates[dates_len - 2] == self.dates.last().unwrap().clone();
            }
            self.dates[dates_len - 2] = self.dates.last().unwrap().clone();
            self.dates.pop();
            self.is_regular.pop();
        }

        if self.dates.len() >= 2 && self.dates[1] <= self.dates[0] {
            self.is_regular[1] = self.dates[1] == self.dates[0];
            self.dates[1] = self.dates[0];
            self.dates.remove(0);
            self.is_regular.remove(0);
        }

        return Ok(Schedule::new(
            self.tenor,
            self.calendar.clone(),
            self.convention,
            self.termination_date_convention,
            self.rule,
            self.end_of_month,
            self.first_date,
            self.next_to_last_date,
            self.dates.clone(),
            self.is_regular.clone(),
        ));
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::time::calendars::target::TARGET;

    use super::*;

    fn allows_end_of_month(period: Period) -> bool {
        return period.units() == TimeUnit::Months
            || period.units() == TimeUnit::Years && period >= Period::new(1, TimeUnit::Months);
    }

    #[test]
    fn test_next_twentieth() {
        let date = Date::new(2022, 1, 1);
        let rule = DateGenerationRule::Twentieth;
        let result = next_twentieth(date, rule);
        assert_eq!(result, Date::new(2022, 1, 20));

        let date = Date::new(2022, 1, 1);
        let rule = DateGenerationRule::TwentiethIMM;
        let result = next_twentieth(date, rule);
        assert_eq!(result, Date::new(2022, 3, 20));
    }

    #[test]
    fn test_allows_end_of_month() {
        let period = Period::new(1, TimeUnit::Months);
        let result = allows_end_of_month(period);
        assert_eq!(result, true);

        let period = Period::new(1, TimeUnit::Years);
        let result = allows_end_of_month(period);
        assert_eq!(result, true);

        let period = Period::new(1, TimeUnit::Days);
        let result = allows_end_of_month(period);
        assert_eq!(result, false);
    }

    #[test]
    fn test_previous_twentieth() {
        let date = Date::new(2022, 1, 1);
        let rule = DateGenerationRule::Twentieth;
        let result = previous_twentieth(date, rule);
        assert_eq!(result, Date::new(2021, 12, 20));

        let date = Date::new(2022, 1, 1);
        let rule = DateGenerationRule::TwentiethIMM;
        let result = previous_twentieth(date, rule);
        assert_eq!(result, Date::new(2021, 12, 20));
    }

    #[test]
    fn test_make_schedule_new() {
        let from = Date::new(2022, 1, 1);
        let to = Date::new(2022, 3, 1);
        let tenor = Period::new(1, TimeUnit::Months);
        let make_schedule = MakeSchedule::new(from, to).with_tenor(tenor);
        assert_eq!(make_schedule.effective_date, from);
        assert_eq!(make_schedule.termination_date, to);
        assert_eq!(make_schedule.tenor, tenor);
        assert_eq!(
            make_schedule.calendar,
            Calendar::NullCalendar(NullCalendar::new())
        );
        assert_eq!(make_schedule.convention, BusinessDayConvention::Unadjusted);
        assert_eq!(
            make_schedule.termination_date_convention,
            BusinessDayConvention::Unadjusted
        );
        assert_eq!(make_schedule.rule, DateGenerationRule::Backward);
        assert_eq!(make_schedule.end_of_month, false);
        assert_eq!(make_schedule.first_date, Date::empty());
        assert_eq!(make_schedule.next_to_last_date, Date::empty());
        assert_eq!(make_schedule.dates, Vec::new());
    }

    #[test]
    fn test_make_schedule_with_frequency() {
        let from = Date::new(2022, 1, 1);
        let to = Date::new(2022, 3, 1);
        let tenor = Period::new(1, TimeUnit::Months);
        let frequency = Frequency::Semiannual;
        let make_schedule = MakeSchedule::new(from, to)
            .with_tenor(tenor)
            .with_frequency(frequency);
        assert_eq!(make_schedule.tenor, Period::new(6, TimeUnit::Months));
    }

    #[test]
    fn test_make_schedule_with_calendar() {
        let from = Date::new(2022, 1, 1);
        let to = Date::new(2022, 3, 1);
        let tenor = Period::new(1, TimeUnit::Months);
        let calendar = Calendar::NullCalendar(NullCalendar::new());
        let make_schedule = MakeSchedule::new(from, to)
            .with_tenor(tenor)
            .with_calendar(calendar);
        assert_eq!(
            make_schedule.calendar,
            Calendar::NullCalendar(NullCalendar::new())
        );
    }

    #[test]
    fn test_make_schedule_with_convention() {
        let from = Date::new(2022, 1, 1);
        let to = Date::new(2022, 3, 1);
        let tenor = Period::new(1, TimeUnit::Months);
        let convention = BusinessDayConvention::Unadjusted;
        let make_schedule = MakeSchedule::new(from, to)
            .with_tenor(tenor)
            .with_convention(convention);
        assert_eq!(make_schedule.convention, BusinessDayConvention::Unadjusted);
    }

    #[test]
    fn test_make_schedule_with_termination_date_convention() {
        let from = Date::new(2022, 1, 1);
        let to = Date::new(2022, 3, 1);
        let tenor = Period::new(1, TimeUnit::Months);
        let termination_date_convention = BusinessDayConvention::Unadjusted;
        let make_schedule = MakeSchedule::new(from, to)
            .with_tenor(tenor)
            .with_termination_date_convention(termination_date_convention);
        assert_eq!(
            make_schedule.termination_date_convention,
            BusinessDayConvention::Unadjusted
        );
    }

    #[test]
    fn test_make_schedule_with_rule() {
        let from = Date::new(2022, 1, 1);
        let to = Date::new(2022, 3, 1);
        let tenor = Period::new(1, TimeUnit::Months);
        let rule = DateGenerationRule::Backward;
        let make_schedule = MakeSchedule::new(from, to)
            .with_tenor(tenor)
            .with_rule(rule);
        assert_eq!(make_schedule.rule, DateGenerationRule::Backward);
    }

    #[test]
    fn test_make_schedule_forwards() {
        let from = Date::new(2022, 1, 1);
        let to = Date::new(2022, 3, 1);
        let tenor = Period::new(1, TimeUnit::Months);
        let make_schedule = MakeSchedule::new(from, to).with_tenor(tenor).forwards();
        assert_eq!(make_schedule.rule, DateGenerationRule::Forward);
    }

    #[test]
    fn test_make_schedule_backwards() {
        let from = Date::new(2022, 1, 1);
        let to = Date::new(2022, 3, 1);
        let tenor = Period::new(1, TimeUnit::Months);
        let make_schedule = MakeSchedule::new(from, to).with_tenor(tenor).backwards();
        assert_eq!(make_schedule.rule, DateGenerationRule::Backward);
    }

    #[test]
    fn test_make_schedule_end_of_month() {
        let from = Date::new(2022, 1, 1);
        let to = Date::new(2022, 3, 1);
        let tenor = Period::new(1, TimeUnit::Months);
        let make_schedule = MakeSchedule::new(from, to)
            .with_tenor(tenor)
            .end_of_month(true);
        assert_eq!(make_schedule.end_of_month, true);
    }

    #[test]
    fn test_make_schedule_with_first_date() {
        let from = Date::new(2022, 1, 1);
        let to = Date::new(2023, 3, 1);
        let tenor = Period::new(1, TimeUnit::Months);
        let first_date = Date::new(2022, 2, 1);
        let make_schedule = MakeSchedule::new(from, to)
            .with_tenor(tenor)
            .with_first_date(first_date);
        assert_eq!(make_schedule.first_date, Date::new(2022, 2, 1));
    }

    #[test]
    fn test_make_schedule_with_next_to_last_date() {
        let from = Date::new(2022, 1, 1);
        let to = Date::new(2023, 3, 1);
        let tenor = Period::new(1, TimeUnit::Months);
        let next_to_last_date = Date::new(2023, 2, 1);
        let make_schedule = MakeSchedule::new(from, to)
            .with_tenor(tenor)
            .with_next_to_last_date(next_to_last_date);
        assert_eq!(make_schedule.next_to_last_date, Date::new(2023, 2, 1));
    }

    #[test]
    fn test_make_simple_schedule_build() -> Result<()> {
        let from = Date::new(2022, 1, 1);
        let to = Date::new(2023, 3, 1);
        // monthly
        let tenor = Period::new(1, TimeUnit::Months);
        let schedule = MakeSchedule::new(from, to).with_tenor(tenor).build()?;

        let dates = vec![
            Date::new(2022, 1, 1),
            Date::new(2022, 2, 1),
            Date::new(2022, 3, 1),
            Date::new(2022, 4, 1),
            Date::new(2022, 5, 1),
            Date::new(2022, 6, 1),
            Date::new(2022, 7, 1),
            Date::new(2022, 8, 1),
            Date::new(2022, 9, 1),
            Date::new(2022, 10, 1),
            Date::new(2022, 11, 1),
            Date::new(2022, 12, 1),
            Date::new(2023, 1, 1),
            Date::new(2023, 2, 1),
            Date::new(2023, 3, 1),
        ];
        assert_eq!(schedule.dates, dates);

        // quarterly
        let tenor = Period::new(3, TimeUnit::Months);
        let schedule = MakeSchedule::new(from, to).with_tenor(tenor).build()?;

        let dates = vec![
            Date::new(2022, 1, 1),
            Date::new(2022, 3, 1),
            Date::new(2022, 6, 1),
            Date::new(2022, 9, 1),
            Date::new(2022, 12, 1),
            Date::new(2023, 3, 1),
        ];
        assert_eq!(schedule.dates, dates);

        // semiannual
        let tenor = Period::new(6, TimeUnit::Months);
        let schedule = MakeSchedule::new(from, to).with_tenor(tenor).build()?;

        let dates = vec![
            Date::new(2022, 1, 1),
            Date::new(2022, 3, 1),
            Date::new(2022, 9, 1),
            Date::new(2023, 3, 1),
        ];

        assert_eq!(schedule.dates, dates);

        // annual
        let tenor = Period::new(1, TimeUnit::Years);
        let schedule = MakeSchedule::new(from, to).with_tenor(tenor).build()?;

        let dates = vec![
            Date::new(2022, 1, 1),
            Date::new(2022, 3, 1),
            Date::new(2023, 3, 1),
        ];

        assert_eq!(schedule.dates, dates);

        Ok(())
    }

    #[test]
    fn test_daily_schedule() -> Result<()> {
        let from = Date::new(2012, 1, 17);
        let to = Date::new(2012, 1, 24);
        let tenor = Period::new(1, TimeUnit::Days);

        let schedule = MakeSchedule::new(from, to)
            .with_tenor(tenor)
            .with_calendar(Calendar::TARGET(TARGET::new()))
            .with_convention(BusinessDayConvention::Preceding)
            .build()?;

        let expected = vec![
            Date::new(2012, 1, 17),
            Date::new(2012, 1, 18),
            Date::new(2012, 1, 19),
            Date::new(2012, 1, 20),
            Date::new(2012, 1, 23),
            Date::new(2012, 1, 24),
        ];

        assert_eq!(schedule.dates, expected);

        Ok(())
    }

    #[test]
    fn test_make_schedule_with_end_of_month() {
        let from = Date::new(2022, 1, 31);
        let to = Date::new(2022, 3, 31);
        let tenor = Period::new(1, TimeUnit::Months);
        let make_schedule = MakeSchedule::new(from, to)
            .with_tenor(tenor)
            .end_of_month(true);
        assert_eq!(make_schedule.end_of_month, true);
    }

    #[test]
    fn test_dates_past_end_date_with_eom_adjustment() -> Result<()> {
        let from = Date::new(2013, 3, 28);
        let to = Date::new(2015, 3, 30);
        let tenor = Period::new(1, TimeUnit::Years);
        let calendar = Calendar::TARGET(TARGET::new());
        let convention = BusinessDayConvention::Unadjusted;
        let termination_date_convention = BusinessDayConvention::Unadjusted;
        let end_of_month = true;
        let schedule = MakeSchedule::new(from, to)
            .with_tenor(tenor)
            .with_calendar(calendar)
            .with_convention(convention)
            .with_termination_date_convention(termination_date_convention)
            .forwards()
            .end_of_month(end_of_month)
            .build()?;

        let expected = vec![
            Date::new(2013, 3, 28),
            Date::new(2014, 3, 31),
            Date::new(2015, 3, 30),
        ];

        assert_eq!(schedule.dates, expected);

        Ok(())
    }

    #[test]
    fn test_dates_same_as_end_date_with_eom_adjustment() -> Result<()> {
        let from = Date::new(2013, 3, 28);
        let to = Date::new(2015, 3, 31);
        let tenor = Period::new(1, TimeUnit::Years);
        let calendar = Calendar::TARGET(TARGET::new());
        let convention = BusinessDayConvention::Unadjusted;
        let termination_date_convention = BusinessDayConvention::Unadjusted;
        let end_of_month = true;
        let schedule = MakeSchedule::new(from, to)
            .with_tenor(tenor)
            .with_calendar(calendar)
            .with_convention(convention)
            .with_termination_date_convention(termination_date_convention)
            .forwards()
            .end_of_month(end_of_month)
            .build()?;

        let expected = vec![
            Date::new(2013, 3, 28),
            Date::new(2014, 3, 31),
            Date::new(2015, 3, 31),
        ];

        assert_eq!(schedule.dates, expected);

        Ok(())
    }

    #[test]
    fn test_schedule_with_first_date() {
        let from = Date::new(2022, 1, 1);
        let to = Date::new(2024, 3, 1);
        let tenor = Period::new(1, TimeUnit::Months);
        let first_date = Date::new(2022, 4, 1);
        let schedule = MakeSchedule::new(from, to)
            .with_tenor(tenor)
            .with_first_date(first_date)
            .build()
            .unwrap();
        let dates = schedule.dates();   
        assert_eq!(dates[0], from);
        assert_eq!(dates[1], first_date);        
    }
}
