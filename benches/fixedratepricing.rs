extern crate rustatlas;
use std::{ops::Deref, rc::Rc};

use rayon::{
    prelude::{IntoParallelRefIterator, ParallelIterator, IntoParallelIterator},
    slice::{ParallelSlice, ParallelSliceMut},
};
use rustatlas::{
    cashflows::{
        cashflow::Side,
        traits::{InterestAccrual, Payable},
    },
    core::marketstore::MarketStore,
    instruments::{fixedrateinstrument::FixedRateInstrument, makefixedrateloan::MakeFixedRateLoan},
    models::{simplemodel::SimpleModel, traits::Model},
    rates::{enums::Compounding, interestrate::InterestRate, traits::HasReferenceDate},
    time::{
        date::Date,
        daycounter::DayCounter,
        enums::{Frequency, TimeUnit},
        period::Period,
    },
    visitors::{
        indexingvisitor::IndexingVisitor,
        npvconstvisitor::NPVConstVisitor,
        parvaluevisitor::ParValueConstVisitor,
        traits::{ConstVisit, HasCashflows, Visit},
    },
};

mod common;
use crate::common::common::*;

fn starting_today_pricing() {
    let market_store = create_store();
    let ref_date = market_store.reference_date();

    let start_date = ref_date;
    let end_date = start_date + Period::new(5, TimeUnit::Years);
    let notional = 100_000.0;
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
        .with_payment_frequency(Frequency::Semiannual)
        .with_side(Side::Receive)
        .bullet()
        .with_discount_curve_id(2)
        .with_notional(notional)
        .build();

    let indexer = IndexingVisitor::new();
    indexer.visit(&mut instrument);

    let model = SimpleModel::new(market_store);

    let data = model.gen_market_data(&indexer.request());

    let ref_data = Rc::new(data);

    let npv_visitor = NPVConstVisitor::new(ref_data.clone());
    let npv = npv_visitor.visit(&instrument);
}

fn forward_starting_pricing() {
    let market_store = create_store();
    let ref_date = market_store.reference_date();

    let start_date = ref_date + Period::new(6, TimeUnit::Months);
    let end_date = start_date + Period::new(5, TimeUnit::Years);

    let notional = 100_000.0;
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
        .with_payment_frequency(Frequency::Semiannual)
        .with_side(Side::Receive)
        .bullet()
        .with_discount_curve_id(0)
        .with_notional(notional)
        .build();

    let indexer = IndexingVisitor::new();
    indexer.visit(&mut instrument);

    let model = SimpleModel::new(market_store);

    let data = model.gen_market_data(&indexer.request());
    let ref_data = Rc::new(data);

    let npv_visitor = NPVConstVisitor::new(ref_data.clone());
    let npv = npv_visitor.visit(&instrument);
}

fn already_started_pricing() {
    let market_store = create_store();
    let ref_date = market_store.reference_date();

    let start_date = ref_date - Period::new(2, TimeUnit::Months);
    let end_date = start_date + Period::new(5, TimeUnit::Years);
    let notional = 100_000.0;
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
        .with_payment_frequency(Frequency::Semiannual)
        .with_side(Side::Receive)
        .bullet()
        .with_discount_curve_id(2)
        .with_notional(notional)
        .build();

    let indexer = IndexingVisitor::new();
    indexer.visit(&mut instrument);

    let model = SimpleModel::new(market_store);

    let data = model.gen_market_data(&indexer.request());

    let ref_data = Rc::new(data);

    let npv_visitor = NPVConstVisitor::new(ref_data.clone());
    let npv = npv_visitor.visit(&instrument);
}

use criterion::{criterion_group, criterion_main, Criterion};

fn multiple() {
    let market_store = create_store();
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
    // let mut instruments = Vec::new();
    // for _ in 0..150000 {
    //     let instrument = MakeFixedRateLoan::new()
    //         .with_start_date(start_date)
    //         .with_end_date(end_date)
    //         .with_rate(rate)
    //         .with_payment_frequency(Frequency::Semiannual)
    //         .with_side(Side::Receive)
    //         .bullet()
    //         .with_discount_curve_id(2)
    //         .with_notional(notional)
    //         .build();
    //     instruments.push(instrument);
    // }

    // par build
    let mut instruments: Vec<FixedRateInstrument> = (0..150000)
        .into_par_iter() // Create a parallel iterator
        .map(|_| {
            MakeFixedRateLoan::new()
                .with_start_date(start_date.clone()) // clone data if needed
                .with_end_date(end_date.clone()) // clone data if needed
                .with_rate(rate)
                .with_payment_frequency(Frequency::Semiannual)
                .with_side(Side::Receive)
                .bullet()
                .with_discount_curve_id(2)
                .with_notional(notional)
                .build()
        })
        .collect(); // Collect the results into a Vec<_>

    fn npv(instruments: &mut [FixedRateInstrument]) -> f64 {
        let store = create_store();
        let mut npv = 0.0;
        let indexer = IndexingVisitor::new();
        instruments.iter_mut().for_each(|inst| indexer.visit(inst));

        let model = SimpleModel::new(store.clone());
        let data = model.gen_market_data(&indexer.request());

        let ref_data = Rc::new(data);
        let npv_visitor = NPVConstVisitor::new(ref_data.clone());
        instruments
            .iter()
            .for_each(|inst| npv += npv_visitor.visit(inst));
        npv
    }
    // let n_threads = rayon::current_num_threads();
    // let chunk_size = instruments.len() / n_threads;
    instruments.par_rchunks_mut(1000).for_each(|chunk| {
        npv(chunk);
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    // c.bench_function("starting_today_pricing", |b| {
    //     b.iter(|| starting_today_pricing())
    // });
    // c.bench_function("forward_starting_pricing", |b| {
    //     b.iter(|| forward_starting_pricing())
    // });
    // c.bench_function("already_started_pricing", |b| {
    //     b.iter(|| already_started_pricing())
    // });
    c.bench_function("multiple", |b| b.iter(|| multiple()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
