use argmin::{
    core::{CostFunction, Error, Executor, State},
    solver::brent::BrentRoot,
};

use crate::{
    core::{meta::MarketData, traits::Registrable},
    instruments::{
        fixedrateinstrument::FixedRateInstrument, floatingrateinstrument::FloatingRateInstrument,
        makefixedrateloan::MakeFixedRateLoan, makefloatingrateloan::MakeFloatingRateLoan,
    },
    utils::errors::Result,
};

use super::{
    fixingvisitor::FixingVisitor,
    npvconstvisitor::NPVConstVisitor,
    traits::{ConstVisit, HasCashflows, Visit},
};

/// # ParValue
/// ParValue is a cost function that calculates the NPV of a generic instrument.
struct ParValue<'a, T> {
    eval: &'a T,
    npv_visitor: Box<NPVConstVisitor<'a>>,
    fixing_visitor: Box<FixingVisitor<'a>>,
}

impl<'a, T> ParValue<'a, T> {
    pub fn new(eval: &'a T, market_data: &'a [MarketData]) -> Self {
        let npv_visitor = NPVConstVisitor::new(market_data, true);
        let fixing_visitor = FixingVisitor::new(market_data);
        ParValue {
            eval,
            npv_visitor: Box::new(npv_visitor),
            fixing_visitor: Box::new(fixing_visitor),
        }
    }
}

impl<'a> CostFunction for ParValue<'a, FixedRateInstrument> {
    type Param = f64;
    type Output = f64;

    fn cost(&self, param: &Self::Param) -> std::result::Result<Self::Output, Error> {
        let builder = MakeFixedRateLoan::from(self.eval);
        let mut inst = builder.with_rate_value(*param).build()?;
        inst.mut_cashflows()
            .iter_mut()
            .zip(self.eval.cashflows().iter())
            .try_for_each(|(cf, old_cf)| -> Result<()> {
                let id = old_cf.id()?;
                cf.set_id(id);
                Ok(())
            })?;

        self.npv_visitor.visit(&inst).map_err(|e| Error::from(e))
    }
}

impl<'a> CostFunction for ParValue<'a, FloatingRateInstrument> {
    type Param = f64;
    type Output = f64;

    fn cost(&self, param: &Self::Param) -> std::result::Result<Self::Output, Error> {
        let builder = MakeFloatingRateLoan::from(self.eval);
        let mut inst = builder.with_spread(*param).build()?;

        inst.mut_cashflows()
            .iter_mut()
            .zip(self.eval.cashflows().iter())
            .try_for_each(|(cf, old_cf)| -> Result<()> {
                let id = old_cf.id()?;
                cf.set_id(id);
                Ok(())
            })?;

        let _ = self.fixing_visitor.visit(&mut inst);
        self.npv_visitor.visit(&inst).map_err(|e| Error::from(e))
    }
}

/// # ParValueConstVisitor
/// ParValueConstVisitor is a visitor that calculates the par rate/spread of.
pub struct ParValueConstVisitor<'a> {
    market_data: &'a [MarketData],
}

impl<'a> ParValueConstVisitor<'a> {
    pub fn new(market_data: &'a [MarketData]) -> Self {
        ParValueConstVisitor { market_data }
    }
}

impl<'a> ConstVisit<FixedRateInstrument> for ParValueConstVisitor<'a> {
    type Output = Result<f64>;
    fn visit(&self, instrument: &FixedRateInstrument) -> Self::Output {
        let cost = ParValue::new(instrument, &self.market_data);
        let solver = BrentRoot::new(-1.0, 1.0, 1e-6);
        let res = Executor::new(cost, solver)
            .configure(|state| state.max_iters(100).target_cost(0.0))
            .run()?;

        Ok(*res.state().get_best_param().unwrap())
    }
}

impl<'a> ConstVisit<FloatingRateInstrument> for ParValueConstVisitor<'a> {
    type Output = Result<f64>;
    fn visit(&self, instrument: &FloatingRateInstrument) -> Self::Output {
        let cost = ParValue::new(instrument, &self.market_data);
        let solver = BrentRoot::new(-1.0, 1.0, 1e-6);
        let res = Executor::new(cost, solver)
            .configure(|state| state.max_iters(100).target_cost(0.0))
            .run()?;

        Ok(*res.state().get_best_param().unwrap())
    }
}
