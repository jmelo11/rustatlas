use std::rc::Rc;

use argmin::{
    core::{CostFunction, Error, Executor},
    solver::brent::BrentRoot,
};

use crate::{
    core::{meta::MarketData, traits::Registrable},
    instruments::{fixedrateinstrument::FixedRateInstrument, makefixedrateloan::MakeFixedRateLoan},
    rates::interestrate::InterestRate,
};

use super::{
    npvconstvisitor::NPVConstVisitor,
    traits::{ConstVisit, HasCashflows},
};

struct FixedRateParValue {
    instrument: Rc<FixedRateInstrument>,
    npv_visitor: Box<NPVConstVisitor>,
}

impl CostFunction for FixedRateParValue {
    type Param = f64;
    type Output = f64;

    fn cost(&self, param: &Self::Param) -> Result<Self::Output, Error> {
        let rate = self.instrument.rate();
        let builder: MakeFixedRateLoan = self.instrument.as_ref().clone().into();
        let new_rate = InterestRate::new(
            *param,
            rate.compounding(),
            rate.frequency(),
            rate.day_counter(),
        );
        let mut new_instrument = builder.with_rate(new_rate).build();
        new_instrument
            .mut_cashflows()
            .iter_mut()
            .zip(self.instrument.cashflows().iter())
            .for_each(|(new_cf, old_cf)| {
                let id = old_cf.registry_id().expect("Cashflow has no registry id");
                new_cf.register_id(id);
            });

        Ok(self.npv_visitor.visit(&new_instrument))
    }
}

// impl CostFunction for FloatingRateParValue {
//     type Param = f64;
//     type Output = f64;

//     fn cost(&self, param: &Self::Param) -> Result<Self::Output, Error> {
//         let builder: MakeFloatingRateLoan = self.instrument.as_ref().clone().into();
//         let mut new_instrument = builder.with_spread(param).build();
//         new_instrument
//             .mut_cashflows()
//             .iter_mut()
//             .zip(self.instrument.cashflows().iter())
//             .for_each(|(new_cf, old_cf)| {
//                 let id = old_cf.registry_id().expect("Cashflow has no registry id");
//                 new_cf.register_id(id);
//             });

//         Ok(self.npv_visitor.visit(&new_instrument))
//     }
// }

pub struct ParValueConstVisitor {
    market_data: Rc<Vec<MarketData>>,
}

impl ParValueConstVisitor {
    pub fn new(market_data: Rc<Vec<MarketData>>) -> Self {
        ParValueConstVisitor {
            market_data: market_data,
        }
    }
}

impl ConstVisit<FixedRateInstrument, f64> for ParValueConstVisitor {
    type Output = f64;
    fn visit(&self, instrument: &FixedRateInstrument) -> f64 {
        let npv_visitor = NPVConstVisitor::new(self.market_data.clone());
        let cost = FixedRateParValue {
            instrument: Rc::new(instrument.clone()),
            npv_visitor: Box::new(npv_visitor),
        };
        let solver = BrentRoot::new(-1.0, 1.0, 1e-6);
        let init_param = 0.05;
        let res = Executor::new(cost, solver)
            .configure(|state| state.param(init_param).max_iters(100).target_cost(0.0))
            .run()
            .expect("Solver failed");

        res.state().best_param.expect("No best parameter found")
    }
}
