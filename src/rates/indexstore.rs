use crate::time::{date::Date, enums::TimeUnit, period::Period};

use super::{
    interestrateindex::traits::InterestRateIndexTrait,
    yieldtermstructure::traits::AdvanceInTimeError,
};

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

    pub fn add_index(&mut self, name: String, index: Box<dyn InterestRateIndexTrait>) {
        if self.reference_date != index.reference_date() {
            panic!("Index reference date does not match market store reference date");
        }
        self.indexes.push(index);
        self.names.push(name);
    }

    pub fn get_index_by_name(&self, name: String) -> Option<&Box<dyn InterestRateIndexTrait>> {
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
            Some(id) => self.indexes.get(id),
            None => None,
        }
    }

    pub fn get_index_by_id(&self, id: usize) -> Option<&Box<dyn InterestRateIndexTrait>> {
        self.indexes.get(id)
    }

    pub fn advance_to_period(&self, period: Period) -> Result<IndexStore, AdvanceInTimeError> {
        let reference_date = self.reference_date + period;
        let mut store = IndexStore::new(reference_date);
        for (name, index) in self.names.iter().zip(self.indexes.iter()) {
            let new_index = index.advance_to_period(period)?;
            store.add_index(name.clone(), new_index);
        }
        Ok(store)
    }

    pub fn advance_to_date(&self, date: Date) -> Result<IndexStore, AdvanceInTimeError> {
        let days = (date - self.reference_date) as i32;
        self.advance_to_period(Period::new(days, TimeUnit::Days))
    }
}
