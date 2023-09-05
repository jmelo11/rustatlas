use std::rc::Rc;

use crate::{
    cashflows::{cashflow::Cashflow, traits::RequiresFixingRate},
    core::{meta::MarketData, traits::Registrable},
};

use super::traits::{HasCashflows, Visit};

/// # FixingVisitor
/// FixingVisitor is a visitor that fixes the rate of a floating rate cashflow.
pub struct FixingVisitor {
    market_data: Rc<Vec<MarketData>>,
}

impl FixingVisitor {
    pub fn new(market_data: Rc<Vec<MarketData>>) -> Self {
        FixingVisitor {
            market_data: market_data,
        }
    }
}

impl<T: HasCashflows> Visit<T, ()> for FixingVisitor {
    type Output = ();
    fn visit(&self, has_cashflows: &mut T) -> Self::Output {
        has_cashflows
            .mut_cashflows()
            .iter_mut()
            .for_each(|cf| match cf {
                Cashflow::FloatingRateCoupon(c) => {
                    let id = match c.registry_id() {
                        Some(id) => id,
                        None => panic!("No id for cashflow"),
                    };
                    let data = match self.market_data.get(id) {
                        Some(data) => data,
                        None => panic!("No market data for id {}", id),
                    };
                    let fixing = match data.fwd() {
                        Some(fwd) => fwd,
                        None => panic!("No forward for id {}", id),
                    };
                    cf.set_fixing_rate(fixing);
                }
                _ => (),
            });
    }
}
