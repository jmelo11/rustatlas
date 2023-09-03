use crate::core::{meta::MarketRequest, traits::Registrable};

use super::traits::{HasCashflows, Visit};

pub struct IndexingVisitor {
    request: Vec<MarketRequest>,
}

impl IndexingVisitor {
    pub fn new() -> Self {
        IndexingVisitor {
            request: Vec::new(),
        }
    }

    pub fn request(&self) -> &Vec<MarketRequest> {
        &self.request
    }
}

impl<T: HasCashflows> Visit<T, ()> for IndexingVisitor {
    type Output = ();
    fn visit(&mut self, has_cashflows: &mut T) -> Self::Output {
        has_cashflows.mut_cashflows().iter_mut().for_each(|cf| {
            cf.register_id(self.request.len());
            self.request.push(cf.market_request());
        });
    }
}
