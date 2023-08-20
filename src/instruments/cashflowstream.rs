use crate::{cashflows::enums::Cashflow, core::registry::Registrable};

pub struct CashflowStream {
    cashflows: Vec<Cashflow>,
}

impl CashflowStream {
    pub fn new(cashflows: Vec<Cashflow>) -> CashflowStream {
        CashflowStream { cashflows }
    }

    pub fn cashflows(&mut self) -> &mut Vec<Cashflow> {
        return &mut self.cashflows;
    }
}
pub trait CashflowStreamBounds {
    fn sort_stream(&mut self);
    fn lower_bound(&self) -> usize;
    fn upper_bound(&self) -> usize;
}

impl CashflowStreamBounds for CashflowStream {
    fn sort_stream(&mut self) {
        self.cashflows
            .sort_by(|a, b| a.registry_id().cmp(&b.registry_id()));
    }

    fn lower_bound(&self) -> usize {
        match self.cashflows.first() {
            Some(cashflow) => match cashflow.registry_id() {
                Some(id) => id,
                None => panic!("First cashflow has no registry_id"),
            },
            None => panic!("CashflowStream has no cashflows"),
        }
    }

    fn upper_bound(&self) -> usize {
        match self.cashflows.last() {
            Some(cashflow) => match cashflow.registry_id() {
                Some(id) => id,
                None => panic!("Last cashflow has no registry_id"),
            },
            None => panic!("CashflowStream has no cashflows"),
        }
    }
}
