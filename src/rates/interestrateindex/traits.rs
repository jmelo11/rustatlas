use std::collections::HashMap;

use crate::{
    rates::{
        traits::{HasReferenceDate, YieldProvider},
        yieldtermstructure::traits::{AdvanceInTimeError, YieldTermStructureTrait},
    },
    time::{date::Date, period::Period},
};

/// # FloatingRateProvider
/// Implement this trait for a struct that holds floating rate information.
pub trait FixingProvider {
    fn fixing(&self, date: Date) -> Option<f64>;
    fn fixings(&self) -> &HashMap<Date, f64>;
    fn add_fixing(&mut self, date: Date, rate: f64);
}

/// # InterestRateIndexClone
/// Trait for cloning a given object.
pub trait InterestRateIndexClone {
    fn clone_box(&self) -> Box<dyn InterestRateIndexTrait>;
}

/// # InterestRateIndexClone for T
impl<T: 'static + InterestRateIndexTrait + Clone> InterestRateIndexClone for T {
    fn clone_box(&self) -> Box<dyn InterestRateIndexTrait> {
        Box::new(self.clone())
    }
}

/// # Clone for Box<dyn InterestRateIndexTrait>
/// Implementation of Clone for Box<dyn InterestRateIndexTrait>.
impl Clone for Box<dyn InterestRateIndexTrait> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

/// # AdvanceInterestRateIndexInTime
/// Trait for advancing in time a given object. Returns a represation of the object
/// as it would be after the given period/time.
pub trait AdvanceInterestRateIndexInTime {
    fn advance_to_period(
        &self,
        period: Period,
    ) -> Result<Box<dyn InterestRateIndexTrait>, AdvanceInTimeError>;
    fn advance_to_date(
        &self,
        date: Date,
    ) -> Result<Box<dyn InterestRateIndexTrait>, AdvanceInTimeError>;
}

/// # HasTermStructure
/// Implement this trait for a struct that holds a term structure.
pub trait HasTermStructure {
    fn term_structure(&self) -> Option<&Box<dyn YieldTermStructureTrait>>;
}

pub trait InterestRateIndexTrait:
    FixingProvider
    + YieldProvider
    + HasReferenceDate
    + AdvanceInterestRateIndexInTime
    + InterestRateIndexClone
    + HasTermStructure
{
}
