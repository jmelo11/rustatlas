use crate::{
    core::{meta::MetaMarketDataNode, traits::Registrable},
    instruments::fixedrateinstrument::FixedRateInstrument,
};

use super::traits::Visit;

pub struct IndexingVisitor {
    meta_market_data: Vec<MetaMarketDataNode>,
}

impl IndexingVisitor {
    pub fn new() -> IndexingVisitor {
        IndexingVisitor {
            meta_market_data: Vec::new(),
        }
    }

    pub fn meta_market_data(&self) -> &Vec<MetaMarketDataNode> {
        &self.meta_market_data
    }
}

impl Visit<FixedRateInstrument> for IndexingVisitor {
    fn visit(&mut self, instruments: &mut Vec<FixedRateInstrument>) {}

    fn par_visit(&mut self, instruments: &mut Vec<FixedRateInstrument>) {
        self.visit(instruments);
    }
}
