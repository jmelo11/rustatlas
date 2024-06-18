extern crate rustatlas;
mod common;
use crate::common::common::*;
#[cfg(feature = "aad")]
use rustatlas::math::aad::tape::TAPE;
use rustatlas::prelude::*;
/// Pricing of a Fixed Rate Loan starting today
/// The loan starts today and ends in 5 years. The notional is 100,000. The rate is 5% compounded annually.
fn starting_today_pricing() {
    print_title("Pricing of a Fixed Rate Loan starting today");
    let market_store = create_store().unwrap();
    let ref_date = market_store.reference_date();

    let start_date = ref_date;
    let end_date = start_date + Period::new(5, TimeUnit::Years);
    let notional = 100_000.0;
    let rate = InterestRate::new(
        Number::new(0.05),
        Compounding::Simple,
        Frequency::Annual,
        DayCounter::Actual360,
    );

    let mut instrument = MakeFixedRateInstrument::new()
        .with_start_date(start_date)
        .with_end_date(end_date)
        .with_rate(rate)
        .with_payment_frequency(Frequency::Semiannual)
        .with_currency(Currency::USD)
        .with_side(Side::Receive)
        .bullet()
        .with_discount_curve_id(Some(2))
        .with_notional(notional)
        .build()
        .unwrap();

    let indexer = IndexingVisitor::new();
    let result = indexer.visit(&mut instrument);
    match result {
        Ok(_) => (),
        Err(e) => panic!("IndexingVisitor failed with error: {}", e),
    }

    let ref_date = market_store.reference_date();
    let model = SimpleModel::new(&market_store);
    let data = model.gen_market_data(&indexer.request()).unwrap();

    print_table(instrument.cashflows(), &data);

    let npv_visitor = NPVConstVisitor::new(&data, true);
    let npv = npv_visitor.visit(&instrument);

    print_separator();
    println!("NPV: {}", npv.unwrap());

    let start_accrual = Date::new(2024, 9, 1);
    let end_accrual = Date::new(2024, 10, 1);
    let accrued_amount = instrument
        .cashflows()
        .iter()
        .fold(Number::new(0.0), |acc, cf| {
            acc + cf.accrued_amount(start_accrual, end_accrual).unwrap()
        });

    println!(
        "Accrued Amount between {} and {}: {}",
        start_accrual, end_accrual, accrued_amount
    );

    let maturing_amount = instrument
        .cashflows()
        .iter()
        .fold(Number::new(0.0), |acc, cf| {
            if cf.payment_date() == ref_date {
                acc + cf.amount().unwrap()
            } else {
                acc
            }
        });

    println!(
        "Maturing Amount between {} and {}: {}",
        start_accrual, end_accrual, maturing_amount
    );

    let par_visitor = ParValueConstVisitor::new(&data);
    let par_value = par_visitor.visit(&instrument).unwrap();
    println!("Par Value: {}", par_value);
}

/// Pricing of a Fixed Rate Loan starting +2Y
/// The loan starts in 2 years and ends in 5 years. The notional is 100,000. The rate is 5% compounded annually.
fn forward_starting_pricing() {
    print_title("Pricing of a Fixed Rate Loan starting +2Y");

    let market_store = create_store().unwrap();
    let ref_date = market_store.reference_date();

    let start_date = ref_date + Period::new(6, TimeUnit::Months);
    let end_date = start_date + Period::new(5, TimeUnit::Years);

    let notional = 100_000.0;
    let rate = InterestRate::new(
        Number::new(0.05),
        Compounding::Simple,
        Frequency::Annual,
        DayCounter::Actual360,
    );

    let mut instrument = MakeFixedRateInstrument::new()
        .with_start_date(start_date)
        .with_end_date(end_date)
        .with_rate(rate)
        .with_payment_frequency(Frequency::Semiannual)
        .with_currency(Currency::USD)
        .with_side(Side::Receive)
        .bullet()
        .with_discount_curve_id(Some(0))
        .with_notional(notional)
        .build()
        .unwrap();

    let indexer = IndexingVisitor::new();
    let _ = indexer.visit(&mut instrument);

    let model = SimpleModel::new(&market_store);

    let data = model.gen_market_data(&indexer.request()).unwrap();
    print_table(instrument.cashflows(), &data);

    let npv_visitor = NPVConstVisitor::new(&data, true);
    let npv = npv_visitor.visit(&instrument);

    print_separator();
    println!("NPV: {}", npv.unwrap());

    let start_accrual = Date::new(2024, 9, 1);
    let end_accrual = Date::new(2024, 10, 1);
    let accrued_amount = instrument
        .cashflows()
        .iter()
        .fold(Number::new(0.0), |acc, cf| {
            acc + cf.accrued_amount(start_accrual, end_accrual).unwrap()
        });
    println!(
        "Accrued Amount between {} and {}: {}",
        start_accrual, end_accrual, accrued_amount
    );
}

