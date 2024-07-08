use std::collections::BTreeMap;

use super::positiongenerator::{PositionGenerator, RolloverStrategy};
use crate::{
    core::marketstore::MarketStore, currencies::enums::Currency, instruments::instrument::Instrument, models::{simplemodel::SimpleModel, traits::Model}, rates::traits::HasReferenceDate, time::{
        calendar::Calendar, calendars::nullcalendar::NullCalendar, date::Date, daycounters::{actual360::Actual360, traits::DayCountProvider}, enums::{BusinessDayConvention, TimeUnit}, period::Period, schedule::MakeSchedule
    }, utils::errors::Result, visitors::{
        cashflowaggregationvisitor::CashflowsAggregatorConstVisitor,
        fixingvisitor::FixingVisitor,
        indexingvisitor::IndexingVisitor,
        traits::{ConstVisit, Visit},
    }
};

/// # RolloverSimulationEngine
/// Engine is the main component of the simulation. It is responsible for:
/// - generating the new positions
/// - indexing the new positions and getting the relevant market data
/// - aggregating the new redemptions to the portfolio redemptions
///
/// ## Parameters
/// * `market_store` - A reference to a market store
/// * `base_redemptions` - A map of redemptions for the base portfolio
/// * `redemption_currency` - The currency of the redemptions
/// * `horizon` - The horizon of the simulation
///
/// ## Details
/// - Requires redemptions and the currency of the redemptions

pub enum GrowthMode{
    Annual,
    PaidAmount,
}

pub struct RolloverSimulationEngine<'a> {
    market_store: &'a MarketStore,
    base_redemptions: BTreeMap<Date, f64>,
    redemptions_currency: Currency,
    eval_dates: Vec<Date>,
    growth_mode: GrowthMode,
    growth_rate: f64,
}

impl<'a> RolloverSimulationEngine<'a> {
    pub fn new(
        market_store: &'a MarketStore,
        base_redemptions: BTreeMap<Date, f64>,
        redemption_currency: Currency,
        horizon: Period,
    ) -> Self {
        let schedule = MakeSchedule::new(
            market_store.reference_date(),
            market_store.reference_date() + horizon,
        )
        .with_tenor(Period::new(1, TimeUnit::Days))
        .with_calendar(Calendar::NullCalendar(NullCalendar::new()))
        .with_convention(BusinessDayConvention::Unadjusted)
        .build()
        .unwrap();

        Self {
            market_store,
            base_redemptions,
            redemptions_currency: redemption_currency,
            eval_dates: schedule.dates().clone(),
            growth_mode: GrowthMode::PaidAmount,
            growth_rate: 0.0,
        }
    }

    pub fn with_growth_mode(mut self, mode: GrowthMode) -> Self {
        self.growth_mode = mode;
        self
    } 

    pub fn with_growth_rate(mut self, rate: f64) -> Self {
        self.growth_rate = rate;
        self
    }

