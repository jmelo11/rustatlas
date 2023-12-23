// use crate::{cashflows::cashflow::Cashflow, prelude::InterestRate, visitors::traits::HasCashflows};

// pub struct FixFloatSwap {
//     fixed_rate: InterestRate,
//     spread: f64,
//     cashflows: Vec<Cashflow>,
//     pos: usize,
//     id: Option<usize>,
// }

// impl FixFloatSwap {
//     pub fn new(
//         fixed_rate: InterestRate,
//         spread: f64,
//         first_leg: Vec<Cashflow>,
//         second_leg: Vec<Cashflow>,
//         id: Option<usize>,
//     ) -> Self {
//         let mut cashflows = Vec::new();
//         cashflows.extend_from_slice(&first_leg);
//         cashflows.extend_from_slice(&second_leg);
//         let pos = first_leg.len();
//         FixFloatSwap {
//             fixed_rate,
//             spread,
//             cashflows,
//             pos,
//             id,
//         }
//     }

//     pub fn id(&self) -> Option<usize> {
//         self.id
//     }

//     pub fn first_leg(&self) -> &[Cashflow] {
//         &self.cashflows[..self.pos]
//     }

//     pub fn second_leg(&self) -> &[Cashflow] {
//         &self.cashflows[self.pos..]
//     }
// }

// impl HasCashflows for FixFloatSwap {
//     fn cashflows(&self) -> &[Cashflow] {
//         self.cashflows.as_slice()
//     }

//     fn mut_cashflows(&mut self) -> &mut [Cashflow] {
//         self.cashflows.as_mut_slice()
//     }
// }
