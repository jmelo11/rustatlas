use std::rc::Rc;

use crate::{
    cashflows::{cashflow::Side, traits::Payable},
    core::{
        meta::{MarketData, MarketDataError},
        traits::Registrable,
    },
};

use super::traits::{ConstVisit, HasCashflows};

/// # NPVConstVisitor
/// NPVConstVisitor is a visitor that calculates the NPV of an instrument.
/// It assumes that the cashflows of the instrument have already been indexed and fixed.
pub struct NPVConstVisitor {
    market_data: Rc<Vec<MarketData>>,
}

impl NPVConstVisitor {
    pub fn new(market_data: Rc<Vec<MarketData>>) -> Self {
        NPVConstVisitor {
            market_data: market_data,
        }
    }
}

impl<T: HasCashflows> ConstVisit<T> for NPVConstVisitor {
    type Output = Result<f64, MarketDataError>;
    fn visit(&self, visitable: &T) -> Self::Output {
        let npv = visitable
            .cashflows()
            .iter()
            .fold(Ok(0.0), |acc_result, cf| {
                let acc = match acc_result {
                    Ok(value) => value,
                    Err(_) => return acc_result,
                };

                let id = cf.registry_id().ok_or(MarketDataError::NoRegistryId)?;
                let cf_market_data = self.market_data.get(id).unwrap();
                let df = cf_market_data
                    .df()
                    .ok_or(MarketDataError::NoDiscountFactor)?;
                let fx = cf_market_data
                    .fx()
                    .ok_or(MarketDataError::NoDiscountFactor)?;
                let flag = match cf.side() {
                    Side::Pay => -1.0,
                    Side::Receive => 1.0,
                };
                let amount = match cf.amount() {
                    Some(amount) => amount,
                    None => panic!("No amount found for cashflow"),
                };

                Ok(acc + df * amount / fx * flag)
            });
        return npv;
    }
}
