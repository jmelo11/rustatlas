
extern crate rustatlas;
use rustatlas::prelude::*;
use std::collections::HashMap;

pub fn print_separator() {
    println!("------------------------");
}

pub fn print_title(title: &str) {
    print_separator();
    println!("{}", title);
    print_separator();
}

pub fn print_table(cashflows: &[Cashflow], market_data: &[MarketData]) {
    println!(
        "{:10} | {:10} | {:10} | {:10}| {:10}",
        "Date", "Amount", "DF", "FWD", "FX"
    );
    for (cf, md) in cashflows.iter().zip(market_data) {
        let date = format!("{:10}", cf.payment_date().to_string());
        let amount = format!("{:10.2}", cf.amount()); // Assuming `cf.amount()` is a float

        let df = match md.df() {
            Some(df) => format!("{:10.2}", df),
            None => "None      ".to_string(), // 10 characters wide
        };

        let fx = match md.fx() {
            Some(fx) => format!("{:10.2}", fx),
            None => "None      ".to_string(), // 10 characters wide
        };

        let fwd = match md.fwd() {
            Some(fwd) => format!("{:10.3}", fwd),
            None => "None      ".to_string(), // 10 characters wide
        };

        println!("{} | {} | {} | {} | {}", date, amount, df, fwd, fx);
    }
}

fn make_fixings(start: Date, end: Date, rate: f64) -> HashMap<Date, f64> {
    let mut fixings = HashMap::new();
    let mut seed = start;
    let mut init = 100.0;
    while seed <= end {
        fixings.insert(seed, init);
        seed = seed + Period::new(1, TimeUnit::Days);
        init = init * (1.0 + rate * 1.0 / 360.0);
    }
    return fixings;
}

pub fn create_store() -> MarketStore {
    let ref_date = Date::new(2021, 9, 1);
    let local_currency = Currency::USD;
    let mut market_store = MarketStore::new(ref_date, local_currency);

    let forecast_rate_1 = InterestRate::new(
        0.02,
        Compounding::Simple,
        Frequency::Annual,
        DayCounter::Actual360,
    );

    let forecast_rate_2 = InterestRate::new(
        0.03,
        Compounding::Simple,
        Frequency::Annual,
        DayCounter::Actual360,
    );

    let discount_rate = InterestRate::new(
        0.05,
        Compounding::Simple,
        Frequency::Annual,
        DayCounter::Actual360,
    );

    let forecast_curve_1 = YieldTermStructure::FlatForwardTermStructure(
        FlatForwardTermStructure::new(ref_date, forecast_rate_1),
    );

    let forecast_curve_2 = YieldTermStructure::FlatForwardTermStructure(
        FlatForwardTermStructure::new(ref_date, forecast_rate_2),
    );

    let discount_curve = YieldTermStructure::FlatForwardTermStructure(
        FlatForwardTermStructure::new(ref_date, discount_rate),
    );

    let mut ibor_fixings = HashMap::new();
    ibor_fixings.insert(Date::new(2021, 9, 1), 0.02); // today
    ibor_fixings.insert(Date::new(2021, 8, 31), 0.02); // yesterday

    let ibor_index = IborIndex::new()
        .with_fixings(ibor_fixings)
        .with_term_structure(forecast_curve_1)
        .with_frequency(Frequency::Annual);

    let overnight_fixings =
        make_fixings(ref_date - Period::new(1, TimeUnit::Years), ref_date, 0.06);
    let overnigth_index = OvernightIndex::new()
        .with_term_structure(forecast_curve_2)
        .with_fixings(overnight_fixings);

    market_store.mut_index_store().add_index(
        "ForecastCurve 1".to_string(),
        InterestRateIndex::IborIndex(ibor_index),
    );

    market_store.mut_index_store().add_index(
        "ForecastCurve 2".to_string(),
        InterestRateIndex::OvernightIndex(overnigth_index),
    );

    let discount_index = IborIndex::new().with_term_structure(discount_curve);

    market_store.mut_index_store().add_index(
        "DiscountCurve".to_string(),
        InterestRateIndex::IborIndex(discount_index),
    );
    return market_store;
}
