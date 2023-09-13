use std::rc::Rc;

use crate::{
    cashflows::{cashflow::Cashflow, traits::RequiresFixingRate},
    core::{meta::MarketData, traits::Registrable},
};

use super::traits::{EvaluationError, HasCashflows, Visit};

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

impl<T: HasCashflows> Visit<T> for FixingVisitor {
    type Output = Result<(), EvaluationError>;
    fn visit(&self, has_cashflows: &mut T) -> Self::Output {
        has_cashflows.mut_cashflows().iter_mut().try_for_each(
            |cf| -> Result<(), EvaluationError> {
                if let Cashflow::FloatingRateCoupon(frcf) = cf {
                    let id = frcf.registry_id().ok_or(EvaluationError::NoRegistryId)?;
                    let cf_market_data = self
                        .market_data
                        .get(id)
                        .ok_or(EvaluationError::NoMarketData)?;
                    let fixing_rate = cf_market_data.fwd().ok_or(EvaluationError::NoFixingRate)?;
                    frcf.set_fixing_rate(fixing_rate);
                }
                Ok(())
            },
        )?;
        Ok(())
    }
}
