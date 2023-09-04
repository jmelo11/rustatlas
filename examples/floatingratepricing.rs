extern crate rustatlas;
use rustatlas::{
    core::meta::MarketData,
    models::{simplemodel::SimpleModel, traits::Model},
    rates::traits::HasReferenceDate,
    time::{
        enums::{Frequency, TimeUnit},
        period::Period,
    },
    visitors::{
        indexingvisitor::IndexingVisitor,
        npvconstvisitor::NPVConstVisitor,
        traits::{ConstVisit, Visit, HasCashflows}, fixingvisitor::FixingVisitor,
    }, instruments::makefloatingrateloan::MakeFloatingRateLoan,
};

mod common;
use crate::common::common::*;

fn starting_today_pricing() {
    print_title("Pricing of a Floating Rate Loan starting today");

    let market_store = create_store();
    let ref_date = market_store.reference_date();

    let start_date = ref_date;
    let end_date = start_date + Period::new(5, TimeUnit::Years);
    let notional = 100_000.0;

    let mut instrument = MakeFloatingRateLoan::new(start_date, end_date)
        .with_frequency(Frequency::Semiannual)
        .bullet()
        .with_notional(notional)
        .with_forecast_curve_id(1)
        .with_discount_curve_id(2)
        .build();

    let indexer = IndexingVisitor::new();
    indexer.visit(&mut instrument);

    let model = SimpleModel::new(market_store);

    let data: Vec<MarketData> = indexer
        .request()
        .iter()
        .map(|req| model.gen_node(start_date, req))
        .collect();

    let fixing_visitor = FixingVisitor::new(data.clone());
    fixing_visitor.visit(&mut instrument);

    print_table(instrument.cashflows(), &data);

    let npv_visitor = NPVConstVisitor::new(data.clone());
    let npv = npv_visitor.visit(&instrument);

    print_separator();
    println!("NPV: {}", npv);
}

fn already_started_pricing() {
    print_title("Pricing of a Floating Rate Loan already started -1Y");

    let market_store = create_store();
    let ref_date = market_store.reference_date();

    let start_date = ref_date - Period::new(3, TimeUnit::Months);
    let end_date = start_date + Period::new(5, TimeUnit::Years);
    let notional = 100_000.0;

    let mut instrument = MakeFloatingRateLoan::new(start_date, end_date)
        .with_frequency(Frequency::Semiannual)
        .bullet()
        .with_notional(notional)
        .with_forecast_curve_id(1)
        .with_discount_curve_id(2)
        .build();

    let indexer = IndexingVisitor::new();
    indexer.visit(&mut instrument);

    let model = SimpleModel::new(market_store);

    let data: Vec<MarketData> = indexer
        .request()
        .iter()
        .map(|req| model.gen_node(ref_date, req))
        .collect();

    let fixing_visitor = FixingVisitor::new(data.clone());
    fixing_visitor.visit(&mut instrument);

    print_table(instrument.cashflows(), &data);

    let npv_visitor = NPVConstVisitor::new(data.clone());
    let npv = npv_visitor.visit(&instrument);

    print_separator();
    println!("NPV: {}", npv);
}

fn main() {
    starting_today_pricing();
    already_started_pricing();
}
