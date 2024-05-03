use std::{
    collections::HashMap,
    sync::{Arc, RwLock, RwLockReadGuard},
};

use crate::{
    currencies::enums::Currency, 
    time::{date::Date, enums::TimeUnit, period::Period}, 
    utils::errors::{AtlasError, Result}
};

use super::{
    interestrateindex::traits::InterestRateIndexTrait,
    yieldtermstructure::traits::YieldTermStructureTrait,
};

/// # IndexStore
/// A store for interest rate indices.
///
/// ## Parameters
/// * `reference_date` - The reference date of the index store
#[derive(Clone)]
pub struct IndexStore {
    reference_date: Date,
    index_map: HashMap<usize, Arc<RwLock<dyn InterestRateIndexTrait>>>,
    currency_curve: HashMap<Currency, usize>,
}

pub trait ReadIndex {
    fn read_index(&self) -> Result<RwLockReadGuard<dyn InterestRateIndexTrait>>;
}

impl ReadIndex for Arc<RwLock<dyn InterestRateIndexTrait>> {
    fn read_index(&self) -> Result<RwLockReadGuard<dyn InterestRateIndexTrait>> {
        self.read()
            .map_err(|_| AtlasError::InvalidValueErr("Could not read index".to_string()))
    }
}

impl IndexStore {
    pub fn new(reference_date: Date) -> IndexStore {
        IndexStore {
            reference_date,
            index_map: HashMap::new(),
            currency_curve: HashMap::new(),
        }
    }

    pub fn reference_date(&self) -> Date {
        self.reference_date
    }

    pub fn add_currency_curve(&mut self, currency: Currency, fx_curve: usize) {
        self.currency_curve.insert(currency, fx_curve);
    }
    
    pub fn get_currency_curve(&self, currency: Currency) -> Result<usize> {
        self.currency_curve
            .get(&currency)
            .cloned()
            .ok_or(AtlasError::NotFoundErr(format!(
                "Currency curve for currency {:?}",
                currency
            )))
    }

    pub fn link_term_structure(
        &self,
        id: usize,
        term_structure: Arc<dyn YieldTermStructureTrait>,
    ) -> Result<()> {
        self.index_map
            .get(&id)
            .ok_or(AtlasError::NotFoundErr(format!(
                "Index with id {} not found",
                id
            )))?
            .write()
            .map_err(|_| AtlasError::InvalidValueErr("Could not write index".to_string()))?
            .link_to(term_structure);
        Ok(())
    }

    pub fn add_index(
        &mut self,
        id: usize,
        index: Arc<RwLock<dyn InterestRateIndexTrait>>,
    ) -> Result<()> {
        if self.reference_date != index.read_index()?.reference_date() {
            return Err(AtlasError::InvalidValueErr(
                format!(
                    "Index ({:?}) reference date ({}) does not match index store reference date ({})",
                    index.read_index()?.name(),
                    index.read_index()?.reference_date(),
                    self.reference_date
                )
                .to_string(),
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

    pub fn replace_index(
        &mut self,
        id: usize,
        index: Arc<RwLock<dyn InterestRateIndexTrait>>,
    ) -> Result<()> {
        if self.reference_date != index.read_index()?.reference_date() {
            return Err(AtlasError::InvalidValueErr(
                format!(
                    "Index ({:?}) reference date ({}) does not match index store reference date ({})",
                    index.read_index()?.name(),
                    index.read_index()?.reference_date(),
                    self.reference_date
                )
                .to_string(),
            ));
        }
        // check if name already exists
        if !self.index_map.contains_key(&id) {
            return Err(AtlasError::InvalidValueErr(format!(
                "Index with id {} does not exist",
                id
            )));
        }

        self.index_map.insert(id, index);

        Ok(())
    }

    pub fn get_index(&self, id: usize) -> Result<Arc<RwLock<dyn InterestRateIndexTrait>>> {
        self.index_map
            .get(&id)
            .cloned()
            .ok_or(AtlasError::NotFoundErr(format!(
                "Index with id {} not found",
                id
            )))
    }

    pub fn get_index_by_name(
        &self,
        name: String,
    ) -> Result<Arc<RwLock<dyn InterestRateIndexTrait>>> {
        for (id, index) in self.index_map.iter() {
            if index.read_index()?.name()? == name {
                return self.get_index(*id);
            }
        }
        Err(AtlasError::NotFoundErr(format!(
            "Index with name {} not found",
            name
        )))
    }

    pub fn get_index_names(&self) -> Result<Vec<String>> {
        let mut names = Vec::new();
        for index in self.index_map.values() {
            names.push(index.read_index()?.name().unwrap());
        }
        Ok(names)
    }

    pub fn get_index_map(&self) -> Result<HashMap<String, usize>> {
        let mut map = HashMap::new();
        for (id, index) in self.index_map.iter() {
            map.insert(index.read_index()?.name().unwrap(), *id);
        }
        Ok(map)
    }

    pub fn get_all_indices(&self) -> Vec<Arc<RwLock<dyn InterestRateIndexTrait>>> {
        let mut indices = Vec::new();
        for index in self.index_map.values() {
            indices.push(index.clone());
        }
        indices
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
            let new_index = index.read_index()?.advance_to_period(period)?;
            store.add_index(*id, new_index)?;
        }

        for (currency, curve) in self.currency_curve.iter() {
            store.add_currency_curve(*currency, *curve);
        }
        
        Ok(store)
    }

    pub fn advance_to_date(&self, date: Date) -> Result<IndexStore> {
        let days = (date - self.reference_date) as i32;
        self.advance_to_period(Period::new(days, TimeUnit::Days))
    }

    /// # swaps the index with the given id to the given index
    pub fn swap_index_by_id(&mut self, from: usize, to: usize) {
        let index = self.index_map.remove(&from).unwrap();
        self.index_map.insert(to, index);
    }

    pub fn currency_forescast_factor (&self,first_currency: Currency, second_currency: Currency, date: Date) -> Result<f64> {
        let first_id = self.get_currency_curve(first_currency)?;
        let second_id = self.get_currency_curve(second_currency)?;

        let first_curve = self.get_index(first_id)?;
        let second_curve = self.get_index(second_id)?;

        let first_df = first_curve.read_index()?.discount_factor(date)?;
        let second_df = second_curve.read_index()?.discount_factor(date)?;

        Ok(second_df/ first_df)
    }
}
