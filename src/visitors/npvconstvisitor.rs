use crate::{
    cashflows::{cashflow::Side, traits::Payable},
    core::{meta::MarketData, traits::Registrable},
    utils::errors::{AtlasError, Result},
};

use super::traits::{ConstVisit, HasCashflows};

/// # NPVConstVisitor
/// NPVConstVisitor is a visitor that calculates the NPV of an instrument.
/// It assumes that the cashflows of the instrument have already been indexed and fixed.
///
/// ## Parameters
/// * `market_data` - The market data to use for NPV calculation
/// * `include_today_cashflows` - Flag to include cashflows with payment date equal to the reference date
pub struct NPVConstVisitor<'a> {
    market_data: &'a [MarketData],
    include_today_cashflows: bool,
}

impl<'a> NPVConstVisitor<'a> {
    pub fn new(market_data: &'a [MarketData], include_today_cashflows: bool) -> Self {
        NPVConstVisitor {
            market_data: market_data,
            include_today_cashflows,
        }
    }
    pub fn set_include_today_cashflows(&mut self, include_today_cashflows: bool) {
        self.include_today_cashflows = include_today_cashflows;
    }
}

impl<'a, T: HasCashflows> ConstVisit<T> for NPVConstVisitor<'a> {
    type Output = Result<f64>;
    fn visit(&self, visitable: &T) -> Self::Output {
        let npv = visitable.cashflows().iter().try_fold(0.0, |acc, cf| {
            let id = cf.id()?;

            let cf_market_data =
                self.market_data
                    .get(id)
                    .ok_or(AtlasError::NotFoundErr(format!(
                        "Market data for cashflow with id {}",
                        id
                    )))?;

            if cf_market_data.reference_date() == cf.payment_date() && !self.include_today_cashflows
                || cf.payment_date() < cf_market_data.reference_date()
            {
                return Ok(acc);
            }

            let df = cf_market_data.df()?;
            let fx = cf_market_data.fx()?;
            let flag = match cf.side() {
                Side::Pay => -1.0,
                Side::Receive => 1.0,
            };
            let numerarie = cf_market_data.numerarie();
            let amount = cf.amount()?;
            Ok(acc + df * amount / fx * flag / numerarie)
        });
        return npv;
    }
}

#[cfg(test)]
mod tests {
    use std::{
        collections::HashMap,
        sync::{Arc, RwLock},
    };

    use rayon::{
        prelude::{IntoParallelIterator, ParallelIterator},
        slice::ParallelSliceMut,
    };

    use crate::{
        core::marketstore::MarketStore,
        currencies::enums::Currency,
        instruments::{
            fixedrateinstrument::FixedRateInstrument, makefixedrateinstrument::MakeFixedRateInstrument, makefloatingrateinstrument::MakeFloatingRateInstrument
        },
        models::{simplemodel::SimpleModel, traits::Model},
        rates::{
            enums::Compounding,
            interestrate::{InterestRate, RateDefinition},
            interestrateindex::{iborindex::IborIndex, overnightindex::OvernightIndex},
            traits::HasReferenceDate,
            yieldtermstructure::flatforwardtermstructure::FlatForwardTermStructure,
        },
        time::{
            date::Date, daycounter::DayCounter, enums::{Frequency, TimeUnit}, period::Period
        },
        visitors::{fixingvisitor::FixingVisitor, indexingvisitor::IndexingVisitor, traits::Visit},
    };

    use super::*;

