//! Python trade specifications.
//!
//! Each Python trade class stores plain parameters; the corresponding Rust
//! instrument is built on demand — as `DualFwd` for AD-enabled pricing or as
//! `f64` for XVA contingent-claim decomposition.

use pyo3::prelude::*;
use quantsupport::prelude::{
    Compounding, ContingentClaim, Currency, Date, DayCounter, DualFwd, FloatFloatCrossCurrencySwap,
    FloatFloatCrossCurrencySwapTrade, Frequency, IntoContingentClaims, LegsProvider,
    MakeFloatFloatCrossCurrencySwap, MakeSwap, MarketIndex, RateDefinition, Side, Swap as QsSwap,
    SwapTrade,
};

use crate::conv::{
    extract_compounding, extract_currency, extract_date, extract_day_counter, extract_frequency,
    extract_market_index, extract_side, qs_err,
};

/// A vanilla fixed-vs-floating interest rate swap.
#[pyclass(name = "Swap", from_py_object)]
#[derive(Clone)]
pub struct Swap {
    pub identifier: String,
    pub start_date: Date,
    pub maturity_date: Date,
    pub notional: f64,
    pub fixed_rate: f64,
    pub spread: Option<f64>,
    pub currency: Currency,
    pub market_index: MarketIndex,
    pub side: Side,
    pub day_counter: DayCounter,
    pub compounding: Compounding,
    pub rate_frequency: Frequency,
    pub fixed_leg_frequency: Frequency,
    pub floating_leg_frequency: Frequency,
    pub trade_date: Option<Date>,
}

macro_rules! build_swap_instrument {
    ($self:expr, $t:ty) => {{
        let mut mk = MakeSwap::<$t>::default()
            .with_identifier($self.identifier.clone())
            .with_start_date($self.start_date)
            .with_maturity_date($self.maturity_date)
            .with_fixed_rate($self.fixed_rate)
            .with_notional($self.notional)
            .with_rate_definition(RateDefinition::new(
                $self.day_counter,
                $self.compounding,
                $self.rate_frequency,
            ))
            .with_currency($self.currency)
            .with_market_index($self.market_index.clone())
            .with_side($self.side)
            .with_fixed_leg_frequency($self.fixed_leg_frequency)
            .with_floating_leg_frequency($self.floating_leg_frequency);
        if let Some(spread) = $self.spread {
            mk = mk.with_spread(spread);
        }
        mk.build().map_err(qs_err)
    }};
}

impl Swap {
    fn trade_date(&self) -> Date {
        self.trade_date.unwrap_or(self.start_date)
    }

    /// Builds an AD-enabled trade for pricing.
    pub fn build_trade_dual(&self) -> PyResult<SwapTrade<DualFwd>> {
        let instrument: QsSwap<DualFwd> = build_swap_instrument!(self, DualFwd)?;
        Ok(SwapTrade::new(
            instrument,
            self.trade_date(),
            self.notional,
            self.side,
        ))
    }

    /// Decomposes the trade into contingent claims for XVA.
    pub fn claims(&self) -> PyResult<Vec<ContingentClaim>> {
        let instrument: QsSwap<f64> = build_swap_instrument!(self, f64)?;
        let trade = SwapTrade::new(instrument, self.trade_date(), self.notional, self.side);
        trade.into_contingent_claims().map_err(qs_err)
    }
}

