use crate::time::date::Date;

use super::{interestrateindex::enums::InterestRateIndex, traits::HasReferenceDate};

#[derive(Clone)]
pub struct IndexStore {
    reference_date: Date,
    indexes: Vec<InterestRateIndex>,
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

    pub fn add_index(&mut self, name: String, index: InterestRateIndex) {
        if self.reference_date != index.reference_date() {
            panic!("Index reference date does not match market store reference date");
        }
        self.indexes.push(index);
        self.names.push(name);
    }

    pub fn get_index_by_name(&self, name: String) -> Option<&InterestRateIndex> {
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

    pub fn get_index_by_id(&self, id: usize) -> Option<&InterestRateIndex> {
        self.indexes.get(id)
    }
}
