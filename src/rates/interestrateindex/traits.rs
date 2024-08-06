use std::{
    collections::{BTreeMap, HashMap},
    sync::{Arc, RwLock},
};

use crate::{
    math::interpolation::enums::Interpolator,
    rates::{
        traits::{HasReferenceDate, YieldProvider},
        yieldtermstructure::traits::YieldTermStructureTrait,
    },
    time::{date::Date, enums::TimeUnit, period::Period},
    utils::errors::Result,
};

/// # FixingProvider
/// Implement this trait for a struct that provides fixing information.
pub trait FixingProvider {
    fn fixing(&self, date: Date) -> Result<f64>;
    fn fixings(&self) -> &HashMap<Date, f64>;
    fn add_fixing(&mut self, date: Date, rate: f64);

    /// Fill missing fixings using interpolation.
    fn fill_missing_fixings(&mut self, interpolator: Interpolator) {
        if !self.fixings().is_empty() {
            let first_date = self.fixings().keys().min().unwrap().clone();
            let last_date = self.fixings().keys().max().unwrap().clone();

            let aux_btreemap = self
                .fixings()
                .iter()
                .map(|(k, v)| (*k, *v))
                .collect::<BTreeMap<Date, f64>>();

            let x: Vec<f64> = aux_btreemap
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

// /// # InterestRateIndexClone
// /// Trait for cloning a given object.
// pub trait InterestRateIndexClone {
//     fn clone_box(&self) -> Box<dyn InterestRateIndexTrait>;
// }

// /// # InterestRateIndexClone for T
// impl<T: 'static + InterestRateIndexTrait + Clone> InterestRateIndexClone for T {
//     fn clone_box(&self) -> Box<dyn InterestRateIndexTrait> {
//         Box::new(self.clone())
//     }
// }

// /// # Clone for Box<dyn InterestRateIndexTrait>
// /// Implementation of Clone for Box<dyn InterestRateIndexTrait>.
// impl Clone for Box<dyn InterestRateIndexTrait> {
//     fn clone(&self) -> Self {
//         self.clone_box()
//     }
// }

/// # AdvanceInterestRateIndexInTime
/// Trait for advancing in time a given object. Returns a represation of the object
/// as it would be after the given period/time.
pub trait AdvanceInterestRateIndexInTime {
    fn advance_to_period(&self, period: Period) -> Result<Arc<RwLock<dyn InterestRateIndexTrait>>>;
    fn advance_to_date(&self, date: Date) -> Result<Arc<RwLock<dyn InterestRateIndexTrait>>>;
}
/// # HasTenor
/// Implement this trait for a struct that holds a tenor.
pub trait HasTenor {
    fn tenor(&self) -> Period;
}

/// # HasTermStructure
/// Implement this trait for a struct that holds a term structure.
pub trait HasTermStructure {
    fn term_structure(&self) -> Result<Arc<dyn YieldTermStructureTrait>>;
}

/// # HasName
/// Implement this trait for a struct that holds a name.
pub trait HasName {
    fn name(&self) -> Result<String>;
}

/// # RelinkableTermStructure
/// Allows to link a term structure to another.
pub trait RelinkableTermStructure {
    fn link_to(&mut self, term_structure: Arc<dyn YieldTermStructureTrait>);
}

/// # InterestRateIndexTrait
/// Implement this trait for a struct that holds interest rate index information.
pub trait InterestRateIndexTrait:
    FixingProvider
    + YieldProvider
    + HasReferenceDate
    + AdvanceInterestRateIndexInTime
    + HasTermStructure
    + RelinkableTermStructure
    + HasTenor
    + HasName
    + Send
    + Sync
{
}
