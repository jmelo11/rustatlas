use std::collections::BTreeMap;

use super::positiongenerator::{PositionGenerator, RolloverStrategy};
use crate::{
    core::marketstore::MarketStore,
    currencies::enums::Currency,
    instruments::instrument::Instrument,
    models::{simplemodel::SimpleModel, traits::Model},
    rates::traits::HasReferenceDate,
    time::{
        calendar::Calendar,
        calendars::nullcalendar::NullCalendar,
        date::Date,
        enums::{BusinessDayConvention, TimeUnit},
        period::Period,
        schedule::MakeSchedule,
    },
    utils::errors::Result,
    visitors::{
        cashflowaggregationvisitor::CashflowsAggregatorConstVisitor,
        fixingvisitor::FixingVisitor,
        indexingvisitor::IndexingVisitor,
        traits::{ConstVisit, Visit},
    },
};

/// # RolloverSimulationEngine
/// Engine is the main component of the simulation. It is responsible for:
/// - generating the new positions
/// - indexing the new positions and getting the relevant market data
/// - aggregating the new redemptions to the portfolio redemptions
///
/// ## Details
/// - Requires redemptions and the currency of the redemptions
pub struct RolloverSimulationEngine<'a> {
    market_store: &'a MarketStore,
    base_redemptions: BTreeMap<Date, f64>,
    redemptions_currency: Currency,
    eval_dates: Vec<Date>,
    chunk_size: usize,
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
            chunk_size: 1000,
        }
    }

    pub fn with_chunk_size(mut self, chunk_size: usize) -> Self {
        self.chunk_size = chunk_size;
        self
    }

    pub fn run(&self, strategies: Vec<RolloverStrategy>) -> Result<Vec<Instrument>> {
        let mut redemptions = self.base_redemptions.clone();
        // vector of new positions
        let mut simulated_instruments = Vec::new();

        // redemptions for target portfolio
        let generator = PositionGenerator::new(self.redemptions_currency, strategies.clone());

        self.eval_dates.iter().try_for_each(|date| -> Result<()> {
            let maturing_amount = redemptions.get(date);
            match maturing_amount {
                Some(amount) => {
                    // relevant data for new positions
                    let tmp_store = self.market_store.advance_to_date(*date)?;
                    let new_generator = generator
                        .clone()
                        .with_market_store(&tmp_store)
                        .with_amount(*amount);

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
                    simulated_instruments.append(&mut positions);

                    // add new redemptions to the vector
                    let aggregator = CashflowsAggregatorConstVisitor::new()
                        .with_validate_currency(self.redemptions_currency);
                    positions.iter().try_for_each(|inst| -> Result<()> {
                        aggregator.visit(inst)?;
                        Ok(())
                    })?;

                    let new_redemptions = aggregator.redemptions();

                    redemptions
                        .iter_mut()
                        .try_for_each(|(date, amount)| -> Result<()> {
                            let new_amount = new_redemptions.get(date);
                            match new_amount {
                                Some(new_amount) => *amount += new_amount,
                                None => {}
                            }
                            Ok(())
                        })?;
                }
                None => {}
            }
            Ok(())
        })?;

        Ok(simulated_instruments)
    }
}

// #[cfg(test)]
// mod tests {

//     use crate::{
//         currencies::enums::Currency,
//         math::interpolation::enums::Interpolator,
//         rates::{
//             interestrate::RateDefinition,
//             interestrateindex::iborindex::IborIndex,
//             yieldtermstructure::{
//                 compositetermstructure::CompositeTermStructure,
//                 discounttermstructure::DiscountTermStructure,
//                 flatforwardtermstructure::FlatForwardTermStructure,
//                 tenorbasedzeroratetermstructure::TenorBasedZeroRateTermStructure,
//             },
//         },
//         time::daycounter::DayCounter,
//     };

//     use super::*;

//     fn create_store() -> Result<MarketStore> {
//         let ref_date = Date::new(2021, 9, 1);
//         let local_currency = Currency::USD;
//         let mut market_store = MarketStore::new(ref_date, local_currency);

//         let base_curve = Box::new(FlatForwardTermStructure::new(
//             ref_date,
//             0.02,
//             RateDefinition::default(),
//         ));

//         let spread_curve = Box::new(TenorBasedZeroRateTermStructure::new(
//             ref_date,
//             vec![
//                 Period::new(1, TimeUnit::Days),
//                 Period::new(3, TimeUnit::Months),
//                 Period::new(6, TimeUnit::Months),
//                 Period::new(1, TimeUnit::Years),
//             ],
//             vec![0.01, 0.03, 0.04, 0.05],
//             RateDefinition::default(),
//             Interpolator::Linear,
//             true,
//         )?);

//         // with this curve, we shouldn't see rate changes since the spread endsup being constant and the base is flatforward
//         let composite_curve = Box::new(CompositeTermStructure::new(spread_curve, base_curve));
//         let composite_index =
//             IborIndex::new(composite_curve.reference_date()).with_term_structure(composite_curve);

//         market_store
//             .mut_index_store()
//             .add_index(0, Box::new(composite_index))?;

//         let discount_factors = vec![1.0, 0.99, 0.978, 0.956, 0.934];
//         let dates = vec![
//             ref_date,
//             ref_date + Period::new(1, TimeUnit::Months),
//             ref_date + Period::new(3, TimeUnit::Months),
//             ref_date + Period::new(6, TimeUnit::Months),
//             ref_date + Period::new(1, TimeUnit::Years),
//         ];

//         let spread_curve_2 = Box::new(TenorBasedZeroRateTermStructure::new(
//             ref_date,
//             vec![
//                 Period::new(1, TimeUnit::Days),
//                 Period::new(3, TimeUnit::Months),
//                 Period::new(6, TimeUnit::Months),
//                 Period::new(1, TimeUnit::Years),
//             ],
//             vec![0.01, 0.03, 0.04, 0.05],
//             RateDefinition::default(),
//             Interpolator::Linear,
//             true,
//         )?);

//         let discount_curve = Box::new(DiscountTermStructure::new(
//             dates,
//             discount_factors,
//             DayCounter::Actual360,
//             Interpolator::Linear,
//             true,
//         )?);

//         let composite_curve_2 =
//             Box::new(CompositeTermStructure::new(spread_curve_2, discount_curve));

//         let composite_index_2 = IborIndex::new(composite_curve_2.reference_date())
//             .with_term_structure(composite_curve_2);

//         market_store
//             .mut_index_store()
//             .add_index(1, Box::new(composite_index_2))?;

//         return Ok(market_store);
//     }
// }
