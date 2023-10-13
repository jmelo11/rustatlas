// use crate::{
//     math::interpolation::{linear::LinearInterpolator, loglinear::LogLinearInterpolator},
//     rates::{
//         enums::Compounding,
//         traits::{HasReferenceDate, YieldProvider, YieldProviderError},
//     },
//     time::{date::Date, enums::Frequency},
// };

// use super::flatforwardtermstructure::FlatForwardTermStructure;
// use super::zeroratetermstructure::ZeroRateTermStructure;
// use super::{
//     discounttermstructure::DiscountTermStructure, spreadtermstructure::SpreadedTermStructure,
// };

// /// # YieldTermStructure
// /// Enum for YieldTermStructure
// #[derive(Clone)]
// pub enum YieldTermStructure {
//     FlatForward(FlatForwardTermStructure),
//     DiscountLinear(DiscountTermStructure<LinearInterpolator>),
//     DiscountLogLinear(DiscountTermStructure<LogLinearInterpolator>),
//     ZeroRateLinear(ZeroRateTermStructure<LinearInterpolator>),
//     ZeroRateLogLinear(ZeroRateTermStructure<LogLinearInterpolator>),
//     ConstantSpreadedDiscount(
//         SpreadedTermStructure<
//             FlatForwardTermStructure,
//             DiscountTermStructure<LogLinearInterpolator>,
//         >,
//     ),
//     CurveSpreadedDiscount(
//         SpreadedTermStructure<
//             ZeroRateTermStructure<LinearInterpolator>,
//             DiscountTermStructure<LogLinearInterpolator>,
//         >,
//     ),
//     CurveSpreadedZero(
//         SpreadedTermStructure<
//             ZeroRateTermStructure<LinearInterpolator>,
//             ZeroRateTermStructure<LinearInterpolator>,
//         >,
//     ),
// }

// impl HasReferenceDate for YieldTermStructure {
//     fn reference_date(&self) -> Date {
//         match self {
//             YieldTermStructure::FlatForward(term_structure) => term_structure.reference_date(),
//             YieldTermStructure::DiscountLinear(term_structure) => term_structure.reference_date(),
//             YieldTermStructure::DiscountLogLinear(term_structure) => {
//                 term_structure.reference_date()
//             }
//             YieldTermStructure::ZeroRateLinear(term_structure) => term_structure.reference_date(),
//             YieldTermStructure::ZeroRateLogLinear(term_structure) => {
//                 term_structure.reference_date()
//             }
//             YieldTermStructure::ConstantSpreadedDiscount(term_structure) => {
//                 term_structure.reference_date()
//             }
//             YieldTermStructure::CurveSpreadedDiscount(term_structure) => {
//                 term_structure.reference_date()
//             }
//             YieldTermStructure::CurveSpreadedZero(term_structure) => {
//                 term_structure.reference_date()
//             }
//         }
//     }
// }

// impl YieldProvider for YieldTermStructure {
//     fn discount_factor(&self, date: Date) -> Result<f64, YieldProviderError> {
//         match self {
//             YieldTermStructure::FlatForward(term_structure) => term_structure.discount_factor(date),
//             YieldTermStructure::DiscountLinear(term_structure) => {
//                 term_structure.discount_factor(date)
//             }
//             YieldTermStructure::DiscountLogLinear(term_structure) => {
//                 term_structure.discount_factor(date)
//             }
//             YieldTermStructure::ZeroRateLinear(term_structure) => {
//                 term_structure.discount_factor(date)
//             }
//             YieldTermStructure::ZeroRateLogLinear(term_structure) => {
//                 term_structure.discount_factor(date)
//             }
//             YieldTermStructure::ConstantSpreadedDiscount(term_structure) => {
//                 term_structure.discount_factor(date)
//             }
//             YieldTermStructure::CurveSpreadedDiscount(term_structure) => {
//                 term_structure.discount_factor(date)
//             }
//             YieldTermStructure::CurveSpreadedZero(term_structure) => {
//                 term_structure.discount_factor(date)
//             }
//         }
//     }

//     fn forward_rate(
//         &self,
//         start_date: Date,
//         end_date: Date,
//         comp: Compounding,
//         freq: Frequency,
//     ) -> Result<f64, YieldProviderError> {
//         match self {
//             YieldTermStructure::FlatForward(term_structure) => {
//                 term_structure.forward_rate(start_date, end_date, comp, freq)
//             }
//             YieldTermStructure::DiscountLinear(term_structure) => {
//                 term_structure.forward_rate(start_date, end_date, comp, freq)
//             }
//             YieldTermStructure::DiscountLogLinear(term_structure) => {
//                 term_structure.forward_rate(start_date, end_date, comp, freq)
//             }
//             YieldTermStructure::ZeroRateLinear(term_structure) => {
//                 term_structure.forward_rate(start_date, end_date, comp, freq)
//             }
//             YieldTermStructure::ZeroRateLogLinear(term_structure) => {
//                 term_structure.forward_rate(start_date, end_date, comp, freq)
//             }
//             YieldTermStructure::ConstantSpreadedDiscount(term_structure) => {
//                 term_structure.forward_rate(start_date, end_date, comp, freq)
//             }
//             YieldTermStructure::CurveSpreadedDiscount(term_structure) => {
//                 term_structure.forward_rate(start_date, end_date, comp, freq)
//             }
//             YieldTermStructure::CurveSpreadedZero(term_structure) => {
//                 term_structure.forward_rate(start_date, end_date, comp, freq)
//             }
//         }
//     }
// }

// // impl AdvanceInTime for YieldTermStructure {
// //     type Output = YieldTermStructure;
// //     fn advance(&self, period: Period) -> YieldTermStructure {
// //         match self {
// //             YieldTermStructure::FlatForwardTermStructure(term_structure) => {
// //                 YieldTermStructure::FlatForwardTermStructure(term_structure.advance(period))
// //             }
// //             YieldTermStructure::Other => panic!("No advance in time for this term structure"),
// //         }
// //     }
// // }
