use std::cell::RefCell;

use crate::core::{
    meta::MarketRequest,
    traits::{MarketRequestError, Registrable},
};

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

impl<T: HasCashflows> Visit<T> for IndexingVisitor {
    type Output = Result<(), MarketRequestError>;
    fn visit(&self, has_cashflows: &mut T) -> Self::Output {
        let mut requests = self.request.borrow_mut();
        for cf in has_cashflows.mut_cashflows() {
            cf.register_id(requests.len());
            let request = cf.market_request()?;
            requests.push(request);
        }
        Ok(())
    }
}
