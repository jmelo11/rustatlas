use std::rc::Rc;

use argmin::core::{CostFunction, Error};

use crate::{
    core::traits::Registrable,
    instruments::{fixedrateinstrument::FixedRateInstrument, makefixedrateloan::MakeFixedRateLoan},
    rates::interestrate::InterestRate,
};

use super::{
    npvconstvisitor::NPVConstVisitor,
    traits::{ConstVisit, HasCashflows},
};

struct ParValue {
    instrument: Rc<FixedRateInstrument>,
    npv_visitor: Box<NPVConstVisitor>,
}

impl CostFunction for ParValue {
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
