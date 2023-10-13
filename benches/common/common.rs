extern crate rustatlas;
use rustatlas::{
    core::marketstore::MarketStore,
    currencies::enums::Currency,
    rates::{
        enums::Compounding,
        interestrate::InterestRate,
        interestrateindex::{
            enums::InterestRateIndex, iborindex::IborIndex, overnightindex::OvernightIndex,
        },
        traits::HasReferenceDate,
        yieldtermstructure::flatforwardtermstructure::FlatForwardTermStructure,
    },
    time::{
        date::Date,
        daycounter::DayCounter,
        enums::{Frequency, TimeUnit},
        period::Period,
    },
};
use std::collections::HashMap;

#[allow(dead_code)]
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

#[allow(dead_code)]
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

    let forecast_curve_1 = Box::new(FlatForwardTermStructure::new(ref_date, forecast_rate_1));

    let forecast_curve_2 = Box::new(FlatForwardTermStructure::new(ref_date, forecast_rate_2));

    let discount_curve = Box::new(FlatForwardTermStructure::new(ref_date, discount_rate));

    let mut ibor_fixings = HashMap::new();
    ibor_fixings.insert(Date::new(2021, 9, 1), 0.02); // today
    ibor_fixings.insert(Date::new(2021, 8, 31), 0.02); // yesterday

    let ibor_index = IborIndex::new(forecast_curve_1.reference_date())
        .with_fixings(ibor_fixings)
        .with_term_structure(forecast_curve_1)
        .with_frequency(Frequency::Annual);

    let overnight_fixings =
        make_fixings(ref_date - Period::new(1, TimeUnit::Years), ref_date, 0.06);
    let overnigth_index = OvernightIndex::new(forecast_curve_2.reference_date())
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

    let discount_index =
        IborIndex::new(discount_curve.reference_date()).with_term_structure(discount_curve);

    market_store.mut_index_store().add_index(
        "DiscountCurve".to_string(),
        InterestRateIndex::IborIndex(discount_index),
    );
    return market_store;
}
