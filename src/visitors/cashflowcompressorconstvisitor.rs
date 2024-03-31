use std::{collections::HashMap, hash::Hash};

use crate::{
    cashflows::cashflow::{Cashflow, CashflowType},
    currencies::enums::Currency,
    instruments::instrument::RateType,
};

/// # CashflowGroup
/// Struct that defines a cashflow group.
pub struct CashflowGroup {
    pub currency: Currency,
    pub rate_type: RateType,
    pub cashflow_type: CashflowType,
    pub discount_curve_id: Option<usize>,
    pub forecast_curve_id: Option<usize>,
}

impl Hash for CashflowGroup {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.currency.hash(state);
        self.rate_type.hash(state);
        self.cashflow_type.hash(state);
        self.discount_curve_id.hash(state);
        self.forecast_curve_id.hash(state);
    }
}

/// # CashflowCompressorConstVisitor
/// This visitor is used to compress cashflows into groups to reduce the number of cashflows that need to be processed.
pub struct CashflowCompressorConstVisitor {
    pub cashflow_groups: HashMap<CashflowGroup, Vec<Cashflow>>,
}

impl CashflowCompressorConstVisitor {
    pub fn new() -> Self {
        Self {
            cashflow_groups: HashMap::new(),
        }
    }
}
