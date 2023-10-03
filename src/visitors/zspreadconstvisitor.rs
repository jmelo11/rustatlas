use std::{ops::Deref, rc::Rc};

use argmin::{
    core::{CostFunction, Error, Executor, State},
    solver::brent::BrentRoot,
};

use crate::{
    cashflows::{cashflow::Cashflow, traits::Payable},
    core::{meta::MarketData, traits::Registrable},
    rates::interestrate::{InterestRate, RateDefinition},
};

use super::traits::{ConstVisit, EvaluationError, HasCashflows};

struct ZSpreadConstVisitor {
    market_data: Rc<Vec<MarketData>>,
    rate_definition: RateDefinition,
    target: f64,
}

impl ZSpreadConstVisitor {
    pub fn new(
        market_data: Rc<Vec<MarketData>>,
        rate_definition: RateDefinition,
        target: f64,
    ) -> Self {
        ZSpreadConstVisitor {
            market_data,
            rate_definition,
            target,
        }
    }
}

struct SpreadedNPV<'a, T> {
    eval: &'a T,
    market_data: &'a [MarketData],
    rate_definition: RateDefinition,
    target: f64,
}

impl<'a, T> SpreadedNPV<'a, T>
where
    T: HasCashflows,
{
    fn cashflow_npv(&self, cf: &Cashflow, z: f64) -> Result<f64, EvaluationError> {
        let id = cf.registry_id().ok_or(EvaluationError::NoRegistryId)?;
        let data = self
            .market_data
            .get(id)
            .ok_or(EvaluationError::NoMarketData)?;

        let df = data.df().ok_or(EvaluationError::NoDiscountFactor)?;
        let t = self
            .rate_definition
            .day_counter()
            .year_fraction(data.reference_date(), cf.payment_date());
        let r = InterestRate::implied_rate(
            1.0 / df,
            self.rate_definition.day_counter(),
            self.rate_definition.compounding(),
            self.rate_definition.frequency(),
            t,
        )?;

        match cf {
            Cashflow::FixedRateCoupon(coupon) => {
                let cf = coupon.amount().ok_or(EvaluationError::NoAmount)?
                    / r.compound_factor_from_yf(t);
                Ok(cf)
            }
            Cashflow::FloatingRateCoupon(coupon) => {
                let cf = coupon.amount().ok_or(EvaluationError::NoAmount)?
                    / r.compound_factor_from_yf(t);
                Ok(cf)
            }
            _ => Ok(0.0),
        }
    }
}

impl<'a, T> CostFunction for SpreadedNPV<'a, T>
where
    T: HasCashflows,
{
    type Param = f64;
    type Output = f64;

    fn cost(&self, param: &Self::Param) -> Result<Self::Output, Error> {
        let npv = self.eval.cashflows().iter().try_fold(
            0.0,
            |acc, cf| -> Result<f64, EvaluationError> {
                let cf_npv = self.cashflow_npv(cf, *param)?;
                Ok(acc + cf_npv)
            },
        )?;
        Ok(self.target - npv)
    }
}

impl<T> ConstVisit<T> for ZSpreadConstVisitor
where
    T: HasCashflows,
{
    type Output = Result<f64, Error>;

    fn visit(&self, visitable: &T) -> Self::Output {
        let npv = SpreadedNPV {
            eval: visitable,
            market_data: self.market_data.deref(),
            rate_definition: self.rate_definition,
            target: self.target,
        };
        let init_param = 0.05;
        let solver = BrentRoot::new(-1.0, 1.0, 1e-6);
        let res = Executor::new(npv, solver)
            .configure(|state| state.param(init_param).max_iters(100).target_cost(0.0))
            .run()?;
        Ok(*res.state().get_best_param().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::{
        cashflows::cashflow::Side,
        core::marketstore::MarketStore,
        currencies::enums::Currency,
        instruments::makefixedrateloan::MakeFixedRateLoan,
        models::{simplemodel::SimpleModel, traits::Model},
        rates::{
            enums::Compounding,
            interestrate::{InterestRate, RateDefinition},
            interestrateindex::{enums::InterestRateIndex, iborindex::IborIndex},
            traits::HasReferenceDate,
            yieldtermstructure::{
                enums::YieldTermStructure, flatforwardtermstructure::FlatForwardTermStructure,
            },
        },
        time::{
            date::Date,
            daycounter::DayCounter,
            enums::{Frequency, TimeUnit},
            period::Period,
        },
        visitors::{
            indexingvisitor::IndexingVisitor,
            traits::{ConstVisit, Visit},
        },
    };

    use super::ZSpreadConstVisitor;

    #[allow(dead_code)]
    pub fn create_store() -> MarketStore {
        let ref_date = Date::new(2021, 9, 1);
        let local_currency = Currency::USD;
        let mut market_store = MarketStore::new(ref_date, local_currency);

        let discount_rate = InterestRate::new(
            0.05,
            Compounding::Simple,
            Frequency::Annual,
            DayCounter::Actual360,
        );

        let discount_curve =
            YieldTermStructure::FlatForward(FlatForwardTermStructure::new(ref_date, discount_rate));

        let ibor_index = IborIndex::new(discount_curve.reference_date())
            .with_term_structure(discount_curve)
            .with_frequency(Frequency::Annual);

        market_store.mut_index_store().add_index(
            "DiscountCurve".to_string(),
            InterestRateIndex::IborIndex(ibor_index),
        );
        return market_store;
    }

    #[test]
    fn test() {
        let start_date = Date::new(2020, 1, 1);
        let end_date = start_date + Period::new(1, TimeUnit::Years);

        let rate = InterestRate::new(
            0.05,
            Compounding::Simple,
            Frequency::Annual,
            DayCounter::Actual360,
        );
        let mut instrument = MakeFixedRateLoan::new()
            .with_start_date(start_date)
            .with_end_date(end_date)
            .with_rate(rate)
            .with_notional(100.0)
            .with_side(Side::Receive)
            .with_currency(Currency::USD)
            .zero()
            .build()
            .unwrap();

        let market_store = Rc::new(create_store());
        let indexer = IndexingVisitor::new();
        let _ = indexer.visit(&mut instrument);

        let model = SimpleModel::new(market_store);

        let data = model.gen_market_data(&indexer.request()).unwrap();
        let ref_data = Rc::new(data);

        let zspread_visitor =
            ZSpreadConstVisitor::new(ref_data.clone(), RateDefinition::default(), 100.0);

        let zspread = zspread_visitor.visit(&instrument).unwrap();
        println!("ZSpread: {}", zspread);
    }
}
