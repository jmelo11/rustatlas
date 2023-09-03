extern crate rustatlas;
use rustatlas::prelude::*;

mod common;
use crate::common::common::*;

fn starting_today_pricing() {
    print_title("Pricing of a Fixed Rate Loan starting today");

    let start_date = Date::new(2021, 9, 1);
    let end_date = Date::new(2026, 9, 1);
    let notional = 100_000.0;
    let rate = InterestRate::new(
        0.05,
        Compounding::Simple,
        Frequency::Annual,
        DayCounter::Actual360,
    );

    let mut instrument = MakeFixedRateLoan::new(start_date, end_date, rate)
        .with_frequency(Frequency::Semiannual)
        .with_side(Side::Receive)
        .bullet()
        .with_discount_curve_id(0)
        .with_notional(notional)
        .build();

    let indexer = IndexingVisitor::new();
    indexer.visit(&mut instrument);

    let market_store = create_store();
    let ref_date = market_store.reference_date();

    let model = SimpleModel::new(market_store);

    let data: Vec<MarketData> = indexer
        .request()
        .iter()
        .map(|req| model.gen_node(ref_date, req))
        .collect();

    print_table(instrument.cashflows(), &data);

    let npv_visitor = NPVConstVisitor::new(data);
    let npv = npv_visitor.visit(&instrument);

    print_separator();
    println!("NPV: {}", npv);

    let start_accrual = Date::new(2024, 9, 1);
    let end_accrual = Date::new(2024, 10, 1);
    let accrued_amount = instrument.cashflows().iter().fold(0.0, |acc, cf| {
        acc + cf.accrued_amount(start_accrual, end_accrual)
    });
    println!(
        "Accrued Amount between {} and {}: {}",
        start_accrual, end_accrual, accrued_amount
    );
}

fn forward_starting_pricing() {
    print_title("Pricing of a Fixed Rate Loan starting +2Y");

    let start_date = Date::new(2023, 9, 1);
    let end_date = Date::new(2026, 9, 1);
    let notional = 100_000.0;
    let rate = InterestRate::new(
        0.05,
        Compounding::Simple,
        Frequency::Annual,
        DayCounter::Actual360,
    );

    let mut instrument = MakeFixedRateLoan::new(start_date, end_date, rate)
        .with_frequency(Frequency::Semiannual)
        .with_side(Side::Receive)
        .bullet()
        .with_discount_curve_id(0)
        .with_notional(notional)
        .build();

    let indexer = IndexingVisitor::new();
    indexer.visit(&mut instrument);

    let market_store = create_store();
    let ref_date = market_store.reference_date();

    let model = SimpleModel::new(market_store);

    let data: Vec<MarketData> = indexer
        .request()
        .iter()
        .map(|req| model.gen_node(ref_date, req))
        .collect();

    print_table(instrument.cashflows(), &data);

    let npv_visitor = NPVConstVisitor::new(data);
    let npv = npv_visitor.visit(&instrument);

    print_separator();
    println!("NPV: {}", npv);

    let start_accrual = Date::new(2024, 9, 1);
    let end_accrual = Date::new(2024, 10, 1);
    let accrued_amount = instrument.cashflows().iter().fold(0.0, |acc, cf| {
        acc + cf.accrued_amount(start_accrual, end_accrual)
    });
    println!(
        "Accrued Amount between {} and {}: {}",
        start_accrual, end_accrual, accrued_amount
    );
}

fn already_started_pricing() {
    print_title("Pricing of a Fixed Rate Loan starting +2Y");

    let start_date = Date::new(2020, 9, 1);
    let end_date = Date::new(2026, 9, 1);
    let notional = 100_000.0;
    let rate = InterestRate::new(
        0.05,
        Compounding::Simple,
        Frequency::Annual,
        DayCounter::Actual360,
    );

    let mut instrument = MakeFixedRateLoan::new(start_date, end_date, rate)
        .with_frequency(Frequency::Semiannual)
        .with_side(Side::Receive)
        .bullet()
        .with_discount_curve_id(0)
        .with_notional(notional)
        .build();

    let indexer = IndexingVisitor::new();
    indexer.visit(&mut instrument);

    let market_store = create_store();
    let ref_date = market_store.reference_date();

    let model = SimpleModel::new(market_store);

    let data: Vec<MarketData> = indexer
        .request()
        .iter()
        .map(|req| model.gen_node(ref_date, req))
        .collect();

    print_table(instrument.cashflows(), &data);

    let npv_visitor = NPVConstVisitor::new(data);
    let npv = npv_visitor.visit(&instrument);

    print_separator();
    println!("NPV: {}", npv);

    let start_accrual = Date::new(2024, 9, 1);
    let end_accrual = Date::new(2024, 10, 1);
    let accrued_amount = instrument.cashflows().iter().fold(0.0, |acc, cf| {
        acc + cf.accrued_amount(start_accrual, end_accrual)
    });
    println!(
        "Accrued Amount between {} and {}: {}",
        start_accrual, end_accrual, accrued_amount
    );
}

fn main() {
    starting_today_pricing();
    println!("\n");
    forward_starting_pricing();
    println!("\n");
    already_started_pricing();
}
