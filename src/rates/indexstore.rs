use super::interestrateindex::enums::InterestRateIndex;

#[derive(Clone)]
pub struct IndexStore {
    indexes: Vec<InterestRateIndex>,
    names: Vec<String>,
}

impl IndexStore {
    pub fn new() -> IndexStore {
        IndexStore {
            indexes: Vec::new(),
            names: Vec::new(),
        }
    }

    pub fn add_index(&mut self, name: String, index: InterestRateIndex) {
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
