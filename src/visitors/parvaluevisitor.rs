use std::{ops::Deref, rc::Rc};

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
};

use super::{
    fixingvisitor::FixingVisitor,
    npvconstvisitor::NPVConstVisitor,
    traits::{ConstVisit, EvaluationError, HasCashflows, Visit},
};

/// # ParValue
/// ParValue is a cost function that calculates the NPV of a generic instrument.
struct ParValue<T> {
    eval: Rc<T>,
    npv_visitor: Box<NPVConstVisitor>,
    fixing_visitor: Box<FixingVisitor>,
}

impl<T> ParValue<T> {
    pub fn new(eval: Rc<T>, market_data: Rc<Vec<MarketData>>) -> Self {
        let npv_visitor = NPVConstVisitor::new(market_data.clone(), true);
        let fixing_visitor = FixingVisitor::new(market_data.clone());
        ParValue {
            eval,
            npv_visitor: Box::new(npv_visitor),
            fixing_visitor: Box::new(fixing_visitor),
        }
    }
}

impl CostFunction for ParValue<FixedRateInstrument> {
    type Param = f64;
    type Output = f64;

    fn cost(&self, param: &Self::Param) -> Result<Self::Output, Error> {
        let builder = MakeFixedRateLoan::from(self.eval.deref());
        let mut inst = builder.with_rate_value(*param).build()?;
        inst.mut_cashflows()
            .iter_mut()
            .zip(self.eval.cashflows().iter())
            .try_for_each(|(cf, old_cf)| -> Result<(), EvaluationError> {
                let id = old_cf.registry_id().ok_or(EvaluationError::NoRegistryId)?;
                cf.register_id(id);
                Ok(())
            })?;

        self.npv_visitor.visit(&inst).map_err(|e| Error::from(e))
    }
}

impl CostFunction for ParValue<FloatingRateInstrument> {
    type Param = f64;
    type Output = f64;

    fn cost(&self, param: &Self::Param) -> Result<Self::Output, Error> {
        let builder = MakeFloatingRateLoan::from(self.eval.deref());
        let mut inst = builder.with_spread(*param).build()?;

        inst.mut_cashflows()
            .iter_mut()
            .zip(self.eval.cashflows().iter())
            .try_for_each(|(cf, old_cf)| -> Result<(), EvaluationError> {
                let id = old_cf.registry_id().ok_or(EvaluationError::NoRegistryId)?;
                cf.register_id(id);
                Ok(())
            })?;

        let _ = self.fixing_visitor.visit(&mut inst);
        self.npv_visitor.visit(&inst).map_err(|e| Error::from(e))
    }
}

/// # ParValueConstVisitor
/// ParValueConstVisitor is a visitor that calculates the par rate/spread of.
pub struct ParValueConstVisitor {
    market_data: Rc<Vec<MarketData>>,
}

impl ParValueConstVisitor {
    pub fn new(market_data: Rc<Vec<MarketData>>) -> Self {
        ParValueConstVisitor { market_data }
    }
}

impl ConstVisit<FixedRateInstrument> for ParValueConstVisitor {
    type Output = Result<f64, Error>;
    fn visit(&self, instrument: &FixedRateInstrument) -> Self::Output {
        let cost = ParValue::new(Rc::new(instrument.clone()), self.market_data.clone());
        let solver = BrentRoot::new(-1.0, 1.0, 1e-6);
        let init_param = 0.05;
        let res = Executor::new(cost, solver)
            .configure(|state| state.param(init_param).max_iters(100).target_cost(0.0))
            .run()?;

        Ok(*res.state().get_best_param().unwrap())
    }
}

impl ConstVisit<FloatingRateInstrument> for ParValueConstVisitor {
    type Output = Result<f64, Error>;
    fn visit(&self, instrument: &FloatingRateInstrument) -> Self::Output {
        let cost = ParValue::new(Rc::new(instrument.clone()), self.market_data.clone());
        let solver = BrentRoot::new(-1.0, 1.0, 1e-6);
        let init_param = 0.05;
        let res = Executor::new(cost, solver)
            .configure(|state| state.param(init_param).max_iters(100).target_cost(0.0))
            .run()?;

        Ok(*res.state().get_best_param().unwrap())
    }
}
