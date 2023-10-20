use std::rc::Rc;

use crate::{
    cashflows::{cashflow::{Side, self}, traits::Payable},
    core::{meta::MarketData, traits::Registrable},
};

use super::traits::{ConstVisit, EvaluationError, HasCashflows};

/// # NPVConstVisitor
/// NPVConstVisitor is a visitor that calculates the NPV of an instrument.
/// It assumes that the cashflows of the instrument have already been indexed and fixed.
pub struct NPVConstVisitor {
    market_data: Rc<Vec<MarketData>>,
    take_today:  bool,
}

impl NPVConstVisitor {
    pub fn new(market_data: Rc<Vec<MarketData>>) -> Self {
        NPVConstVisitor {
            market_data: market_data,
            take_today:  true,
        }
    }
    pub fn set_take_today(&mut self, take_today: bool) {
        self.take_today = take_today;
    }
}

impl<T: HasCashflows> ConstVisit<T> for NPVConstVisitor {
    type Output = Result<f64, EvaluationError>;
    fn visit(&self, visitable: &T) -> Self::Output {
        let npv = visitable.cashflows().iter().try_fold(0.0, |acc, cf| {

            let id = cf.registry_id().ok_or(EvaluationError::NoRegistryId)?;

            let cf_market_data = self
                .market_data
                .get(id)
                .ok_or(EvaluationError::NoMarketData)?;


            let mut aux = 1.0;
            let market_data_date = cf_market_data.reference_date();
            if market_data_date == cf.payment_date() && !self.take_today {
                aux = 0.0;
            }

            let df = cf_market_data
                .df()
                .ok_or(EvaluationError::NoDiscountFactor)?;

            let fx = cf_market_data.fx().ok_or(EvaluationError::NoExchangeRate)?;

            let flag = match cf.side() {
                Side::Pay => -1.0,
                Side::Receive => 1.0,
            };

            let amount = cf.amount().ok_or(EvaluationError::NoAmount)?;

            Ok(acc + aux * df * amount / fx * flag)
        });
        return npv;
    }
}