    pub fn run(&self, strategies: Vec<RolloverStrategy>) -> Result<Vec<Instrument>> {
        let mut redemptions = self.base_redemptions.clone(); // redemptions for target portfolio
        
        let outstanding_0 = self.base_redemptions.clone().iter().fold(0.0, |acc, (_, value)| acc + value); // total outstanding amount
        let mut outstanding = outstanding_0; 
        let first_date = self.eval_dates.first().unwrap();

        // vector of new positions
        let mut simulated_instruments = Vec::new();

        // redemptions for target portfolio
        let generator = PositionGenerator::new(self.redemptions_currency, strategies.clone());
        self.eval_dates.iter().try_for_each(|date| -> Result<()> {
            let maturing_amount = redemptions.get(date);

            let redemption = match maturing_amount {
                Some(amount) => *amount, 
                None => 0.0
            };
            let amount = match self.growth_mode {
                GrowthMode::Annual => {
                    let delta_date = Actual360::year_fraction(*first_date, *date);
                    outstanding -= redemption;
                    let placement = outstanding_0* (1.0 + self.growth_rate * delta_date) - outstanding;
                    outstanding += placement;
                    placement
                }   
                GrowthMode::PaidAmount => {
                    let placement = redemption * (1.0 + self.growth_rate);
                    placement
                }
            };
            
            if amount != 0.0 {
                let amount_abs = amount.abs();

                // relevant data for new positions
                let tmp_store = if self.market_store.reference_date() != *date {
                    self.market_store.advance_to_date(*date)?
                } else {
                    self.market_store.clone()
                };

                let new_generator = generator
                    .clone()
                    .with_market_store(&tmp_store)
                    .with_amount(amount_abs);

                // generate positions
                let mut positions = new_generator.generate();

                // indexing
                let indexing_visitor = IndexingVisitor::new();
                positions.iter_mut().try_for_each(|inst| -> Result<()> {
                    indexing_visitor.visit(inst)?;
                    Ok(())
                })?;

                // market data for new positions
                let model = SimpleModel::new(&tmp_store);
                let data = model.gen_market_data(&indexing_visitor.request())?;

                // fixing for new positions
                let fixing_visitor = FixingVisitor::new(&data);
                positions.iter_mut().try_for_each(|inst| -> Result<()> {
                    fixing_visitor.visit(inst)?;
                    Ok(())
                })?;

                // add new positions to the vector
                simulated_instruments.append(&mut positions.clone());

                // add new redemptions to the vector
                let aggregator = CashflowsAggregatorConstVisitor::new()
                    .with_validate_currency(self.redemptions_currency);
                positions.iter().try_for_each(|inst| -> Result<()> {
                    aggregator.visit(inst)?;
                    Ok(())
                })?;

                let new_redemptions = aggregator.redemptions();

                //println!("New redemptions: {:?}", new_redemptions);
                for (key, value) in new_redemptions {
                    let entry = redemptions.entry(key).or_insert(0.0);
                    *entry += value;
                }
            }

            Ok(())
        })?;

        Ok(simulated_instruments)
    }
}

#[cfg(test)]
mod tests {

    use std::sync::{Arc, RwLock};

    use crate::{
        cashflows::{cashflow::{Cashflow, Side}, traits::Payable}, currencies::enums::Currency, instruments::{instrument::RateType, traits::Structure}, math::interpolation::enums::Interpolator, rates::{
            interestrate::RateDefinition,
            interestrateindex::iborindex::IborIndex,
            yieldtermstructure::{
                compositetermstructure::CompositeTermStructure,
                discounttermstructure::DiscountTermStructure,
                flatforwardtermstructure::FlatForwardTermStructure,
                tenorbasedzeroratetermstructure::TenorBasedZeroRateTermStructure,
            },
        }, time::{daycounter::DayCounter, enums::Frequency}, visitors::traits::HasCashflows
    };

    use super::*;

    // function to get the outstanding amount at a given date -- Move to a library? 
    pub fn get_outstandings_at_date(instruments: &[Instrument], eval_date: Date) -> Result<f64> {
        let outstanding = instruments
            .iter()
            .map(|inst| {
                let mut local_sum = 0.0;
                inst.cashflows().iter().for_each(|cf| match cf {
                    Cashflow::Disbursement(f) => {
                        let payment_date = f.payment_date();
                        if payment_date <= eval_date {
                            local_sum += f.amount().unwrap()*f.side().sign();
                        }
                    }
                    Cashflow::Redemption(f) => {
                        let payment_date = f.payment_date(); 
                        if payment_date <= eval_date {
                            local_sum += f.amount().unwrap()*f.side().sign();
                        }
                    }
                    _ => {}
                });
                local_sum
            }).sum::<f64>();
        Ok(outstanding)    
    }

