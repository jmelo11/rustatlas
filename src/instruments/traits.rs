use crate::{cashflows::enums::Cashflow, core::traits::Registrable};

pub trait CashflowsBounds {
    fn cashflows(&self) -> &Vec<Cashflow>;
    fn sort_stream(&mut self) {
        self.cashflows()
            .sort_by(|a, b| a.registry_id().cmp(&b.registry_id()));
    }

    fn lower_bound(&self) -> usize {
        match self.cashflows().first() {
            Some(cashflow) => match cashflow.registry_id() {
                Some(id) => id,
                None => panic!("First cashflow has no registry_id"),
            },
            None => panic!("CashflowStream has no cashflows"),
        }
    }

    fn upper_bound(&self) -> usize {
        match self.cashflows().last() {
            Some(cashflow) => match cashflow.registry_id() {
                Some(id) => id,
                None => panic!("Last cashflow has no registry_id"),
            },
            None => panic!("CashflowStream has no cashflows"),
        }
    }
}
