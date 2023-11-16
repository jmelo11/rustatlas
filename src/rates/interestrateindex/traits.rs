use std::collections::{BTreeMap, HashMap};

use crate::{
    math::interpolation::enums::Interpolator,
    rates::{
        traits::{HasReferenceDate, YieldProvider},
        yieldtermstructure::traits::YieldTermStructureTrait,
    },
    time::{date::Date, enums::TimeUnit, period::Period},
    utils::errors::Result,
};

/// # FloatingRateProvider
/// Implement this trait for a struct that holds floating rate information.
/// This trait is implemented by IborIndex and OvernightIndex.
pub trait FixingProvider {
    fn fixing(&self, date: Date) -> Result<f64>;
    fn fixings(&self) -> &HashMap<Date, f64>;
    fn add_fixing(&mut self, date: Date, rate: f64);
    fn fill_missing_fixings(&mut self, interpolator: Interpolator) {
        if !self.fixings().is_empty() {
            let first_date = self.fixings().keys().min().unwrap().clone();
            let last_date = self.fixings().keys().max().unwrap().clone();

            let aux_btreemap = self
                .fixings()
                .iter()
                .map(|(k, v)| (*k, *v))
                .collect::<BTreeMap<Date, f64>>();

            let x = aux_btreemap
                .keys()
                .map(|&d| (d - first_date) as f64)
                .collect::<Vec<f64>>();

            let y = aux_btreemap.values().map(|r| *r).collect::<Vec<f64>>();

            let mut current_date = first_date.clone();

            while current_date <= last_date {
                if !self.fixings().contains_key(&current_date) {
                    let days = (current_date - first_date) as f64;
                    let rate = interpolator.interpolate(days, &x, &y, false);
                    self.add_fixing(current_date, rate);
                }
                current_date = current_date + Period::new(1, TimeUnit::Days);
            }
        }
    }
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
    fn advance_to_period(&self, period: Period) -> Result<Box<dyn InterestRateIndexTrait>>;
    fn advance_to_date(&self, date: Date) -> Result<Box<dyn InterestRateIndexTrait>>;
}

/// # HasTermStructure
/// Implement this trait for a struct that holds a term structure.
pub trait HasTermStructure {
    fn term_structure(&self) -> Result<&Box<dyn YieldTermStructureTrait>>;
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