    fn create_store() -> Result<MarketStore> {
        let ref_date = Date::new(2021, 9, 1);
        let local_currency = Currency::USD;
        let mut market_store = MarketStore::new(ref_date, local_currency);

        let base_curve = Arc::new(FlatForwardTermStructure::new(
            ref_date,
            0.02,
            RateDefinition::default(),
        ));

        let spread_curve = Arc::new(TenorBasedZeroRateTermStructure::new(
            ref_date,
            vec![
                Period::new(1, TimeUnit::Days),
                Period::new(3, TimeUnit::Months),
                Period::new(6, TimeUnit::Months),
                Period::new(1, TimeUnit::Years),
            ],
            vec![0.01, 0.03, 0.04, 0.05],
            RateDefinition::default(),
            Interpolator::Linear,
            true,
        )?);

        // with this curve, we shouldn't see rate changes since the spread endsup being constant and the base is flatforward
        let composite_curve = Arc::new(CompositeTermStructure::new(spread_curve, base_curve));
        let composite_index =
            IborIndex::new(composite_curve.reference_date()).with_term_structure(composite_curve);

        market_store
            .mut_index_store()
            .add_index(0, Arc::new(RwLock::new(composite_index)))?;

        let discount_factors = vec![1.0, 0.99, 0.978, 0.956, 0.934];
        let dates = vec![
            ref_date,
            ref_date + Period::new(1, TimeUnit::Months),
            ref_date + Period::new(3, TimeUnit::Months),
            ref_date + Period::new(6, TimeUnit::Months),
            ref_date + Period::new(1, TimeUnit::Years),
        ];

        let spread_curve_2 = Arc::new(TenorBasedZeroRateTermStructure::new(
            ref_date,
            vec![
                Period::new(1, TimeUnit::Days),
                Period::new(3, TimeUnit::Months),
                Period::new(6, TimeUnit::Months),
                Period::new(1, TimeUnit::Years),
            ],
            vec![0.01, 0.03, 0.04, 0.05],
            RateDefinition::default(),
            Interpolator::Linear,
            true,
        )?);

        let discount_curve = Arc::new(DiscountTermStructure::new(
            dates,
            discount_factors,
            DayCounter::Actual360,
            Interpolator::Linear,
            true,
        )?);

        let composite_curve_2 =
            Arc::new(CompositeTermStructure::new(spread_curve_2, discount_curve));

        let composite_index_2 = IborIndex::new(composite_curve_2.reference_date())
            .with_term_structure(composite_curve_2);

        market_store
            .mut_index_store()
            .add_index(1, Arc::new(RwLock::new(composite_index_2)))?;

        return Ok(market_store);
    }

    #[test]
    fn test_rollover_simulation_engine() -> Result<()> {
        let market_store = create_store().unwrap();
        let horizon = Period::new(5, TimeUnit::Years);

        let base_redemptions = [
            (Date::new(2021, 9, 1), 100.0),
            (Date::new(2021, 10, 1), 100.0),
            (Date::new(2021, 11, 1), 100.0),
            (Date::new(2021, 12, 1), 100.0),
            (Date::new(2022, 1, 1), 150.0),
            (Date::new(2022, 2, 1), 150.0),
            (Date::new(2022, 3, 1), 150.0),
            (Date::new(2022, 4, 1), 150.0),
            (Date::new(2022, 5, 1), 200.0),
            (Date::new(2022, 6, 1), 200.0),
            (Date::new(2022, 7, 1), 200.0),
            (Date::new(2022, 8, 1), 200.0),
        ]
        .iter()
        .map(|&(date, value)| (date, value))
        .collect::<BTreeMap<_, _>>();

        let engine =
            RolloverSimulationEngine::new(&market_store, base_redemptions, Currency::USD, horizon);

        let strategies = vec![
            RolloverStrategy::new(
                0.5,
                Structure::Bullet,
                Frequency::Semiannual,
                Period::new(1, TimeUnit::Years),
                Side::Receive,
                RateType::Fixed,
                RateDefinition::default(),
                0,
                None),
            RolloverStrategy::new(
                0.5,
                Structure::Bullet,
                Frequency::Semiannual,
                Period::new(2, TimeUnit::Years),
                Side::Receive,
                RateType::Fixed,
                RateDefinition::default(),
                0,
                None),
        ];
        let inst = engine.run(strategies)?;
        let eval_date = Date::new(2023, 9, 2);
        let outstanding = get_outstandings_at_date(&inst, eval_date)?;
        assert_eq!(outstanding, -1800.0);

        let eval_date = Date::new(2024, 9, 2);
        let outstanding = get_outstandings_at_date(&inst, eval_date)?;
        assert_eq!(outstanding, -1800.0);
        Ok(())
    }

