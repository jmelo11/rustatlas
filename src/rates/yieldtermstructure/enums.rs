use crate::{
    rates::{
        enums::Compounding,
        traits::{HasReferenceDate, YieldProvider},
    },
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

impl HasReferenceDate for YieldTermStructure {
    fn reference_date(&self) -> Date {
        match self {
            YieldTermStructure::FlatForwardTermStructure(term_structure) => {
                term_structure.reference_date()
            }
            YieldTermStructure::Other => panic!("No reference date for this term structure"),
        }
    }
}

impl YieldProvider for YieldTermStructure {
    fn discount_factor(&self, date: Date) -> f64 {
        match self {
            YieldTermStructure::FlatForwardTermStructure(term_structure) => {
                term_structure.discount_factor(date)
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
}
