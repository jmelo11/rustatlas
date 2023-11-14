use std::collections::{HashMap, BTreeMap};

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

            let y = aux_btreemap
                .values()
                .map(|r| *r)
                .collect::<Vec<f64>>();

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

#[cfg(test)]
mod tests { 
    use crate::prelude::{OvernightIndex, IborIndex};
    use serde_json::{Result, Value};
    use super::*;

    #[test]
    fn test_fixing_provider_overnight () -> Result<()> {
        let data = r#"{
            "2023-06-01": 21938.71,
            "2023-06-02": 21945.57,
            "2023-06-05": 21966.14,
            "2023-06-06": 21973.0,
            "2023-06-07": 21979.87,
            "2023-06-08": 21986.74,
            "2023-06-09": 21993.61,
            "2023-06-12": 22014.23,
            "2023-06-13": 22021.11,
            "2023-06-14": 22027.99,
            "2023-06-15": 22034.87,
            "2023-06-16": 22041.76,
            "2023-06-19": 22062.42,
            "2023-06-20": 22069.31,
            "2023-06-21": 22069.31,
            "2023-06-22": 22083.1,
            "2023-06-23": 22090.0,
            "2023-06-26": 22090.0,
            "2023-06-27": 22117.61,
            "2023-06-28": 22124.52,
            "2023-06-29": 22131.43,
            "2023-06-30": 22138.35,
            "2023-07-03": 22159.1,
            "2023-07-04": 22166.02,
            "2023-07-05": 22172.95,
            "2023-07-06": 22179.88,
            "2023-07-07": 22186.81,
            "2023-07-10": 22207.61,
            "2023-07-11": 22214.55,
            "2023-07-12": 22221.49,
            "2023-07-13": 22228.43,
            "2023-07-14": 22235.38,
            "2023-07-17": 22256.23,
            "2023-07-18": 22263.19,
            "2023-07-19": 22270.15,
            "2023-07-20": 22277.11,
            "2023-07-21": 22284.07,
            "2023-07-24": 22304.96,
            "2023-07-25": 22311.93,
            "2023-07-26": 22318.9,
            "2023-07-27": 22325.87,
            "2023-07-28": 22332.85,
            "2023-07-31": 22353.79,
            "2023-08-01": 22360.15,
            "2023-08-02": 22366.52,
            "2023-08-03": 22372.89,
            "2023-08-04": 22379.26,
            "2023-08-07": 22398.38,
            "2023-08-08": 22404.76,
            "2023-08-09": 22411.14,
            "2023-08-10": 22417.52,
            "2023-08-11": 22423.9,
            "2023-08-14": 22443.05,
            "2023-08-15": 22443.05,
            "2023-08-16": 22455.83,
            "2023-08-17": 22462.22,
            "2023-08-18": 22468.62,
            "2023-08-21": 22487.81,
            "2023-08-22": 22494.21,
            "2023-08-23": 22500.61,
            "2023-08-24": 22507.02,
            "2023-08-25": 22513.43,
            "2023-08-28": 22532.66,
            "2023-08-29": 22539.08,
            "2023-08-30": 22545.5,
            "2023-08-31": 22551.92,
            "2023-09-01": 22558.34,
            "2023-09-04": 22577.59,
            "2023-09-05": 22584.02,
            "2023-09-06": 22590.45,
            "2023-09-07": 22596.41,
            "2023-09-08": 22602.37,
            "2023-09-11": 22620.11,
            "2023-09-12": 22626.08,
            "2023-09-13": 22632.05,
            "2023-09-14": 22638.02,
            "2023-09-15": 22643.99,
            "2023-09-18": 22643.99,
            "2023-09-19": 22643.99,
            "2023-09-20": 22673.87,
            "2023-09-21": 22679.85,
            "2023-09-22": 22685.83,
            "2023-09-25": 22703.79,
            "2023-09-26": 22709.78,
            "2023-09-27": 22715.77,
            "2023-09-28": 22721.73,
            "2023-09-29": 22727.72,
            "2023-10-02": 22745.71,
            "2023-10-03": 22751.71,
            "2023-10-04": 22757.69,
            "2023-10-05": 22763.68,
            "2023-10-06": 22769.67,
            "2023-10-09": 22769.67,
            "2023-10-10": 22793.7,
            "2023-10-11": 22799.72,
            "2023-10-12": 22805.74,
            "2023-10-13": 22811.76,
            "2023-10-16": 22829.82,
            "2023-10-17": 22835.84,
            "2023-10-18": 22841.87,
            "2023-10-19": 22847.9,
            "2023-10-20": 22853.93,
            "2023-10-23": 22872.02,
            "2023-10-24": 22878.06,
            "2023-10-25": 22884.1,
            "2023-10-26": 22890.14,
            "2023-10-27": 22890.14,
            "2023-10-30": 22914.3,
            "2023-10-31": 22920.03,
            "2023-11-01": 22920.03,
            "2023-11-02": 22931.49,
            "2023-11-03": 22937.22,
            "2023-11-06": 22954.42 }"#;
        
        let fixing: HashMap<Date, f64> = serde_json::from_str(data)?;
        let mut overnight_index =  OvernightIndex::new(Date::new(2023, 11, 6))
            .with_fixings(fixing);

        overnight_index.fill_missing_fixings(Interpolator::Linear);

