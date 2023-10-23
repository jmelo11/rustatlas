use std::rc::Rc;

use crate::{
    cashflows::{cashflow::Side, traits::Payable},
    core::{meta::MarketData, traits::Registrable},
    utils::errors::{AtlasError, Result},
};

use super::traits::{ConstVisit, HasCashflows};

/// # NPVConstVisitor
/// NPVConstVisitor is a visitor that calculates the NPV of an instrument.
/// It assumes that the cashflows of the instrument have already been indexed and fixed.
pub struct NPVConstVisitor {
    market_data: Rc<Vec<MarketData>>,
    include_today_cashflows: bool,
}

impl NPVConstVisitor {
    pub fn new(market_data: Rc<Vec<MarketData>>, include_today_cashflows: bool) -> Self {
        NPVConstVisitor {
            market_data: market_data,
            include_today_cashflows,
        }
    }
    pub fn set_include_today_cashflows(&mut self, include_today_cashflows: bool) {
        self.include_today_cashflows = include_today_cashflows;
    }
}

impl<T: HasCashflows> ConstVisit<T> for NPVConstVisitor {
    type Output = Result<f64>;
    fn visit(&self, visitable: &T) -> Self::Output {
        let npv = visitable.cashflows().iter().try_fold(0.0, |acc, cf| {
            let id = cf.id()?;

            let cf_market_data =
                self.market_data
                    .get(id)
                    .ok_or(AtlasError::NotFoundErr(format!(
                        "Market data for cashflow with id {}",
                        id
                    )))?;

            if cf_market_data.reference_date() == cf.payment_date() && !self.include_today_cashflows
            {
                return Ok(acc);
            }

            let df = cf_market_data.df()?;
            let fx = cf_market_data.fx()?;
            let flag = match cf.side() {
                Side::Pay => -1.0,
                Side::Receive => 1.0,
            };
            let amount = cf.amount()?;
            Ok(acc + df * amount / fx * flag)
        });
        return npv;
    }
}
