extern crate rustatlas;
use rayon::{
    prelude::{IntoParallelIterator, ParallelIterator},
    slice::ParallelSliceMut,
};
use rustatlas::prelude::*;
use std::sync::Arc;
mod common;
use crate::common::common::*;
use criterion::{criterion_group, criterion_main, Criterion};

fn multiple_instrument_valuations() {
    let market_store = create_store().unwrap();
    let ref_date = market_store.reference_date();

    let start_date = ref_date;
    let end_date = start_date + Period::new(10, TimeUnit::Years);
    let notional = 100_000.0;
    let rate = InterestRate::new(
        Numeric::new(0.05),
        Compounding::Simple,
        Frequency::Annual,
        DayCounter::Thirty360,
    );

    // par build
    let mut instruments: Vec<FixedRateInstrument> = (0..1000)
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
        let store = Arc::new(create_store().unwrap());
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
    // let n_threads = rayon::current_num_threads();
    // let chunk_size = instruments.len() / n_threads;
    instruments.par_rchunks_mut(1000).for_each(|chunk| {
        npv(chunk);
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("multiple_instrument_valuationss", |b| {
        b.iter(|| multiple_instrument_valuations())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