    #[test]
    fn test_rollover_simulation_engine_with_growth_rate() -> Result<()> {
        let market_store = create_store().unwrap();
        let horizon = Period::new(5, TimeUnit::Years);

        let base_redemptions = [
            (Date::new(2021, 9, 1), 100.0),
            (Date::new(2021, 10, 1), 100.0),
            (Date::new(2021, 11, 1), 100.0),
            (Date::new(2021, 12, 1), 100.0),
            (Date::new(2022, 1, 1), 150.0),
            (Date::new(2022, 2, 1), 150.0),
            (Date::new(2022, 3, 1), 150.0),
            (Date::new(2022, 4, 1), 150.0),
            (Date::new(2022, 5, 1), 200.0),
            (Date::new(2022, 6, 1), 200.0),
            (Date::new(2022, 7, 1), 200.0),
            (Date::new(2022, 8, 1), 200.0),
        ]
        .iter()
        .map(|&(date, value)| (date, value))
        .collect::<BTreeMap<_, _>>();

        let engine =
            RolloverSimulationEngine::new(&market_store, base_redemptions, Currency::USD, horizon)
                .with_growth_rate(0.1);
        
        let strategies = vec![
            RolloverStrategy::new(
                0.5,
                Structure::Bullet,
                Frequency::Semiannual,
                Period::new(5, TimeUnit::Years),
                Side::Receive,
                RateType::Fixed,
                RateDefinition::default(),
                0,
                None),
            RolloverStrategy::new(
                0.5,
                Structure::Bullet,
                Frequency::Semiannual,
                Period::new(10, TimeUnit::Years),
                Side::Receive,
                RateType::Fixed,
                RateDefinition::default(),
                0,
                None),
        ];
        let inst = engine.run(strategies)?;
        let eval_date = Date::new(2023, 9, 2);
        let outstanding = get_outstandings_at_date(&inst, eval_date)?;
        assert_eq!(outstanding, -1980.0);
        Ok(())
    }

    #[test]
    fn test_rollover_simulation_engine_with_anual_growth_mode() -> Result<()> {
        let market_store = create_store().unwrap();
        let horizon = Period::new(5, TimeUnit::Years);

        let base_redemptions = [
            (Date::new(2021, 9, 1), 100.0),
            (Date::new(2021, 10, 1), 100.0),
            (Date::new(2021, 11, 1), 100.0),
            (Date::new(2021, 12, 1), 100.0),
            (Date::new(2022, 1, 1), 150.0),
            (Date::new(2022, 2, 1), 150.0),
            (Date::new(2022, 3, 1), 150.0),
            (Date::new(2022, 4, 1), 150.0),
            (Date::new(2022, 5, 1), 200.0),
            (Date::new(2022, 6, 1), 200.0),
            (Date::new(2022, 7, 1), 200.0),
            (Date::new(2022, 8, 1), 200.0),
        ]
        .iter()
        .map(|&(date, value)| (date, value))
        .collect::<BTreeMap<_, _>>();

        let engine =
            RolloverSimulationEngine::new(&market_store, base_redemptions, Currency::USD, horizon)
                .with_growth_mode(GrowthMode::Annual);
        
        let strategies = vec![
            RolloverStrategy::new(
                0.5,
                Structure::Bullet,
                Frequency::Semiannual,
                Period::new(5, TimeUnit::Years),
                Side::Receive,
                RateType::Fixed,
                RateDefinition::default(),
                0,
                None),
            RolloverStrategy::new(
                0.5,
                Structure::Bullet,
                Frequency::Semiannual,
                Period::new(10, TimeUnit::Years),
                Side::Receive,
                RateType::Fixed,
                RateDefinition::default(),
                0,
                None),
        ];

        let inst = engine.run(strategies)?;
        let eval_date = Date::new(2023, 9, 2);
        let outstanding = get_outstandings_at_date(&inst, eval_date)?;
        assert_eq!(outstanding, -1800.0);

        let eval_date = Date::new(2024, 9, 2);
        let outstanding = get_outstandings_at_date(&inst, eval_date)?;
        println!("Outstanding: {}", outstanding);
        assert_eq!(outstanding, -1800.0);
        Ok(())
    }

