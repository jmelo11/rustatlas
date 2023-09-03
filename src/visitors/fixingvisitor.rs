use crate::core::meta::MarketData;

struct FixingVisitor {
    market_data: Vec<MarketData>,
}

impl FixingVisitor {
    pub fn new(market_data: Vec<MarketData>) -> Self {
        FixingVisitor {
            market_data: market_data,
        }
    }
}