/// Pricing of a Fixed Rate Loan starting -2Y
/// The loan starts in 2 years and ends in 5 years. The notional is 100,000. The rate is 5% compounded annually.
fn already_started_pricing() {
    print_title("Pricing of a Fixed Rate Loan starting +2Y");

    let market_store = create_store().unwrap();
    let ref_date = market_store.reference_date();

    let start_date = ref_date - Period::new(2, TimeUnit::Months);
    let end_date = start_date + Period::new(5, TimeUnit::Years);
    let notional = 100_000.0;
    let rate = InterestRate::new(
        Number::new(0.05),
        Compounding::Simple,
        Frequency::Annual,
        DayCounter::Actual360,
    );

    let builder = MakeFixedRateInstrument::new()
        .with_start_date(start_date)
        .with_end_date(end_date)
        .with_currency(Currency::USD)
        .with_payment_frequency(Frequency::Semiannual)
        .with_side(Side::Receive)
        .bullet()
        .with_discount_curve_id(Some(2))
        .with_notional(notional);

    let mut instrument = builder.clone().with_rate(rate).build().unwrap();

    let indexer = IndexingVisitor::new();
    let result = indexer.visit(&mut instrument);
    match result {
        Ok(_) => (),
        Err(e) => panic!("IndexingVisitor failed with error: {}", e),
    }

    let model = SimpleModel::new(&market_store);

    let data = model.gen_market_data(&indexer.request()).unwrap();
    print_table(instrument.cashflows(), &data);

    let npv_visitor = NPVConstVisitor::new(&data, true);
    let npv = npv_visitor.visit(&instrument).unwrap();

    print_separator();
    println!("NPV: {}", npv.clone());

    let start_accrual = Date::new(2024, 9, 1);
    let end_accrual = Date::new(2024, 10, 1);
    let accrued_amount = instrument
        .cashflows()
        .iter()
        .fold(Number::new(0.0), |acc, cf| {
            acc + cf.accrued_amount(start_accrual, end_accrual).unwrap()
        });
    println!(
        "Accrued Amount between {} and {}: {}",
        start_accrual, end_accrual, accrued_amount
    );

    let par_visitor = ParValueConstVisitor::new(&data);
    let par_value = par_visitor.visit(&instrument).unwrap();
    println!("Par Value: {}", par_value);
}

#[cfg(feature = "aad")]
/// Sensitivity analysis using AAD
fn sensitivity_analysis_using_aad() {
    let market_store = create_store().unwrap();
    TAPE.with(|tape| tape.activate()); // Activate the tape for AAD
    let ref_date = market_store.reference_date();

    let start_date = ref_date - Period::new(2, TimeUnit::Months);
    let end_date = start_date + Period::new(5, TimeUnit::Years);
    let notional = 100_000.0;

    let rate_val = Number::new(0.05);
    let rate = InterestRate::new(
        rate_val,
        Compounding::Simple,
        Frequency::Annual,
        DayCounter::Actual360,
    );

    let builder = MakeFixedRateInstrument::new()
        .with_start_date(start_date)
        .with_end_date(end_date)
        .with_currency(Currency::USD)
        .with_payment_frequency(Frequency::Semiannual)
        .with_side(Side::Receive)
        .bullet()
        .with_discount_curve_id(Some(2))
        .with_notional(notional);

    let mut instrument = builder.clone().with_rate(rate).build().unwrap();
    let indexer = IndexingVisitor::new();

    let result = indexer.visit(&mut instrument);
    match result {
        Ok(_) => (),
        Err(e) => panic!("IndexingVisitor failed with error: {}", e),
    }

    let model = SimpleModel::new(&market_store);
    let data = model.gen_market_data(&indexer.request()).unwrap();
    let npv_visitor = NPVConstVisitor::new(&data, true);
    let npv = npv_visitor.visit(&instrument).unwrap();

    let dr_dnpv = TAPE.with(|tape| tape.derivative(&npv, &rate_val)) * 0.01;

    let new_rate = InterestRate::new(
        Number::new(0.06),
        Compounding::Simple,
        Frequency::Annual,
        DayCounter::Actual360,
    );

    let mut new_instrument = builder.with_rate(new_rate).build().unwrap();
    let _ = indexer.visit(&mut new_instrument);
    let new_data = model.gen_market_data(&indexer.request()).unwrap();
    let new_visitor = NPVConstVisitor::new(&new_data, true);
    let new_npv = new_visitor.visit(&new_instrument).unwrap();
    println!("AAD derivative: {}", dr_dnpv);
    println!("Numerical derivative: {}", new_npv - npv);
}

fn main() {
    starting_today_pricing();
    println!("\n");
    forward_starting_pricing();
    println!("\n");
    already_started_pricing();
    #[cfg(feature = "aad")]
    println!("\n");
    #[cfg(feature = "aad")]
    sensitivity_analysis_using_aad();
}