    #[test]
    fn test_rollover_simulation_engine_with_anual_growth_mode_and_growth_rate() -> Result<()> {
        let market_store = create_store().unwrap();
        let horizon = Period::new(5, TimeUnit::Years);

        let base_redemptions = [
            (Date::new(2021, 9, 1), 100.0),
            (Date::new(2021, 10, 1), 100.0),
            (Date::new(2021, 11, 1), 100.0),
            (Date::new(2021, 12, 1), 100.0),
            (Date::new(2022, 1, 1), 150.0),
            (Date::new(2022, 2, 1), 150.0),
            (Date::new(2022, 3, 1), 150.0),
            (Date::new(2022, 4, 1), 150.0),
            (Date::new(2022, 5, 1), 200.0),
            (Date::new(2022, 6, 1), 200.0),
            (Date::new(2022, 7, 1), 200.0),
            (Date::new(2022, 8, 1), 200.0),
        ]
        .iter()
        .map(|&(date, value)| (date, value))
        .collect::<BTreeMap<_, _>>();

        let engine =
            RolloverSimulationEngine::new(&market_store, base_redemptions, Currency::USD, horizon)
                .with_growth_mode(GrowthMode::Annual)
                .with_growth_rate(0.1);
        
        let strategies = vec![
            RolloverStrategy::new(
                0.5,
                Structure::Bullet,
                Frequency::Semiannual,
                Period::new(5, TimeUnit::Years),
                Side::Receive,
                RateType::Fixed,
                RateDefinition::default(),
                0,
                None),
            RolloverStrategy::new(
                0.5,
                Structure::Bullet,
                Frequency::Semiannual,
                Period::new(10, TimeUnit::Years),
                Side::Receive,
                RateType::Fixed,
                RateDefinition::default(),
                0,
                None),
        ];

        let inst = engine.run(strategies)?;
        let eval_date = Date::new(2023, 9, 1);
        let delta_date = Actual360::year_fraction(Date::new(2021, 9, 1), eval_date);
        let outstanding = get_outstandings_at_date(&inst, eval_date)?;
        assert!((outstanding + 1800.0* (1.0 + 0.1 * delta_date)).abs() < 1e-6);

        let eval_date = Date::new(2024, 9, 1);
        let delta_date = Actual360::year_fraction(Date::new(2021, 9, 1), eval_date);
        let outstanding = get_outstandings_at_date(&inst, eval_date)?;
        assert!((outstanding + 1800.0* (1.0 + 0.1 * delta_date)).abs() < 1e-6);
        Ok(())
    }

    #[test]
    fn test_rollover_simulation_engine_with_anual_growth_mode_and_growth_rate_2() -> Result<()> {
        let market_store = create_store().unwrap();
        let horizon = Period::new(5, TimeUnit::Years);

        let base_redemptions = [
            (Date::new(2021, 9, 1), 100.0),
            (Date::new(2021, 10, 1), 100.0),
            (Date::new(2021, 11, 1), 100.0),
            (Date::new(2021, 12, 1), 100.0),
            (Date::new(2022, 1, 1), 150.0),
            (Date::new(2022, 2, 1), 150.0),
            (Date::new(2022, 3, 1), 150.0),
            (Date::new(2022, 4, 1), 150.0),
            (Date::new(2022, 5, 1), 200.0),
            (Date::new(2022, 6, 1), 200.0),
            (Date::new(2022, 7, 1), 200.0),
            (Date::new(2022, 8, 1), 200.0),
        ]
        .iter()
        .map(|&(date, value)| (date, value))
        .collect::<BTreeMap<_, _>>();

        let engine =
            RolloverSimulationEngine::new(&market_store, base_redemptions, Currency::USD, horizon)
                .with_growth_mode(GrowthMode::Annual)
                .with_growth_rate(0.1);
        
        let strategies = vec![
            RolloverStrategy::new(
                0.5,
                Structure::Bullet,
                Frequency::Semiannual,
                Period::new(1, TimeUnit::Years),
                Side::Receive,
                RateType::Fixed,
                RateDefinition::default(),
                0,
                None),
            RolloverStrategy::new(
                0.5,
                Structure::Bullet,
                Frequency::Semiannual,
                Period::new(6, TimeUnit::Months),
                Side::Receive,
                RateType::Fixed,
                RateDefinition::default(),
                0,
                None),
        ];

        let inst = engine.run(strategies)?;
        let eval_date = Date::new(2023, 9, 1);
        let delta_date = Actual360::year_fraction(Date::new(2021, 9, 1), eval_date);
        let outstanding = get_outstandings_at_date(&inst, eval_date)?;
        assert!((outstanding + 1800.0* (1.0 + 0.1 * delta_date)).abs() < 1e-6);

        let eval_date = Date::new(2024, 9, 1);
        let delta_date = Actual360::year_fraction(Date::new(2021, 9, 1), eval_date);
        let outstanding = get_outstandings_at_date(&inst, eval_date)?;
        assert!((outstanding + 1800.0* (1.0 + 0.1 * delta_date)).abs() < 1e-6);
        Ok(())
    }
}
