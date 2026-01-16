//! Benchmark for fixed rate pricing calculations.
extern crate rustatlas;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
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
use std::sync::Arc;
mod common;
use crate::common::common::create_store;
use criterion::{criterion_group, criterion_main, Criterion};

fn npv(instruments: &mut [FixedRateInstrument]) -> Result<f64, Box<dyn std::error::Error>> {
    let store = Arc::new(create_store()?);
    let mut total_npv = 0.0;
    
    let indexer = IndexingVisitor::new();
    
    // Should index all instruments
    for inst in instruments.iter_mut() {
        indexer
            .visit(inst)
            .map_err(|e| format!("IndexingVisitor failed: {}", e))?;
    }

    let model = SimpleModel::new(&store);
    let data = model.gen_market_data(&indexer.request())?;

    let npv_visitor = NPVConstVisitor::new(&data, true);
    
    // Calculate NPV for all instruments
    for inst in instruments.iter() {
        let inst_npv = npv_visitor
            .visit(inst)
            .map_err(|e| format!("NPVConstVisitor failed: {}", e))?;
        total_npv += inst_npv;
    }

    Ok(total_npv)
}

/// Benchmark function that creates and processes 150,000 fixed rate instruments in parallel.
fn multiple() -> Result<(), Box<dyn std::error::Error>> {
    let market_store = create_store()?;
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

    // Build instruments in parallel
    let mut instruments: Vec<FixedRateInstrument> = (0..150000)
        .into_par_iter()
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
        })
        .collect::<Result<Vec<_>, _>>()?;

    // Process instruments in parallel chunks
    instruments.par_rchunks_mut(1000).for_each(|chunk| {
        if let Err(e) = npv(chunk) {
            eprintln!("Error processing chunk: {}", e);
        }
    });

    Ok(())
}

/// Benchmark criterion for fixed rate pricing calculations.
fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("multiple", |b| {
        b.iter(|| {
            multiple().expect("benchmark failed")
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);