use crate::cashflows::cashflow::Cashflow;


pub struct CashflowCompressorConstVisitor {
    pub cashflows: Vec<Cashflow>,
}

impl CashflowCompressorConstVisitor {
    pub fn new() -> Self {
        Self {
            cashflows: Vec::new(),
        }
    }
}

