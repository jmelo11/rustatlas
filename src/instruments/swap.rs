use crate::{cashflows::cashflow::Cashflow, visitors::traits::HasCashflows};

pub struct Swap {
    cashflows: Vec<Cashflow>,
    pos: usize,
}

impl Swap {
    pub fn new(first_leg: Vec<Cashflow>, second_leg: Vec<Cashflow>) -> Self {
        let mut cashflows = Vec::new();
        cashflows.extend_from_slice(&first_leg);
        cashflows.extend_from_slice(&second_leg);
        let pos = first_leg.len();
        Swap { cashflows, pos }
    }

    pub fn first_leg(&self) -> &[Cashflow] {
        &self.cashflows[..self.pos]
    }

    pub fn second_leg(&self) -> &[Cashflow] {
        &self.cashflows[self.pos..]
    }
}

impl HasCashflows for Swap {
    fn cashflows(&self) -> &[Cashflow] {
        self.cashflows.as_slice()
    }

    fn mut_cashflows(&mut self) -> &mut [Cashflow] {
        self.cashflows.as_mut_slice()
    }
}