#[pymethods]
impl Swap {
    #[new]
    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (
        identifier,
        start_date,
        maturity_date,
        notional,
        fixed_rate,
        currency,
        market_index,
        side = None,
        spread = None,
        day_counter = None,
        compounding = None,
        rate_frequency = None,
        fixed_leg_frequency = None,
        floating_leg_frequency = None,
        trade_date = None,
    ))]
    fn new(
        identifier: String,
        start_date: &Bound<'_, PyAny>,
        maturity_date: &Bound<'_, PyAny>,
        notional: f64,
        fixed_rate: f64,
        currency: &Bound<'_, PyAny>,
        market_index: &Bound<'_, PyAny>,
        side: Option<&Bound<'_, PyAny>>,
        spread: Option<f64>,
        day_counter: Option<&Bound<'_, PyAny>>,
        compounding: Option<&Bound<'_, PyAny>>,
        rate_frequency: Option<&Bound<'_, PyAny>>,
        fixed_leg_frequency: Option<&Bound<'_, PyAny>>,
        floating_leg_frequency: Option<&Bound<'_, PyAny>>,
        trade_date: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<Self> {
        Ok(Self {
            identifier,
            start_date: extract_date(start_date)?,
            maturity_date: extract_date(maturity_date)?,
            notional,
            fixed_rate,
            spread,
            currency: extract_currency(currency)?,
            market_index: extract_market_index(market_index)?,
            side: side
                .map(extract_side)
                .transpose()?
                .unwrap_or(Side::LongReceive),
            day_counter: day_counter
                .map(extract_day_counter)
                .transpose()?
                .unwrap_or(DayCounter::Actual360),
            compounding: compounding
                .map(extract_compounding)
                .transpose()?
                .unwrap_or(Compounding::Simple),
            rate_frequency: rate_frequency
                .map(extract_frequency)
                .transpose()?
                .unwrap_or(Frequency::Semiannual),
            fixed_leg_frequency: fixed_leg_frequency
                .map(extract_frequency)
                .transpose()?
                .unwrap_or(Frequency::Semiannual),
            floating_leg_frequency: floating_leg_frequency
                .map(extract_frequency)
                .transpose()?
                .unwrap_or(Frequency::Semiannual),
            trade_date: trade_date.map(extract_date).transpose()?,
        })
    }

    #[getter]
    fn identifier(&self) -> &str {
        &self.identifier
    }

    #[getter(start_date)]
    fn start_date_py(&self) -> crate::time::Date {
        crate::time::Date {
            inner: self.start_date,
        }
    }

    #[getter(maturity_date)]
    fn maturity_date_py(&self) -> crate::time::Date {
        crate::time::Date {
            inner: self.maturity_date,
        }
    }

    #[getter(notional)]
    fn notional_py(&self) -> f64 {
        self.notional
    }

    #[getter(fixed_rate)]
    fn fixed_rate_py(&self) -> f64 {
        self.fixed_rate
    }

    #[getter(currency)]
    fn currency_py(&self) -> crate::enums::Currency {
        self.currency.into()
    }

    #[getter(market_index)]
    fn market_index_py(&self) -> crate::enums::MarketIndex {
        crate::enums::MarketIndex {
            inner: self.market_index.clone(),
        }
    }

    #[getter(side)]
    fn side_py(&self) -> crate::enums::Side {
        self.side.into()
    }

    fn __repr__(&self) -> String {
        format!(
            "Swap(identifier='{}', start_date='{}', maturity_date='{}', notional={}, fixed_rate={})",
            self.identifier, self.start_date, self.maturity_date, self.notional, self.fixed_rate
        )
    }
}

/// A float-vs-float cross-currency swap.
#[pyclass(name = "CrossCurrencySwap", from_py_object)]
#[derive(Clone)]
pub struct CrossCurrencySwap {
    pub identifier: String,
    pub start_date: Date,
    pub maturity_date: Date,
    pub domestic_notional: f64,
    pub foreign_notional: f64,
    pub domestic_spread: f64,
    pub foreign_spread: f64,
    pub domestic_currency: Currency,
    pub foreign_currency: Currency,
    pub domestic_market_index: MarketIndex,
    pub foreign_market_index: MarketIndex,
    pub side: Side,
    pub domestic_leg_frequency: Frequency,
    pub foreign_leg_frequency: Frequency,
    pub trade_date: Option<Date>,
}

macro_rules! build_xccy_instrument {
    ($self:expr, $t:ty) => {{
        MakeFloatFloatCrossCurrencySwap::<$t>::default()
            .with_identifier($self.identifier.clone())
            .with_start_date($self.start_date)
            .with_maturity_date($self.maturity_date)
            .with_domestic_notional($self.domestic_notional)
            .with_foreign_notional($self.foreign_notional)
            .with_domestic_spread($self.domestic_spread)
            .with_foreign_spread($self.foreign_spread)
            .with_domestic_currency($self.domestic_currency)
            .with_foreign_currency($self.foreign_currency)
            .with_domestic_market_index($self.domestic_market_index.clone())
            .with_foreign_market_index($self.foreign_market_index.clone())
            .with_side($self.side)
            .with_domestic_leg_frequency($self.domestic_leg_frequency)
            .with_foreign_leg_frequency($self.foreign_leg_frequency)
            .build()
            .map_err(qs_err)
    }};
}

impl CrossCurrencySwap {
    fn trade_date(&self) -> Date {
        self.trade_date.unwrap_or(self.start_date)
    }

