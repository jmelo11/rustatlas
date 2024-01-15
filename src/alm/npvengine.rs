use std::collections::BTreeMap;

use rayon::{
    iter::ParallelIterator,
    slice::{ParallelSlice, ParallelSliceMut},
};

use crate::{
    core::marketstore::MarketStore,
    instruments::instrument::Instrument,
    models::{simplemodel::SimpleModel, traits::Model},
    time::date::Date,
    utils::errors::{AtlasError, Result},
    visitors::{
        fixingvisitor::FixingVisitor,
        indexingvisitor::IndexingVisitor,
        npvbydateconstvisitor::NPVByDateConstVisitor,
        traits::{ConstVisit, Visit},
    },
};

/// # NPVEngine
/// The NPVEngine is responsible for calculating the NPV of a portfolio.
/// It is a parallelized engine that uses rayon to parallelize the calculation.
///
/// ## Parameters
/// * `instruments` - A mutable slice of instruments
/// * `market_store` - A reference to a market store
/// * `chunk_size` - The chunk size to use for parallelization
pub struct NPVEngine<'a> {
    instruments: &'a mut [Instrument],
    market_store: &'a MarketStore,
    chunk_size: usize,
}

impl<'a> NPVEngine<'a> {
    pub fn new(instruments: &'a mut [Instrument], market_store: &'a MarketStore) -> Self {
        NPVEngine {
            instruments,
            market_store,
            chunk_size: 1000,
        }
    }

    pub fn with_chunk_size(mut self, chunk_size: usize) -> Self {
        self.chunk_size = chunk_size;
        self
    }

    pub fn run(&mut self) -> Result<BTreeMap<Date, f64>> {
        // indexing
        let indexing_visitor = IndexingVisitor::new();
        self.instruments
            .iter_mut()
            .try_for_each(|inst| -> Result<()> {
                indexing_visitor.visit(inst)?;
                Ok(())
            })?;

        // market data for base positions
        let model = SimpleModel::new(self.market_store);
        let data = model.gen_market_data(&indexing_visitor.request())?;

        // fixing for base positions
        let fixing_visitor = FixingVisitor::new(&data);

        self.instruments
            .par_rchunks_mut(self.chunk_size)
            .try_for_each(|chunk| {
                chunk.iter_mut().try_for_each(|inst| {
                    fixing_visitor.visit(inst).map_err(|e| {
                        AtlasError::EvaluationErr(format!(
                            "An error was found while processing instrument with id {:?}: {}",
                            inst.id(),
                            e
                        ))
                    })
                })
            })?;

        // npv
        let npv_by_date_visitor = NPVByDateConstVisitor::new(&data, false);
        let npv_date_map = self
            .instruments
            .par_rchunks(self.chunk_size)
            .map(|chunk| {
                let chunk_npv = chunk
                    .iter()
                    .map(|inst| {
                        let npv_map = npv_by_date_visitor
                            .visit(inst)
                            .map_err(|e| {
                                AtlasError::EvaluationErr(format!(
                                "An error was found while processing instrument with id {:?}: {}",
                                inst.id(),
                                e
                            ))
                            })
                            .unwrap();
                        npv_map
                    })
                    .flatten()
                    .collect::<BTreeMap<Date, f64>>();
                chunk_npv
            })
            .flatten()
            .collect::<BTreeMap<Date, f64>>();

        Ok(npv_date_map)
    }
}

// #[cfg(test)]
// mod tests {
//     use std::collections::HashMap;

//     use crate::{
//         cashflows::cashflow::Side,
//         currencies::enums::Currency,
//         instruments::{
//             instrument::{Instrument, RateType},
//             makefixedrateinstrument::MakeFixedRateInstrument,
//         },
//         rates::{
//             enums::Compounding,
//             interestrate::{InterestRate, RateDefinition},
//             interestrateindex::{iborindex::IborIndex, overnightindex::OvernightIndex},
//             traits::HasReferenceDate,
//             yieldtermstructure::flatforwardtermstructure::FlatForwardTermStructure,
//         },
//         time::{
//             daycounter::DayCounter,
//             enums::{Frequency, TimeUnit},
//             period::Period,
//         },
//     };

//     use super::*;

//     fn make_fixings(start: Date, end: Date, rate: f64) -> HashMap<Date, f64> {
//         let mut fixings = HashMap::new();
//         let mut seed = start;
//         let mut init = 100.0;
//         while seed <= end {
//             fixings.insert(seed, init);
//             seed = seed + Period::new(1, TimeUnit::Days);
//             init = init * (1.0 + rate * 1.0 / 360.0);
//         }
//         return fixings;
//     }

//     fn create_store() -> Result<MarketStore> {
//         let ref_date = Date::new(2021, 9, 1);
//         let local_currency = Currency::USD;
//         let mut market_store = MarketStore::new(ref_date, local_currency);

//         let forecast_curve_1 = Box::new(FlatForwardTermStructure::new(
//             ref_date,
//             0.02,
//             RateDefinition::default(),
//         ));

//         let forecast_curve_2 = Box::new(FlatForwardTermStructure::new(
//             ref_date,
//             0.03,
//             RateDefinition::default(),
//         ));

//         let discount_curve = Box::new(FlatForwardTermStructure::new(
//             ref_date,
//             0.05,
//             RateDefinition::default(),
//         ));

//         let mut ibor_fixings = HashMap::new();
//         ibor_fixings.insert(Date::new(2021, 9, 1), 0.02); // today
//         ibor_fixings.insert(Date::new(2021, 8, 31), 0.02); // yesterday

//         let ibor_index = IborIndex::new(forecast_curve_1.reference_date())
//             .with_fixings(ibor_fixings)
//             .with_term_structure(forecast_curve_1)
//             .with_frequency(Frequency::Annual);

//         let overnight_fixings =
//             make_fixings(ref_date - Period::new(1, TimeUnit::Years), ref_date, 0.06);

//         let overnigth_index = OvernightIndex::new(forecast_curve_2.reference_date())
//             .with_term_structure(forecast_curve_2)
//             .with_fixings(overnight_fixings);

//         market_store
//             .mut_index_store()
//             .add_index(0, Box::new(ibor_index))?;

//         market_store
//             .mut_index_store()
//             .add_index(1, Box::new(overnigth_index))?;

//         let discount_index =
//             IborIndex::new(discount_curve.reference_date()).with_term_structure(discount_curve);

//         market_store
//             .mut_index_store()
//             .add_index(2, Box::new(discount_index))?;
//         return Ok(market_store);
//     }
// }
