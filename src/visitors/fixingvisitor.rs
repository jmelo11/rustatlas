use crate::{
    cashflows::{cashflow::Cashflow, traits::RequiresFixingRate},
    core::{meta::MarketData, traits::Registrable},
    utils::errors::{AtlasError, Result},
};

use super::traits::{HasCashflows, Visit};

/// # FixingVisitor
/// FixingVisitor is a visitor that fixes the rate of a floating rate cashflow.
///
/// ## Parameters
/// * `market_data` - The market data to use for fixing
pub struct FixingVisitor<'a> {
    market_data: &'a [MarketData],
}

impl<'a> FixingVisitor<'a> {
    pub fn new(market_data: &'a [MarketData]) -> Self {
        FixingVisitor {
            market_data: market_data,
        }
    }
}

impl<'a, T: HasCashflows> Visit<T> for FixingVisitor<'a> {
    type Output = Result<()>;
    fn visit(&self, has_cashflows: &mut T) -> Self::Output {
        has_cashflows
            .mut_cashflows()
            .iter_mut()
            .try_for_each(|cf| -> Result<()> {
                if let Cashflow::FloatingRateCoupon(frcf) = cf {
                    let id = frcf.id()?;
                    let cf_market_data =
                        self.market_data
                            .get(id)
                            .ok_or(AtlasError::NotFoundErr(format!(
                                "Market data for cashflow with id {}",
                                id
                            )))?;
                    let fixing_rate = cf_market_data.fwd()?;
                    frcf.set_fixing_rate(fixing_rate);
                }
                Ok(())
            })?;
        Ok(())
    }
}
