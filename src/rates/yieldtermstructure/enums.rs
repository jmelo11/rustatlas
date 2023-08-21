use crate::{
    rates::{enums::Compounding, traits::YieldProvider},
    time::{date::Date, enums::Frequency},
};

use super::flatforwardtermstructure::FlatForwardTermStructure;

/// # YieldTermStructure
/// Enum for YieldTermStructure
#[derive(Clone, Copy)]
pub enum YieldTermStructure {
    FlatForwardTermStructure(FlatForwardTermStructure),
    Other,
}

impl YieldProvider for YieldTermStructure {
    fn discount_factor(&self, start: Date, end: Date) -> f64 {
        match self {
            YieldTermStructure::FlatForwardTermStructure(term_structure) => {
                term_structure.discount_factor(start, end)
            }
            YieldTermStructure::Other => panic!("No discount for this term structure"),
        }
    }

    fn forward_rate(
        &self,
        start_date: Date,
        end_date: Date,
        comp: Compounding,
        freq: Frequency,
    ) -> f64 {
        match self {
            YieldTermStructure::FlatForwardTermStructure(term_structure) => {
                term_structure.forward_rate(start_date, end_date, comp, freq)
            }
            YieldTermStructure::Other => panic!("No forward rate for this term structure"),
        }
    }

    fn compound_factor(&self, start: Date, end: Date) -> f64 {
        match self {
            YieldTermStructure::FlatForwardTermStructure(term_structure) => {
                term_structure.compound_factor(start, end)
            }
            YieldTermStructure::Other => panic!("No compound factor for this term structure"),
        }
    }
}
