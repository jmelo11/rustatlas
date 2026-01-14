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
use crate::common::common::*;
use criterion::{criterion_group, criterion_main, Criterion};

type AppResult<T> = Result<T, Box<dyn std::error::Error>>;

fn multiple() -> AppResult<()> {
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

    // par build - collect Results separately, then unwrap
    let instruments: Vec<FixedRateInstrument> = (0..150000)
        .into_par_iter()
        .map(|_| {
            MakeFixedRateInstrument::new()
                .with_start_date(start_date.clone())
                .with_end_date(end_date.clone())
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

    let mut instruments = instruments;
    npv(&mut instruments)?;

    Ok(())
}

fn npv(instruments: &mut [FixedRateInstrument]) -> AppResult<f64> {
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

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("multiple", |b| {
        b.iter(|| multiple().expect("benchmark failed"))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
