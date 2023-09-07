extern crate rustatlas;
use std::rc::Rc;

use rustatlas::{
    core::meta::MarketData,
    instruments::makefloatingrateloan::MakeFloatingRateLoan,
    models::{simplemodel::SimpleModel, traits::Model},
    rates::traits::HasReferenceDate,
    time::{
        enums::{Frequency, TimeUnit},
        period::Period,
    },
    visitors::{
        fixingvisitor::FixingVisitor,
        indexingvisitor::IndexingVisitor,
        npvconstvisitor::NPVConstVisitor,
        parvaluevisitor::ParValueConstVisitor,
        traits::{ConstVisit, HasCashflows, Visit},
    },
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

    let mut instrument = MakeFloatingRateLoan::new()
        .with_start_date(start_date)
        .with_end_date(end_date)
        .with_payment_frequency(Frequency::Semiannual)
        .bullet()
        .with_notional(notional)
        .with_forecast_curve_id(Some(1))
        .with_discount_curve_id(Some(2))
        .build();

    let indexer = IndexingVisitor::new();
    indexer.visit(&mut instrument);

    let model = SimpleModel::new(market_store);

    let data = model.gen_market_data(&indexer.request());

    let ref_data: Rc<Vec<MarketData>> = Rc::new(data);

    let fixing_visitor = FixingVisitor::new(ref_data.clone());
    fixing_visitor.visit(&mut instrument);

    print_table(instrument.cashflows(), ref_data.clone());

    let npv_visitor = NPVConstVisitor::new(ref_data.clone());
    let npv = npv_visitor.visit(&instrument);

    print_separator();
    println!("NPV: {}", npv);

    let par_visitor = ParValueConstVisitor::new(ref_data.clone());
    let par_value = par_visitor.visit(&instrument);
    println!("Par Value: {}", par_value);
}

fn already_started_pricing() {
    print_title("Pricing of a Floating Rate Loan already started -1Y");

    let market_store = create_store();
    let ref_date = market_store.reference_date();

    let start_date = ref_date - Period::new(3, TimeUnit::Months);
    let end_date = start_date + Period::new(5, TimeUnit::Years);
    let notional = 100_000.0;

    let mut instrument = MakeFloatingRateLoan::new()
        .with_start_date(start_date)
        .with_end_date(end_date)
        .with_payment_frequency(Frequency::Semiannual)
        .bullet()
        .with_notional(notional)
        .with_forecast_curve_id(Some(1))
        .with_discount_curve_id(Some(2))
        .build();

    let indexer = IndexingVisitor::new();
    indexer.visit(&mut instrument);

    let model = SimpleModel::new(market_store);

    let data = model.gen_market_data(&indexer.request());

    let ref_data: Rc<Vec<MarketData>> = Rc::new(data);
    let fixing_visitor = FixingVisitor::new(ref_data.clone());
    fixing_visitor.visit(&mut instrument);

    print_table(instrument.cashflows(), ref_data.clone());

    let npv_visitor = NPVConstVisitor::new(ref_data.clone());
    let npv = npv_visitor.visit(&instrument);

    print_separator();
    println!("NPV: {}", npv);
}

fn main() {
    starting_today_pricing();
    already_started_pricing();
}
