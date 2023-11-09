use std::collections::HashMap;

use crate::{
    time::{date::Date, enums::TimeUnit, period::Period},
    utils::errors::{AtlasError, Result},
};

use super::interestrateindex::traits::InterestRateIndexTrait;

#[derive(Clone)]
pub struct IndexStore {
    reference_date: Date,
    indexes: Vec<Box<dyn InterestRateIndexTrait>>,
    names: Vec<String>,
}

impl IndexStore {
    pub fn new(reference_date: Date) -> IndexStore {
        IndexStore {
            reference_date,
            indexes: Vec::new(),
            names: Vec::new(),
        }
    }

    pub fn reference_date(&self) -> Date {
        self.reference_date
    }

    pub fn add_index(
        &mut self,
        name: String,
        index: Box<dyn InterestRateIndexTrait>,
    ) -> Result<()> {
        if self.reference_date != index.reference_date() {
            return Err(AtlasError::InvalidValueErr(
                "Index reference date does not match store reference date".to_string(),
            ));
        }
        // check if name already exists
        if self.names.iter().any(|s| s == &name) {
            return Err(AtlasError::InvalidValueErr(
                "Index name already exists".to_string(),
            ));
        }

        self.indexes.push(index);
        self.names.push(name);
        Ok(())
    }

    pub fn get_index_pos_by_name(&self, name: &String) -> Result<usize> {
        self.names
            .iter()
            .position(|s| s == name)
            .ok_or(AtlasError::NotFoundErr(format!("Index {} not found", name)))
    }

    pub fn get_index_by_name(&self, name: String) -> Result<&Box<dyn InterestRateIndexTrait>> {
        let item = self.names.iter().enumerate().find_map(
            |(n, s)| {
                if s == &name {
                    Some(n)
                } else {
                    None
                }
            },
        );
        match item {
            Some(id) => self.get_index_by_id(id),
            None => Err(AtlasError::NotFoundErr(format!("Index {} not found", name))),
        }
    }

    pub fn get_index_by_id(&self, id: usize) -> Result<&Box<dyn InterestRateIndexTrait>> {
        self.indexes.get(id).ok_or(AtlasError::NotFoundErr(format!(
            "Index with id {} not found",
            id
        )))
    }

    pub fn id_to_name_map(&self) -> HashMap<usize, String> {
        self.names
            .iter()
            .enumerate()
            .map(|(n, s)| (n, s.clone()))
            .collect()
    }

    pub fn name_to_id_map(&self) -> HashMap<String, usize> {
        self.names
            .iter()
            .enumerate()
            .map(|(n, s)| (s.clone(), n))
            .collect()
    }

    pub fn advance_to_period(&self, period: Period) -> Result<IndexStore> {
        let reference_date = self.reference_date + period;
        let mut store = IndexStore::new(reference_date);
        for (name, index) in self.names.iter().zip(self.indexes.iter()) {
            let new_index = index.advance_to_period(period)?;
            store.add_index(name.clone(), new_index)?;
        }
        Ok(store)
    }

    pub fn advance_to_date(&self, date: Date) -> Result<IndexStore> {
        let days = (date - self.reference_date) as i32;
        self.advance_to_period(Period::new(days, TimeUnit::Days))
    }
}