    /// Builds an AD-enabled trade for pricing.
    pub fn build_trade_dual(&self) -> PyResult<FloatFloatCrossCurrencySwapTrade<DualFwd>> {
        let instrument: FloatFloatCrossCurrencySwap<DualFwd> =
            build_xccy_instrument!(self, DualFwd)?;
        Ok(FloatFloatCrossCurrencySwapTrade::new(
            instrument,
            self.trade_date(),
            self.domestic_notional,
            self.foreign_notional,
            self.side,
        ))
    }

    /// Decomposes the trade into contingent claims for XVA.
    pub fn claims(&self) -> PyResult<Vec<ContingentClaim>> {
        let instrument: FloatFloatCrossCurrencySwap<f64> = build_xccy_instrument!(self, f64)?;
        instrument
            .legs()
            .to_vec()
            .into_contingent_claims(&self.identifier)
            .map_err(qs_err)
    }
}

#[pymethods]
impl CrossCurrencySwap {
    #[new]
    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (
        identifier,
        start_date,
        maturity_date,
        domestic_notional,
        foreign_notional,
        domestic_currency,
        foreign_currency,
        domestic_market_index,
        foreign_market_index,
        side = None,
        domestic_spread = 0.0,
        foreign_spread = 0.0,
        domestic_leg_frequency = None,
        foreign_leg_frequency = None,
        trade_date = None,
    ))]
    fn new(
        identifier: String,
        start_date: &Bound<'_, PyAny>,
        maturity_date: &Bound<'_, PyAny>,
        domestic_notional: f64,
        foreign_notional: f64,
        domestic_currency: &Bound<'_, PyAny>,
        foreign_currency: &Bound<'_, PyAny>,
        domestic_market_index: &Bound<'_, PyAny>,
        foreign_market_index: &Bound<'_, PyAny>,
        side: Option<&Bound<'_, PyAny>>,
        domestic_spread: f64,
        foreign_spread: f64,
        domestic_leg_frequency: Option<&Bound<'_, PyAny>>,
        foreign_leg_frequency: Option<&Bound<'_, PyAny>>,
        trade_date: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<Self> {
        Ok(Self {
            identifier,
            start_date: extract_date(start_date)?,
            maturity_date: extract_date(maturity_date)?,
            domestic_notional,
            foreign_notional,
            domestic_spread,
            foreign_spread,
            domestic_currency: extract_currency(domestic_currency)?,
            foreign_currency: extract_currency(foreign_currency)?,
            domestic_market_index: extract_market_index(domestic_market_index)?,
            foreign_market_index: extract_market_index(foreign_market_index)?,
            side: side
                .map(extract_side)
                .transpose()?
                .unwrap_or(Side::LongReceive),
            domestic_leg_frequency: domestic_leg_frequency
                .map(extract_frequency)
                .transpose()?
                .unwrap_or(Frequency::Semiannual),
            foreign_leg_frequency: foreign_leg_frequency
                .map(extract_frequency)
                .transpose()?
                .unwrap_or(Frequency::Semiannual),
            trade_date: trade_date.map(extract_date).transpose()?,
        })
    }

    #[getter]
    fn identifier(&self) -> &str {
        &self.identifier
    }

    #[getter(start_date)]
    fn start_date_py(&self) -> crate::time::Date {
        crate::time::Date {
            inner: self.start_date,
        }
    }

    #[getter(maturity_date)]
    fn maturity_date_py(&self) -> crate::time::Date {
        crate::time::Date {
            inner: self.maturity_date,
        }
    }

    #[getter(domestic_currency)]
    fn domestic_currency_py(&self) -> crate::enums::Currency {
        self.domestic_currency.into()
    }

    #[getter(foreign_currency)]
    fn foreign_currency_py(&self) -> crate::enums::Currency {
        self.foreign_currency.into()
    }

    #[getter(domestic_market_index)]
    fn domestic_market_index_py(&self) -> crate::enums::MarketIndex {
        crate::enums::MarketIndex {
            inner: self.domestic_market_index.clone(),
        }
    }

    #[getter(foreign_market_index)]
    fn foreign_market_index_py(&self) -> crate::enums::MarketIndex {
        crate::enums::MarketIndex {
            inner: self.foreign_market_index.clone(),
        }
    }

    #[getter(side)]
    fn side_py(&self) -> crate::enums::Side {
        self.side.into()
    }

    fn __repr__(&self) -> String {
        format!(
            "CrossCurrencySwap(identifier='{}', start_date='{}', maturity_date='{}')",
            self.identifier, self.start_date, self.maturity_date
        )
    }
}
