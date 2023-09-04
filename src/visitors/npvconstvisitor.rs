use std::rc::Rc;

use crate::{
    cashflows::{cashflow::Side, traits::Payable},
    core::{meta::MarketData, traits::Registrable},
};

use super::traits::{ConstVisit, HasCashflows};

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

impl<T: HasCashflows> ConstVisit<T, f64> for NPVConstVisitor {
    type Output = f64;
    fn visit(&self, visitable: &T) -> Self::Output {
        let npv = visitable.cashflows().iter().fold(0.0, |acc, cf| {
            let id = match cf.registry_id() {
                Some(id) => id,
                None => panic!("No id found for cashflow"),
            };
            let cf_market_data = self.market_data.get(id).unwrap();
            let df = match cf_market_data.df() {
                Some(df) => df,
                None => panic!("No discount factor found for cashflow"),
            };
            let fx = match cf_market_data.fx() {
                Some(fx) => fx,
                None => panic!("No exchange rate found for cashflow"),
            };
            let flag = match cf.side() {
                Side::Pay => -1.0,
                Side::Receive => 1.0,
            };

            acc + df * cf.amount() / fx * flag
        });
        return npv;
    }
}
