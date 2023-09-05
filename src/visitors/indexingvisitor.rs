use std::cell::RefCell;

use crate::core::{meta::MarketRequest, traits::Registrable};

use super::traits::{HasCashflows, Visit};

/// # IndexingVisitor
/// IndexingVisitor is a visitor that registers the cashflows of an instrument
/// and generates a vector of market requests.
pub struct IndexingVisitor {
    request: RefCell<Vec<MarketRequest>>,
}

impl IndexingVisitor {
    pub fn new() -> Self {
        IndexingVisitor {
            request: RefCell::new(Vec::new()),
        }
    }

    pub fn request(&self) -> Vec<MarketRequest> {
        self.request.borrow().clone()
    }
}

impl<T: HasCashflows> Visit<T, ()> for IndexingVisitor {
    type Output = ();
    fn visit(&self, has_cashflows: &mut T) -> Self::Output {
        let mut requests = self.request.borrow_mut();
        has_cashflows.mut_cashflows().iter_mut().for_each(|cf| {
            cf.register_id(requests.len());
            requests.push(cf.market_request());
        });
    }
}
