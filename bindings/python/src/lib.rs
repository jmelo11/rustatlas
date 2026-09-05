//! Python bindings for the quantsupport derivative pricing library.

mod context;
mod conv;
mod enums;
mod explore;
mod market;
mod results;
mod time;
mod trades;
mod xva;

use pyo3::prelude::*;

pyo3::create_exception!(
    quantsupport,
    QuantSupportError,
    pyo3::exceptions::PyException,
    "Raised when a quantsupport operation fails."
);

#[pymodule]
fn quantsupport(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Enums
    m.add_class::<enums::Currency>()?;
    m.add_class::<enums::MarketIndex>()?;
    m.add_class::<enums::Side>()?;
    m.add_class::<enums::Compounding>()?;
    m.add_class::<enums::Frequency>()?;
    m.add_class::<enums::DayCounter>()?;
    m.add_class::<enums::TimeUnit>()?;
    m.add_class::<enums::BusinessDayConvention>()?;
    m.add_class::<enums::Request>()?;
    m.add_class::<enums::VolatilityType>()?;
    m.add_class::<enums::SmileType>()?;
    m.add_class::<enums::ScenarioType>()?;
    m.add_class::<enums::OptionType>()?;
    m.add_class::<enums::CapFloorType>()?;
    m.add_class::<enums::CapletFloorletType>()?;
    m.add_class::<enums::PaymentStructure>()?;
    // Time
    m.add_class::<time::Date>()?;
    m.add_class::<time::Period>()?;
    m.add_class::<time::Calendar>()?;
    // Market data & configurations
    m.add_class::<market::QuoteStore>()?;
    m.add_class::<market::FixingStore>()?;
    m.add_class::<market::FxStore>()?;
    m.add_class::<market::CurveConfiguration>()?;
    m.add_class::<market::VolatilitySurfaceConfiguration>()?;
    m.add_class::<market::VolatilityCubeConfiguration>()?;
    m.add_class::<market::SimulationConfiguration>()?;
    m.add_class::<market::Scenario>()?;
    // Constructed element views
    m.add_class::<explore::DiscountCurve>()?;
    m.add_class::<explore::VolatilitySurface>()?;
    m.add_class::<explore::VolatilityCube>()?;
    m.add_class::<explore::Simulation>()?;
    // Context
    m.add_class::<context::DiscountingConfig>()?;
    m.add_class::<context::PricingContext>()?;
    // Trades
    m.add_class::<trades::Swap>()?;
    m.add_class::<trades::CrossCurrencySwap>()?;
    m.add_class::<trades::BasisSwap>()?;
    m.add_class::<trades::FixFloatCrossCurrencySwap>()?;
    m.add_class::<trades::FixedRateBond>()?;
    m.add_class::<trades::FloatingRateNote>()?;
    m.add_class::<trades::FixedRateDeposit>()?;
    m.add_class::<trades::FxForwardPy>()?;
    m.add_class::<trades::FxOptionPy>()?;
    m.add_class::<trades::EquityOption>()?;
    m.add_class::<trades::CreditDefaultSwapPy>()?;
    m.add_class::<trades::CapFloorPy>()?;
    m.add_class::<trades::CapletFloorletPy>()?;
    m.add_class::<trades::RateFuturesPy>()?;
    // Results
    m.add_class::<results::EvaluationResults>()?;
    // XVA
    m.add_class::<xva::XvaConfig>()?;
    m.add_class::<xva::CsaTerms>()?;
    m.add_class::<xva::NettingSet>()?;
    m.add_class::<xva::XvaResult>()?;
    m.add_class::<xva::ExposureProfile>()?;
    m.add("QuantSupportError", m.py().get_type::<QuantSupportError>())?;
    Ok(())
}