    pub fn create_store() -> Result<MarketStore> {
        let ref_date = Date::new(2021, 9, 1);
        let local_currency = Currency::USD;
        let mut market_store = MarketStore::new(ref_date, local_currency);

        let forecast_curve_1 = Arc::new(FlatForwardTermStructure::new(
            ref_date,
            0.02,
            RateDefinition::default(),
        ));

        let forecast_curve_2 = Arc::new(FlatForwardTermStructure::new(
            ref_date,
            0.03,
            RateDefinition::default(),
        ));

        let discount_curve = Arc::new(FlatForwardTermStructure::new(
            ref_date,
            0.05,
            RateDefinition::new(
                DayCounter::Thirty360,
                Compounding::Compounded,
                Frequency::Annual,
            )
        ));

        let mut ibor_fixings = HashMap::new();
        ibor_fixings.insert(Date::new(2021, 9, 1), 0.02); // today
        ibor_fixings.insert(Date::new(2021, 8, 31), 0.02); // yesterday

        let ibor_index = IborIndex::new(forecast_curve_1.reference_date())
            .with_fixings(ibor_fixings)
            .with_term_structure(forecast_curve_1)
            .with_frequency(Frequency::Annual);

        let overnight_fixings =
            make_fixings(ref_date - Period::new(1, TimeUnit::Years), ref_date, 0.06);
        let overnigth_index = OvernightIndex::new(forecast_curve_2.reference_date())
            .with_term_structure(forecast_curve_2)
            .with_fixings(overnight_fixings);

        market_store
            .mut_index_store()
            .add_index(0, Arc::new(RwLock::new(ibor_index)))?;

        market_store
            .mut_index_store()
            .add_index(1, Arc::new(RwLock::new(overnigth_index)))?;

        let discount_index =
            IborIndex::new(discount_curve.reference_date()).with_term_structure(discount_curve);

        market_store
            .mut_index_store()
            .add_index(2, Arc::new(RwLock::new(discount_index)))?;
        return Ok(market_store);
    }

    fn make_fixings(start: Date, end: Date, rate: f64) -> HashMap<Date, f64> {
        let mut fixings = HashMap::new();
        let mut seed = start;
        let mut init = 100.0;
        while seed <= end {
            fixings.insert(seed, init);
            seed = seed + Period::new(1, TimeUnit::Days);
            init = init * (1.0 + rate * 1.0 / 360.0);
        }
        return fixings;
    }

    #[test]
    fn test_npv_fixed_bullet() -> Result<()> {
        let market_store = create_store().unwrap();
        let ref_date = market_store.reference_date();

        let start_date = ref_date;
        let end_date = start_date + Period::new(10, TimeUnit::Years);
        let notional = 100_000.0;
        let rate = InterestRate::new(
            0.05,
            Compounding::Compounded,
            Frequency::Annual,
            DayCounter::Thirty360,
        );

        let mut instrument = MakeFixedRateInstrument::new()
            .with_start_date(start_date)
            .with_end_date(end_date)
            .with_rate(rate)
            .with_payment_frequency(Frequency::Semiannual)
            .with_side(Side::Receive)
            .with_currency(Currency::USD)
            .bullet()
            .with_discount_curve_id(Some(2))
            .with_notional(notional)
            .build()?;

        let indexer = IndexingVisitor::new();
        indexer.visit(&mut instrument)?;

        let model = SimpleModel::new(&market_store);
        let data = model.gen_market_data(&indexer.request())?;

        let npv_visitor = NPVConstVisitor::new(&data, true);
        let npv = npv_visitor.visit(&instrument)?;

        assert!(npv.abs() < 1e-6);

        Ok(())
    }

    #[test]
    fn test_npv_fixed_bullet_negative_rate() -> Result<()> {
        let market_store = create_store().unwrap();
        let ref_date = market_store.reference_date();

        let start_date = ref_date;
        let end_date = start_date + Period::new(10, TimeUnit::Years);
        let notional = 100_000.0;
        let rate = InterestRate::new(
            -0.05,
            Compounding::Compounded,
            Frequency::Annual,
            DayCounter::Thirty360,
        );

        let mut instrument = MakeFixedRateInstrument::new()
            .with_start_date(start_date)
            .with_end_date(end_date)
            .with_rate(rate)
            .with_payment_frequency(Frequency::Semiannual)
            .with_side(Side::Receive)
            .with_currency(Currency::USD)
            .bullet()
            .with_discount_curve_id(Some(2))
            .with_notional(notional)
            .build()?;

        let indexer = IndexingVisitor::new();
        indexer.visit(&mut instrument)?;

        let model = SimpleModel::new(&market_store);
        let data = model.gen_market_data(&indexer.request())?;

        let npv_visitor = NPVConstVisitor::new(&data, true);
        let npv = npv_visitor.visit(&instrument)?;
        
        assert!(npv.abs() > 70000.0);
        Ok(())
    }