        assert!( overnight_index.fixings().get(&Date::new(2023, 6, 3)).unwrap() - 21952.4266666 < 0.001 );
        Ok(())

    }

    #[test]
    fn test_fixing_provider_ibor () -> Result<()> {
        let data = r#"{
            "2023-06-01": 21938.71,
            "2023-06-02": 21945.57,
            "2023-06-05": 21966.14,
            "2023-06-06": 21973.0,
            "2023-06-07": 21979.87,
            "2023-06-08": 21986.74,
            "2023-06-09": 21993.61,
            "2023-06-12": 22014.23,
            "2023-06-13": 22021.11,
            "2023-06-14": 22027.99,
            "2023-06-15": 22034.87,
            "2023-06-16": 22041.76,
            "2023-06-19": 22062.42,
            "2023-06-20": 22069.31,
            "2023-06-21": 22069.31,
            "2023-06-22": 22083.1,
            "2023-06-23": 22090.0,
            "2023-06-26": 22090.0,
            "2023-06-27": 22117.61,
            "2023-06-28": 22124.52,
            "2023-06-29": 22131.43,
            "2023-06-30": 22138.35,
            "2023-07-03": 22159.1,
            "2023-07-04": 22166.02,
            "2023-07-05": 22172.95,
            "2023-07-06": 22179.88,
            "2023-07-07": 22186.81,
            "2023-07-10": 22207.61,
            "2023-07-11": 22214.55,
            "2023-07-12": 22221.49,
            "2023-07-13": 22228.43,
            "2023-07-14": 22235.38,
            "2023-07-17": 22256.23,
            "2023-07-18": 22263.19,
            "2023-07-19": 22270.15,
            "2023-07-20": 22277.11,
            "2023-07-21": 22284.07,
            "2023-07-24": 22304.96,
            "2023-07-25": 22311.93,
            "2023-07-26": 22318.9,
            "2023-07-27": 22325.87,
            "2023-07-28": 22332.85,
            "2023-07-31": 22353.79,
            "2023-08-01": 22360.15,
            "2023-08-02": 22366.52,
            "2023-08-03": 22372.89,
            "2023-08-04": 22379.26,
            "2023-08-07": 22398.38,
            "2023-08-08": 22404.76,
            "2023-08-09": 22411.14,
            "2023-08-10": 22417.52,
            "2023-08-11": 22423.9,
            "2023-08-14": 22443.05,
            "2023-08-15": 22443.05,
            "2023-08-16": 22455.83,
            "2023-08-17": 22462.22,
            "2023-08-18": 22468.62,
            "2023-08-21": 22487.81,
            "2023-08-22": 22494.21,
            "2023-08-23": 22500.61,
            "2023-08-24": 22507.02,
            "2023-08-25": 22513.43,
            "2023-08-28": 22532.66,
            "2023-08-29": 22539.08,
            "2023-08-30": 22545.5,
            "2023-08-31": 22551.92,
            "2023-09-01": 22558.34,
            "2023-09-04": 22577.59,
            "2023-09-05": 22584.02,
            "2023-09-06": 22590.45,
            "2023-09-07": 22596.41,
            "2023-09-08": 22602.37,
            "2023-09-11": 22620.11,
            "2023-09-12": 22626.08,
            "2023-09-13": 22632.05,
            "2023-09-14": 22638.02,
            "2023-09-15": 22643.99,
            "2023-09-18": 22643.99,
            "2023-09-19": 22643.99,
            "2023-09-20": 22673.87,
            "2023-09-21": 22679.85,
            "2023-09-22": 22685.83,
            "2023-09-25": 22703.79,
            "2023-09-26": 22709.78,
            "2023-09-27": 22715.77,
            "2023-09-28": 22721.73,
            "2023-09-29": 22727.72,
            "2023-10-02": 22745.71,
            "2023-10-03": 22751.71,
            "2023-10-04": 22757.69,
            "2023-10-05": 22763.68,
            "2023-10-06": 22769.67,
            "2023-10-09": 22769.67,
            "2023-10-10": 22793.7,
            "2023-10-11": 22799.72,
            "2023-10-12": 22805.74,
            "2023-10-13": 22811.76,
            "2023-10-16": 22829.82,
            "2023-10-17": 22835.84,
            "2023-10-18": 22841.87,
            "2023-10-19": 22847.9,
            "2023-10-20": 22853.93,
            "2023-10-23": 22872.02,
            "2023-10-24": 22878.06,
            "2023-10-25": 22884.1,
            "2023-10-26": 22890.14,
            "2023-10-27": 22890.14,
            "2023-10-30": 22914.3,
            "2023-10-31": 22920.03,
            "2023-11-01": 22920.03,
            "2023-11-02": 22931.49,
            "2023-11-03": 22937.22,
            "2023-11-06": 22954.42 }"#;
        
        let fixing: HashMap<Date, f64> = serde_json::from_str(data)?;
        let mut ibor_index =  IborIndex::new(Date::new(2023, 11, 6))    
            .with_fixings(fixing);

        ibor_index.fill_missing_fixings(Interpolator::Linear);
        assert!( ibor_index.fixings().get(&Date::new(2023, 6, 3)).unwrap() - 21952.4266666 < 0.001 );
        Ok(())

    }

}



