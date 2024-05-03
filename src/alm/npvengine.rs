use std::collections::BTreeMap;

use rayon::{
    iter::ParallelIterator,
    slice::{ParallelSlice, ParallelSliceMut},
};

use crate::{
    core::marketstore::MarketStore,
    instruments::instrument::Instrument,
    models::{simplemodel::SimpleModel, traits::Model},
    rates::traits::HasReferenceDate,
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
        let npv_by_date_visitor =
            NPVByDateConstVisitor::new(self.market_store.reference_date(), &data, false);
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
                    .fold(BTreeMap::new(), |mut acc, npv_map| {
                        npv_map.iter().for_each(|(date, npv)| {
                            let acc_npv = acc.entry(*date).or_insert(0.0);
                            *acc_npv += npv;
                        });
                        acc
                    });
                chunk_npv
            })
            .reduce(BTreeMap::new, |mut acc, chunk_npv| {
                chunk_npv.iter().for_each(|(date, npv)| {
                    let acc_npv = acc.entry(*date).or_insert(0.0);
                    *acc_npv += npv;
                });
                acc
            });

        Ok(npv_date_map)
    }
}
