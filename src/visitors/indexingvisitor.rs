use crate::{
    core::{meta::MetaMarketData, registry::Registrable},
    instruments::fixedrateinstrument::FixedRateInstrument,
};

use super::traits::Visit;

pub struct IndexingVisitor {
    meta_market_data: Vec<MetaMarketData>,
}

impl IndexingVisitor {
    pub fn new() -> IndexingVisitor {
        IndexingVisitor {
            meta_market_data: Vec::new(),
        }
    }

    pub fn meta_market_data(&self) -> &Vec<MetaMarketData> {
        &self.meta_market_data
    }
}

impl Visit<FixedRateInstrument> for IndexingVisitor {
    fn visit(&mut self, instruments: &mut Vec<FixedRateInstrument>) {
        for ins in instruments {
            ins.cashflows().iter_mut().for_each(|cf| {
                let id = self.meta_market_data.len();
                cf.register_id(id);
                let meta = cf.meta_market_data();
                self.meta_market_data.push(meta);
            });
        }
    }

    fn par_visit(&mut self, instruments: &mut Vec<FixedRateInstrument>) {
        self.visit(instruments);
    }
}
