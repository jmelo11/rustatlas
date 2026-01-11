//! Example demonstrating fixed rate instrument pricing with various scenarios.

extern crate rustatlas;

use rustatlas::{
    cashflows::{
        cashflow::Side,
        traits::{InterestAccrual, Payable},
    },
    instruments::makefixedrateinstrument::MakeFixedRateInstrument,
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
use crate::common::common::{create_store, print_separator, print_table, print_title};

fn starting_today_pricing() {
    print_title("Pricing of a Fixed Rate Loan starting today");
    let market_store =
        create_store().unwrap_or_else(|err| panic!("Failed to create store: {err}"));
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

    let mut instrument = MakeFixedRateInstrument::new()
        .with_start_date(start_date)
        .with_end_date(end_date)
        .with_rate(rate)
        .with_payment_frequency(Frequency::Semiannual)
        .with_side(Side::Receive)
        .bullet()
        .with_discount_curve_id(Some(2))
        .with_notional(notional)
        .build()
        .unwrap_or_else(|err| panic!("Failed to build instrument: {err}"));

    let indexer = IndexingVisitor::new();
    indexer
        .visit(&mut instrument)
        .unwrap_or_else(|err| panic!("IndexingVisitor failed with error: {err}"));

    let ref_date = market_store.reference_date();
    let model = SimpleModel::new(&market_store);

    let data = model
        .gen_market_data(&indexer.request())
        .unwrap_or_else(|err| panic!("Failed to generate market data: {err}"));
    print_table(instrument.cashflows(), &data);

    let npv_visitor = NPVConstVisitor::new(&data, true);
    let npv = npv_visitor.visit(&instrument);

    print_separator();
    println!(
        "NPV: {}",
        npv.unwrap_or_else(|err| panic!("Failed to compute NPV: {err}"))
    );

    let start_accrual = Date::new(2024, 9, 1);
    let end_accrual = Date::new(2024, 10, 1);
    let accrued_amount = instrument.cashflows().iter().fold(0.0, |acc, cf| {
        acc + cf
            .accrued_amount(start_accrual, end_accrual)
            .unwrap_or_else(|err| panic!("Failed to compute accrued amount: {err}"))
    });

    println!("Accrued Amount between {start_accrual} and {end_accrual}: {accrued_amount}");

    let maturing_amount = instrument.cashflows().iter().fold(0.0, |acc, cf| {
        if cf.payment_date() == ref_date {
            acc + cf
                .amount()
                .unwrap_or_else(|err| panic!("Failed to load cashflow amount: {err}"))
        } else {
            acc
        }
    });

    println!("Maturing Amount between {start_accrual} and {end_accrual}: {maturing_amount}");

    let par_visitor = ParValueConstVisitor::new(&data);
    let par_value = par_visitor
        .visit(&instrument)
        .unwrap_or_else(|err| panic!("Failed to compute par value: {err}"));
    println!("Par Value: {par_value}");
}

fn forward_starting_pricing() {
    print_title("Pricing of a Fixed Rate Loan starting +2Y");

    let market_store =
        create_store().unwrap_or_else(|err| panic!("Failed to create store: {err}"));
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

    let mut instrument = MakeFixedRateInstrument::new()
        .with_start_date(start_date)
        .with_end_date(end_date)
        .with_rate(rate)
        .with_payment_frequency(Frequency::Semiannual)
        .with_side(Side::Receive)
        .bullet()
        .with_discount_curve_id(Some(0))
        .with_notional(notional)
        .build()
        .unwrap_or_else(|err| panic!("Failed to build instrument: {err}"));

    let indexer = IndexingVisitor::new();
    indexer
        .visit(&mut instrument)
        .unwrap_or_else(|err| panic!("IndexingVisitor failed with error: {err}"));

    let model = SimpleModel::new(&market_store);

    let data = model
        .gen_market_data(&indexer.request())
        .unwrap_or_else(|err| panic!("Failed to generate market data: {err}"));
    print_table(instrument.cashflows(), &data);

    let npv_visitor = NPVConstVisitor::new(&data, true);
    let npv = npv_visitor.visit(&instrument);

    print_separator();
    println!(
        "NPV: {}",
        npv.unwrap_or_else(|err| panic!("Failed to compute NPV: {err}"))
    );

    let start_accrual = Date::new(2024, 9, 1);
    let end_accrual = Date::new(2024, 10, 1);
    let accrued_amount = instrument.cashflows().iter().fold(0.0, |acc, cf| {
        acc + cf
            .accrued_amount(start_accrual, end_accrual)
            .unwrap_or_else(|err| panic!("Failed to compute accrued amount: {err}"))
    });
    println!("Accrued Amount between {start_accrual} and {end_accrual}: {accrued_amount}");
}

fn already_started_pricing() {
    print_title("Pricing of a Fixed Rate Loan starting +2Y");

    let market_store =
        create_store().unwrap_or_else(|err| panic!("Failed to create store: {err}"));
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

    let mut instrument = MakeFixedRateInstrument::new()
        .with_start_date(start_date)
        .with_end_date(end_date)
        .with_rate(rate)
        .with_payment_frequency(Frequency::Semiannual)
        .with_side(Side::Receive)
        .bullet()
        .with_discount_curve_id(Some(2))
        .with_notional(notional)
        .build()
        .unwrap_or_else(|err| panic!("Failed to build instrument: {err}"));

    let indexer = IndexingVisitor::new();
    indexer
        .visit(&mut instrument)
        .unwrap_or_else(|err| panic!("IndexingVisitor failed with error: {err}"));

    let model = SimpleModel::new(&market_store);

    let data = model
        .gen_market_data(&indexer.request())
        .unwrap_or_else(|err| panic!("Failed to generate market data: {err}"));
    print_table(instrument.cashflows(), &data);

    let npv_visitor = NPVConstVisitor::new(&data, true);
    let npv = npv_visitor.visit(&instrument);

    print_separator();
    println!(
        "NPV: {}",
        npv.unwrap_or_else(|err| panic!("Failed to compute NPV: {err}"))
    );

    let start_accrual = Date::new(2024, 9, 1);
    let end_accrual = Date::new(2024, 10, 1);
    let accrued_amount = instrument.cashflows().iter().fold(0.0, |acc, cf| {
        acc + cf
            .accrued_amount(start_accrual, end_accrual)
            .unwrap_or_else(|err| panic!("Failed to compute accrued amount: {err}"))
    });
    println!("Accrued Amount between {start_accrual} and {end_accrual}: {accrued_amount}");

    let par_visitor = ParValueConstVisitor::new(&data);
    let par_value = par_visitor
        .visit(&instrument)
        .unwrap_or_else(|err| panic!("Failed to compute par value: {err}"));
    println!("Par Value: {par_value}");
}

fn main() {
    starting_today_pricing();
    println!("\n");
    forward_starting_pricing();
    println!("\n");
    already_started_pricing();
}
