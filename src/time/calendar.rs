use serde::Serialize;

use super::calendars::{
    nullcalendar::NullCalendar,
    target::TARGET,
    traits::{ImplCalendar, IsCalendar},
    unitedstates::UnitedStates,
    weekendsonly::WeekendsOnly,
};
use crate::{
    time::date::Date,
    utils::errors::{AtlasError, Result},
};
use std::collections::HashSet;

/// # Calendar
/// A calendar.
///
/// ## Enums
/// * `NullCalendar` - A calendar that considers all days as business days.
/// * `WeekendsOnly` - A calendar that considers only weekends as business days.
/// * `TARGET` - A calendar that considers only TARGET business days as business days.
/// * `UnitedStates` - A calendar for the United States.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Calendar {
    NullCalendar(NullCalendar),
    WeekendsOnly(WeekendsOnly),
    TARGET(TARGET),
    UnitedStates(UnitedStates),
}

impl Serialize for Calendar {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = match self {
            Calendar::NullCalendar(cal) => cal.impl_name(),
            Calendar::WeekendsOnly(cal) => cal.impl_name(),
            Calendar::TARGET(cal) => cal.impl_name(),
            Calendar::UnitedStates(cal) => cal.impl_name(),
        };
        serializer.serialize_str(&s)
    }
}

impl<'de> serde::Deserialize<'de> for Calendar {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Calendar, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "NullCalendar" => Ok(Calendar::NullCalendar(NullCalendar::new())),
            "WeekendsOnly" => Ok(Calendar::WeekendsOnly(WeekendsOnly::new())),
            "TARGET" => Ok(Calendar::TARGET(TARGET::new())),
            "UnitedStates" => Ok(Calendar::UnitedStates(UnitedStates::default())),
            _ => Err(serde::de::Error::custom(format!("Invalid calendar: {}", s))),
        }
    }
}

impl TryFrom<String> for Calendar {
    type Error = AtlasError;

    fn try_from(s: String) -> Result<Self> {
        match s.as_str() {
            "NullCalendar" => Ok(Calendar::NullCalendar(NullCalendar::new())),
            "WeekendsOnly" => Ok(Calendar::WeekendsOnly(WeekendsOnly::new())),
            "TARGET" => Ok(Calendar::TARGET(TARGET::new())),
            "UnitedStates" => Ok(Calendar::UnitedStates(UnitedStates::default())),
            _ => Err(AtlasError::InvalidValueErr(format!(
                "Invalid calendar: {}",
                s
            ))),
        }
    }
}

impl From<Calendar> for String {
    fn from(calendar: Calendar) -> Self {
        match calendar {
            Calendar::NullCalendar(_) => "NullCalendar".to_string(),
            Calendar::WeekendsOnly(_) => "WeekendsOnly".to_string(),
            Calendar::TARGET(_) => "TARGET".to_string(),
            Calendar::UnitedStates(_) => "UnitedStates".to_string(),
        }
    }
}

impl ImplCalendar for Calendar {
    fn impl_name(&self) -> String {
        match self {
            Calendar::NullCalendar(cal) => cal.impl_name(),
            Calendar::WeekendsOnly(cal) => cal.impl_name(),
            Calendar::TARGET(cal) => cal.impl_name(),
            Calendar::UnitedStates(cal) => cal.impl_name(),
        }
    }

    fn impl_is_business_day(&self, date: &Date) -> bool {
        match self {
            Calendar::NullCalendar(cal) => cal.impl_is_business_day(date),
            Calendar::WeekendsOnly(cal) => cal.impl_is_business_day(date),
            Calendar::TARGET(cal) => cal.impl_is_business_day(date),
            Calendar::UnitedStates(cal) => cal.impl_is_business_day(date),
        }
    }

    fn added_holidays(&self) -> HashSet<Date> {
        match self {
            Calendar::NullCalendar(cal) => cal.added_holidays(),
            Calendar::WeekendsOnly(cal) => cal.added_holidays(),
            Calendar::TARGET(cal) => cal.added_holidays(),
            Calendar::UnitedStates(cal) => cal.added_holidays(),
        }
    }

    fn removed_holidays(&self) -> HashSet<Date> {
        match self {
            Calendar::NullCalendar(cal) => cal.removed_holidays(),
            Calendar::WeekendsOnly(cal) => cal.removed_holidays(),
            Calendar::TARGET(cal) => cal.removed_holidays(),
            Calendar::UnitedStates(cal) => cal.removed_holidays(),
        }
    }

    fn add_holiday(&mut self, date: Date) {
        match self {
            Calendar::NullCalendar(cal) => cal.add_holiday(date),
            Calendar::WeekendsOnly(cal) => cal.add_holiday(date),
            Calendar::TARGET(cal) => cal.add_holiday(date),
            Calendar::UnitedStates(cal) => cal.add_holiday(date),
        }
    }

    fn remove_holiday(&mut self, date: Date) {
        match self {
            Calendar::NullCalendar(cal) => cal.remove_holiday(date),
            Calendar::WeekendsOnly(cal) => cal.remove_holiday(date),
            Calendar::TARGET(cal) => cal.remove_holiday(date),
            Calendar::UnitedStates(cal) => cal.remove_holiday(date),
        }
    }

    fn holiday_list(&self, from: Date, to: Date, include_weekends: bool) -> Vec<Date> {
        match self {
            Calendar::NullCalendar(cal) => cal.holiday_list(from, to, include_weekends),
            Calendar::WeekendsOnly(cal) => cal.holiday_list(from, to, include_weekends),
            Calendar::TARGET(cal) => cal.holiday_list(from, to, include_weekends),
            Calendar::UnitedStates(cal) => cal.holiday_list(from, to, include_weekends),
        }
    }

    fn business_day_list(&self, from: Date, to: Date) -> Vec<Date> {
        match self {
            Calendar::NullCalendar(cal) => cal.business_day_list(from, to),
            Calendar::WeekendsOnly(cal) => cal.business_day_list(from, to),
            Calendar::TARGET(cal) => cal.business_day_list(from, to),
            Calendar::UnitedStates(cal) => cal.business_day_list(from, to),
        }
    }
}

impl IsCalendar for Calendar {}
