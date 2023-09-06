use std::{ops::Deref, rc::Rc};

use argmin::{
    core::{CostFunction, Error, Executor},
    solver::brent::BrentRoot,
};

use crate::{
    core::{meta::MarketData, traits::Registrable},
    instruments::{fixedrateinstrument::FixedRateInstrument, makefixedrateloan::MakeFixedRateLoan},
};

use super::{
    npvconstvisitor::NPVConstVisitor,
    traits::{ConstVisit, HasCashflows},
};

/// # ParValue
/// ParValue is a cost function that calculates the NPV of a generic instrument.
struct ParValue<T> {
    eval: Rc<T>,
    npv_visitor: Box<NPVConstVisitor>,
}

impl<T> ParValue<T> {
    pub fn new(eval: Rc<T>, market_data: Rc<Vec<MarketData>>) -> Self {
        let npv_visitor = NPVConstVisitor::new(market_data);
        ParValue {
            eval,
            npv_visitor: Box::new(npv_visitor),
        }
    }
}

impl CostFunction for ParValue<FixedRateInstrument> {
    type Param = f64;
    type Output = f64;

    fn cost(&self, param: &Self::Param) -> Result<Self::Output, Error> {
        let builder = MakeFixedRateLoan::from(self.eval.deref());
        let mut inst = builder.with_rate_value(*param).build();
        inst.mut_cashflows()
            .iter_mut()
            .zip(self.eval.cashflows().iter())
            .for_each(|(new_cf, old_cf)| {
                let id = old_cf.registry_id().expect("Cashflow has no registry id");
                new_cf.register_id(id);
            });

        Ok(self.npv_visitor.visit(&inst))
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

impl ConstVisit<FixedRateInstrument, f64> for ParValueConstVisitor {
    type Output = f64;
    fn visit(&self, instrument: &FixedRateInstrument) -> f64 {
        let cost = ParValue::new(Rc::new(instrument.clone()), self.market_data.clone());
        let solver = BrentRoot::new(-1.0, 1.0, 1e-6);
        let init_param = 0.05;
        let res = Executor::new(cost, solver)
            .configure(|state| state.param(init_param).max_iters(100).target_cost(0.0))
            .run()
            .expect("Solver failed");

        res.state().best_param.expect("No best parameter found")
    }
}
