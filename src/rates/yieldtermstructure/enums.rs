use crate::{
    rates::{
        enums::Compounding,
        traits::{HasReferenceDate, YieldProvider, YieldProviderError},
    },
    time::{date::Date, enums::Frequency}, math::interpolation::{linear::LinearInterpolator, loglinear::LogLinearInterpolator},
};

use super::flatforwardtermstructure::FlatForwardTermStructure;
use super::discounttermstructure::DiscountTermStructure;
use super::zeroratecurve::ZeroRateCurve;
//use super::spreadtremstructure::SpreadedTermStructure;

/// # YieldTermStructure
/// Enum for YieldTermStructure

#[derive(Clone)]
pub enum YieldTermStructure {
    FlatForwardTermStructure(FlatForwardTermStructure),
    DiscountStructureLinearInterpolation(DiscountTermStructure<LinearInterpolator>),
    DiscountStructureLogLinearInterpolation(DiscountTermStructure<LogLinearInterpolator>),
    ZeroRateCurveLinearInterpolation(ZeroRateCurve<LinearInterpolator>),
    ZeroRateCurveLogLinearInterpolation(ZeroRateCurve<LogLinearInterpolator>),
    //SpreadTermStructure(SpreadedTermStructure<YieldProvider,YieldProvider>),
    Other,
}

impl HasReferenceDate for YieldTermStructure {
    fn reference_date(&self) -> Date {
        match self {
            YieldTermStructure::FlatForwardTermStructure(term_structure) => {
                term_structure.reference_date()
            }
            YieldTermStructure::DiscountStructureLinearInterpolation(term_structure) => {
                term_structure.reference_date()
            }
            YieldTermStructure::DiscountStructureLogLinearInterpolation(term_structure) => {
                term_structure.reference_date()
            }
            YieldTermStructure::ZeroRateCurveLinearInterpolation(term_structure) => {
                term_structure.reference_date()
            }
            YieldTermStructure::ZeroRateCurveLogLinearInterpolation(term_structure) => {
                term_structure.reference_date()
            }
            // YieldTermStructure::SpreadTermStructure(term_structure) => {
            //     term_structure.reference_date()
            // }
            YieldTermStructure::Other => panic!("No reference date for this term structure"),
        }
    }
}

impl YieldProvider for YieldTermStructure {
    fn discount_factor(&self, date: Date) -> Result<f64, YieldProviderError> {
        match self {
            YieldTermStructure::FlatForwardTermStructure(term_structure) => {
                term_structure.discount_factor(date)
            }
            YieldTermStructure::DiscountStructureLinearInterpolation(term_structure) => {
                term_structure.discount_factor(date)
            }
            YieldTermStructure::DiscountStructureLogLinearInterpolation(term_structure) => {
                term_structure.discount_factor(date)
            }
            YieldTermStructure::ZeroRateCurveLinearInterpolation(term_structure) => {
                term_structure.discount_factor(date)
            }
            YieldTermStructure::ZeroRateCurveLogLinearInterpolation(term_structure) => {
                term_structure.discount_factor(date)
            }
            // YieldTermStructure::SpreadTermStructure(term_structure) => {
            //     term_structure.discount_factor(date)
            // }
            YieldTermStructure::Other => panic!("No discount for this term structure"),
        }
    }

    fn forward_rate(
        &self,
        start_date: Date,
        end_date: Date,
        comp: Compounding,
        freq: Frequency,
    ) -> Result<f64, YieldProviderError> {
        match self {
            YieldTermStructure::FlatForwardTermStructure(term_structure) => {
                term_structure.forward_rate(start_date, end_date, comp, freq)
            }
            YieldTermStructure::DiscountStructureLinearInterpolation(term_structure) => {
                term_structure.forward_rate(start_date, end_date, comp, freq)
            }
            YieldTermStructure::DiscountStructureLogLinearInterpolation(term_structure) => {
                term_structure.forward_rate(start_date, end_date, comp, freq)
            }
            YieldTermStructure::ZeroRateCurveLinearInterpolation(term_structure) => {
                term_structure.forward_rate(start_date, end_date, comp, freq)
            }
            YieldTermStructure::ZeroRateCurveLogLinearInterpolation(term_structure) => {
                term_structure.forward_rate(start_date, end_date, comp, freq)
            }
            // YieldTermStructure::SpreadTermStructure(term_structure) => {
            //     term_structure.forward_rate(start_date, end_date, comp, freq)
            // }
            YieldTermStructure::Other => panic!("No forward rate for this term structure"),
        }
    }
}

// impl AdvanceInTime for YieldTermStructure {
//     type Output = YieldTermStructure;
//     fn advance(&self, period: Period) -> YieldTermStructure {
//         match self {
//             YieldTermStructure::FlatForwardTermStructure(term_structure) => {
//                 YieldTermStructure::FlatForwardTermStructure(term_structure.advance(period))
//             }
//             YieldTermStructure::Other => panic!("No advance in time for this term structure"),
//         }
//     }
// }
