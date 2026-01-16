//! Example demonstrating pricing of floating rate instruments using the rustatlas library.

extern crate rustatlas;
use rustatlas::{
    instruments::makefloatingrateinstrument::MakeFloatingRateInstrument,
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
use crate::common::common::{create_store, print_separator, print_table, print_title};

type AppResult<T> = Result<T, Box<dyn std::error::Error>>;

fn starting_today_pricing() -> AppResult<()> {
    print_title("Pricing of a Floating Rate Loan starting today");

    let market_store = create_store()?;
    let ref_date = market_store.reference_date();
    let start_date = ref_date;
    let end_date = start_date + Period::new(5, TimeUnit::Years);
    let notional = 100_000.0;

    let mut instrument = MakeFloatingRateInstrument::new()
        .with_start_date(start_date)
        .with_end_date(end_date)
        .with_payment_frequency(Frequency::Semiannual)
        .bullet()
        .with_notional(notional)
        .with_forecast_curve_id(Some(1))
        .with_discount_curve_id(Some(2))
        .build()?;

    let indexer = IndexingVisitor::new();
    indexer
        .visit(&mut instrument)
        .map_err(|e| format!("IndexingVisitor failed: {}", e))?;

    let model = SimpleModel::new(&market_store);
    let data = model.gen_market_data(&indexer.request())?;

    let fixing_visitor = FixingVisitor::new(&data);
    fixing_visitor.visit(&mut instrument)?;

    print_table(instrument.cashflows(), &data);

    let npv_visitor = NPVConstVisitor::new(&data, true);
    let npv = npv_visitor.visit(&instrument)?;

    print_separator();
    println!("NPV: {}", npv);

    let par_visitor = ParValueConstVisitor::new(&data);
    let par_value = par_visitor.visit(&instrument)?;
    println!("Par Value: {}", par_value);

    Ok(())
}

fn already_started_pricing() -> AppResult<()> {
    print_title("Pricing of a Floating Rate Loan already started -1Y");

    let market_store = create_store()?;
    let ref_date = market_store.reference_date();
    let start_date = ref_date - Period::new(3, TimeUnit::Months);
    let end_date = start_date + Period::new(5, TimeUnit::Years);
    let notional = 100_000.0;

    let mut instrument = MakeFloatingRateInstrument::new()
        .with_start_date(start_date)
        .with_end_date(end_date)
        .with_payment_frequency(Frequency::Semiannual)
        .bullet()
        .with_notional(notional)
        .with_forecast_curve_id(Some(1))
        .with_discount_curve_id(Some(2))
        .build()?;

    let indexer = IndexingVisitor::new();
    indexer
        .visit(&mut instrument)
        .map_err(|e| format!("IndexingVisitor failed: {}", e))?;

    let model = SimpleModel::new(&market_store);
    let data = model.gen_market_data(&indexer.request())?;

    let fixing_visitor = FixingVisitor::new(&data);
    fixing_visitor.visit(&mut instrument)?;

    print_table(instrument.cashflows(), &data);

    let npv_visitor = NPVConstVisitor::new(&data, true);
    let npv = npv_visitor.visit(&instrument)?;

    print_separator();
    println!("NPV: {}", npv);

    Ok(())
}

fn main() {
    if let Err(e) = starting_today_pricing() {
        eprintln!("Error in starting_today_pricing: {}", e);
    }

    if let Err(e) = already_started_pricing() {
        eprintln!("Error in already_started_pricing: {}", e);
    }
}