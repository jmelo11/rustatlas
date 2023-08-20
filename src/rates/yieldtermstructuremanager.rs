use crate::currencies::enums::Currency;

use super::{interestrateindex::enums::InterestRateIndex, yieldtermstructure::enums::YieldTermStructure};
use std::collections::HashMap;

pub struct CurveContext {
    id: usize,
    term_structure: YieldTermStructure,
    interest_rate_index: InterestRateIndex,
    currency: Currency,
}

impl CurveContext {
    fn new(
        id: usize,
        term_structure: YieldTermStructure,
        interest_rate_index: InterestRateIndex,
        currency: Currency,
    ) -> CurveContext {
        CurveContext {
            id,
            term_structure,
            interest_rate_index,
            currency,
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn term_structure(&self) -> YieldTermStructure {
        self.term_structure
    }

    pub fn interest_rate_index(&self) -> &InterestRateIndex {
        &self.interest_rate_index
    }
}

pub struct YieldTermStructureManager {
    contexts: Vec<CurveContext>,
    names: HashMap<String, usize>,
}

impl YieldTermStructureManager {
    pub fn new() -> YieldTermStructureManager {
        YieldTermStructureManager {
            contexts: Vec::new(),
            names: HashMap::new(),
        }
    }

    pub fn add_curve_context(
        &mut self,
        name: String,
        term_structure: YieldTermStructure,
        interest_rate_index: InterestRateIndex,
        currency: Currency,
    ) {
        let id = self.contexts.len();
        let context = CurveContext::new(id, term_structure, interest_rate_index, currency);
        self.contexts.push(context);
        self.names.insert(name, id);
    }

    pub fn get_curve_context_by_name(&self, name: String) -> Option<&CurveContext> {
        match self.names.get(&name) {
            Some(id) => self.contexts.get(*id),
            None => None,
        }
    }

    fn get_curve_context_by_id(&self, id: usize) -> Option<&CurveContext> {
        self.contexts.get(id)
    }
}
