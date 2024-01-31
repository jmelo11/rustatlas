use std::collections::HashMap;

use crate::{
    time::{date::Date, enums::TimeUnit, period::Period},
    utils::errors::{AtlasError, Result},
};

use super::interestrateindex::traits::InterestRateIndexTrait;

/// # IndexStore
/// A store for interest rate indices.
/// 
/// ## Parameters
/// * `reference_date` - The reference date of the index store
#[derive(Clone)]
pub struct IndexStore {
    reference_date: Date,
    index_map: HashMap<usize, Box<dyn InterestRateIndexTrait>>,
}

impl IndexStore {
    pub fn new(reference_date: Date) -> IndexStore {
        IndexStore {
            reference_date,
            index_map: HashMap::new(),
        }
    }

    pub fn reference_date(&self) -> Date {
        self.reference_date
    }

    pub fn add_index(&mut self, id: usize, index: Box<dyn InterestRateIndexTrait>) -> Result<()> {
        if self.reference_date != index.reference_date() {
            return Err(AtlasError::InvalidValueErr(
                "Index reference date does not match store reference date".to_string(),
            ));
        }
        // check if name already exists
        if self.index_map.contains_key(&id) {
            return Err(AtlasError::InvalidValueErr(format!(
                "Index with id {} already exists",
                id
            )));
        }

        self.index_map.insert(id, index);

        Ok(())
    }

    pub fn get_index(&self, id: usize) -> Result<&Box<dyn InterestRateIndexTrait>> {
        self.index_map
            .get(&id)
            .ok_or(AtlasError::NotFoundErr(format!(
                "Index with id {} not found",
                id
            )))
    }

    pub fn get_index_by_name(&self, name: String) -> Result<&Box<dyn InterestRateIndexTrait>> {
        for (id, index) in self.index_map.iter() {
            if index.name()? == name {
                return self.get_index(*id);
            }
        }
        Err(AtlasError::NotFoundErr(format!(
            "Index with name {} not found",
            name
        )))
    }

    pub fn get_index_names(&self) -> Vec<String> {
        let mut names = Vec::new();
        for index in self.index_map.values() {
            names.push(index.name().unwrap());
        }
        names
    }

    pub fn get_index_map(&self) -> HashMap<String,usize> {
        let mut map = HashMap::new();
        for (id, index) in self.index_map.iter() {
            map.insert(index.name().unwrap(), *id);
        }
        map
    }

    pub fn next_available_id(&self) -> usize {
        let keys = self.index_map.keys();
        let mut max = 0;
        for key in keys {
            if *key > max {
                max = *key;
            }
        }
        max + 1
    }

    pub fn advance_to_period(&self, period: Period) -> Result<IndexStore> {
        let reference_date = self.reference_date + period;
        let mut store = IndexStore::new(reference_date);
        for (id, index) in self.index_map.iter() {
            let new_index = index.advance_to_period(period)?;
            store.add_index(*id, new_index)?;
        }
        Ok(store)
    }

    pub fn advance_to_date(&self, date: Date) -> Result<IndexStore> {
        let days = (date - self.reference_date) as i32;
        self.advance_to_period(Period::new(days, TimeUnit::Days))
    }
}
