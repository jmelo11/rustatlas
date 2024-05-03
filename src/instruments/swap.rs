use super::leg::Leg;
use crate::cashflows::cashflow::Cashflow;

/// # Swap
/// A financial swap derivative.
pub struct Swap {
    cashflows: Vec<Cashflow>,
    legs: Vec<Leg>,
    id: Option<String>,
}

impl Swap {
    /// Create a new swap.
    pub fn new(cashflows: Vec<Cashflow>, legs: Vec<Leg>, id: Option<String>) -> Self {
        Swap {
            cashflows,
            legs,
            id,
        }
    }

    /// Add a leg to the swap.
    pub fn add_leg(&mut self, leg: Leg) {
        self.legs.push(leg.clone());
        self.cashflows
            .extend(leg.clone().cashflows().iter().cloned());
    }

    /// Get the legs of the swap.
    pub fn legs(&self) -> &Vec<Leg> {
        &self.legs
    }

    /// Get the cashflows of the swap.
    pub fn cashflows(&self) -> &Vec<Cashflow> {
        &self.cashflows
    }

    /// Get the id of the swap.
    pub fn id(&self) -> &Option<String> {
        &self.id
    }

    /// Set the id of the swap.
    pub fn set_id(&mut self, id: String) {
        self.id = Some(id);
    }
}
