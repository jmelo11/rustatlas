//! Benchmark for fixed rate pricing calculations.
extern crate rustatlas;

use std::sync::Arc;

use rayon::{
    prelude::{IntoParallelIterator, ParallelIterator},
    slice::ParallelSliceMut,
};
use rustatlas::{
    cashflows::cashflow::Side,
    currencies::enums::Currency,
    instruments::{
        fixedrateinstrument::FixedRateInstrument, makefixedrateinstrument::MakeFixedRateInstrument,
    },
    models::{simplemodel::SimpleModel, traits::Model},
    rates::{enums::Compounding, interestrate::InterestRate, traits::HasReferenceDate},
    time::{
        daycounter::DayCounter,
        enums::{Frequency, TimeUnit},
        period::Period,
    },
    visitors::{
        indexingvisitor::IndexingVisitor,
        npvconstvisitor::NPVConstVisitor,
        traits::{ConstVisit, Visit},
    },
};

mod common;
use crate::common::common::create_store;

use criterion::Criterion;

fn npv(instruments: &mut [FixedRateInstrument]) -> f64 {
    let store = Arc::new(
        create_store().unwrap_or_else(|err| panic!("Failed to create store: {err}")),
    );
    let mut npv = 0.0;
    let indexer = IndexingVisitor::new();
    for inst in instruments.iter_mut() {
        indexer
            .visit(inst)
            .unwrap_or_else(|err| panic!("Failed to index instrument: {err}"));
    }

    let model = SimpleModel::new(&store);
    let data = model
        .gen_market_data(&indexer.request())
        .unwrap_or_else(|err| panic!("Failed to generate market data: {err}"));

    let npv_visitor = NPVConstVisitor::new(&data, true);
    for inst in instruments.iter() {
        npv += npv_visitor
            .visit(inst)
            .unwrap_or_else(|err| panic!("Failed to compute NPV: {err}"));
    }
    npv
}

/// Benchmark function that creates and processes 150,000 fixed rate instruments in parallel.
fn multiple() {
    let market_store =
        create_store().unwrap_or_else(|err| panic!("Failed to create store: {err}"));
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
                .with_start_date(start_date)
                .with_end_date(end_date)
                .with_rate(rate)
                .with_payment_frequency(Frequency::Semiannual)
                .with_side(Side::Receive)
                .with_currency(Currency::USD)
                .bullet()
                .with_discount_curve_id(Some(2))
                .with_notional(notional)
                .build()
                .unwrap_or_else(|err| panic!("Failed to build instrument: {err}"))
        })
        .collect(); // Collect the results into a Vec<_>

    // let n_threads = rayon::current_num_threads();
    // let chunk_size = instruments.len() / n_threads;
    instruments.par_rchunks_mut(1000).for_each(|chunk| {
        npv(chunk);
    });
}

/// Benchmark criterion for fixed rate pricing calculations.
fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("multiple", |b| b.iter(multiple));
}

fn main() {
    let mut c = Criterion::default().configure_from_args();
    criterion_benchmark(&mut c);
}