    #[test]
    fn test_npv_floating_bullet() -> Result<()> {
        let market_store = create_store().unwrap();
        let ref_date = market_store.reference_date();

        let start_date = ref_date;
        let end_date = start_date + Period::new(10, TimeUnit::Years);
        let notional = 100_000.0;
        let rate_definition = RateDefinition::new(
            DayCounter::Thirty360,
            Compounding::Compounded,
            Frequency::Annual,
        );

        let mut instrument = MakeFloatingRateInstrument::new()
            .with_start_date(start_date)
            .with_end_date(end_date)
            .with_rate_definition(rate_definition)
            .with_payment_frequency(Frequency::Semiannual)
            .with_side(Side::Receive)
            .with_currency(Currency::USD)
            .with_spread(0.0)
            .bullet()
            .with_discount_curve_id(Some(0))
            .with_forecast_curve_id(Some(0))
            .with_notional(notional)
            .build()?;

        let indexer = IndexingVisitor::new();
        indexer.visit(&mut instrument)?;

        let model = SimpleModel::new(&market_store);
        let data = model.gen_market_data(&indexer.request())?;

        let fixing_visitor = FixingVisitor::new(&data);
        fixing_visitor.visit(&mut instrument)?;

        let npv_visitor = NPVConstVisitor::new(&data, true);
        let npv = npv_visitor.visit(&instrument)?;

        assert_ne!(npv, 0.0);
        Ok(())
    }

    #[test]
    fn test_npv_fixed_equal_payment() -> Result<()> {
        let market_store = create_store().unwrap();
        let ref_date = market_store.reference_date();

        let start_date = ref_date;
        let end_date = start_date + Period::new(10, TimeUnit::Years);
        let notional = 100_000.0;
        let rate = InterestRate::new(
            0.05,
            Compounding::Compounded,
            Frequency::Annual,
            DayCounter::Thirty360
        );

        let mut instrument = MakeFixedRateInstrument::new()
                .with_start_date(start_date)
                .with_end_date(end_date)
                .with_rate(rate)
                .with_payment_frequency(Frequency::Semiannual)
                .with_side(Side::Receive)
                .with_currency(Currency::USD)
                .with_discount_curve_id(Some(2))
                .with_notional(notional)
                .equal_payments()
                .build()?;


        let builder = MakeFixedRateInstrument::from(&instrument.clone());
        let mut instrument_rebuilt = builder.build()?;
                    
        let indexer = IndexingVisitor::new();
        indexer.visit(&mut instrument)?;
        indexer.visit(&mut instrument_rebuilt)?;

        let model = SimpleModel::new(&market_store);
        let data = model.gen_market_data(&indexer.request())?;

        let npv_visitor = NPVConstVisitor::new(&data, true);
        let npv = npv_visitor.visit(&instrument)?;
        let npv_rebuilt = npv_visitor.visit(&instrument_rebuilt)?;

        assert!(npv.abs() < 1e-6);
        assert!(npv_rebuilt.abs() < 1e-6);
        
        Ok(())  
    }

    #[test]
    fn generator_tests() -> Result<()> {
        let market_store = create_store().unwrap();
        let ref_date = market_store.reference_date();

        let start_date = ref_date;
        let end_date = start_date + Period::new(10, TimeUnit::Years);
        let notional = 100_000.0;
        let rate = InterestRate::new(
            0.05,
            Compounding::Simple,
            Frequency::Annual,
            DayCounter::Thirty360,
        );

        // par build
        let mut instruments: Vec<FixedRateInstrument> = (0..150000)
            .into_par_iter() // Create a parallel iterator
            .map(|_| {
                MakeFixedRateInstrument::new()
                    .with_start_date(start_date.clone()) // clone data if needed
                    .with_end_date(end_date.clone()) // clone data if needed
                    .with_rate(rate)
                    .with_payment_frequency(Frequency::Semiannual)
                    .with_side(Side::Receive)
                    .with_currency(Currency::USD)
                    .bullet()
                    .with_discount_curve_id(Some(2))
                    .with_notional(notional)
                    .build()
                    .unwrap()
            })
            .collect(); // Collect the results into a Vec<_>

        fn npv(instruments: &mut [FixedRateInstrument]) -> f64 {
            let store = create_store().unwrap();
            let mut npv = 0.0;
            let indexer = IndexingVisitor::new();
            instruments
                .iter_mut()
                .for_each(|inst| indexer.visit(inst).unwrap());

            let model = SimpleModel::new(&store);
            let data = model.gen_market_data(&indexer.request()).unwrap();

            let npv_visitor = NPVConstVisitor::new(&data, true);
            instruments
                .iter()
                .for_each(|inst| npv += npv_visitor.visit(inst).unwrap());
            npv
        }

        instruments.par_rchunks_mut(1000).for_each(|chunk| {
            npv(chunk);
        });

        Ok(())
    }
}
