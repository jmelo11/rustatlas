use std::cell::RefCell;

use super::traits::{HasCashflows, Visit};
use crate::{
    core::{meta::MarketRequest, traits::Registrable},
    utils::errors::Result,
};

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
    type Output = Result<()>;
    fn visit(&self, has_cashflows: &mut T) -> Self::Output {
        let mut requests = self.request.borrow_mut();
        has_cashflows
            .mut_cashflows()
            .iter_mut()
            .try_for_each(|cf| -> Result<()> {
                cf.set_id(requests.len());
                let request = cf.market_request()?;
                requests.push(request);
                Ok(())
            })?;
        Ok(())
    }
}
