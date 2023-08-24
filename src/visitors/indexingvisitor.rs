// use crate::{
//     core::meta::MetaMarketDataNode, instruments::fixedrateinstrument::FixedRateInstrument,
// };

// use super::traits::Visit;

// pub struct IndexingVisitor {
//     meta_market_data: Vec<MetaMarketDataNode>,
// }

// impl IndexingVisitor {
//     pub fn new() -> IndexingVisitor {
//         IndexingVisitor {
//             meta_market_data: Vec::new(),
//         }
//     }
// }

// impl Visit<FixedRateInstrument> for IndexingVisitor {
//     fn visit(&mut self, instruments: &mut [&FixedRateInstrument]) {}

//     fn par_visit(&mut self, instruments: &mut [&FixedRateInstrument]) {
//         self.visit(instruments);
//     }
// }
