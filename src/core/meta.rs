use crate::{
    currencies::enums::Currency,
    prelude::{Compounding, Frequency},
    time::date::Date,
};

/// # ExchangeRateRequest
/// Meta data for an exchange rate. Holds the currency and the reference date required to fetch the
/// exchange rate.
///
/// ## Parameters
/// * `currency` - The currency of the exchange rate.
/// * `reference_date` - The reference date of the exchange rate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExchangeRateRequest {
    first_currency: Currency,
    second_currency: Option<Currency>,
    reference_date: Option<Date>,
}

impl ExchangeRateRequest {
    pub fn new(
        first_currency: Currency,
        second_currency: Option<Currency>,
        reference_date: Option<Date>,
    ) -> ExchangeRateRequest {
        ExchangeRateRequest {
            first_currency,
            second_currency,
            reference_date,
        }
    }

    pub fn first_currency(&self) -> Currency {
        self.first_currency
    }

    pub fn second_currency(&self) -> Option<Currency> {
        self.second_currency
    }

    pub fn reference_date(&self) -> Option<Date> {
        self.reference_date
    }
}

/// # DiscountFactorRequest
/// Meta data for a discount factor. Holds the discount curve id and the reference date required to
/// fetch the discount factor.
///
/// ## Parameters
/// * `discount_curve_id` - The discount curve id of the discount factor.
/// * `reference_date` - The reference date of the discount factor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DiscountFactorRequest {
    provider_id: usize,
    reference_date: Date,
}

impl DiscountFactorRequest {
    pub fn new(provider_id: usize, reference_date: Date) -> DiscountFactorRequest {
        DiscountFactorRequest {
            provider_id,
            reference_date,
        }
    }

    pub fn provider_id(&self) -> usize {
        self.provider_id
    }

    pub fn reference_date(&self) -> Date {
        self.reference_date
    }

    pub fn set_reference_date(&mut self, reference_date: Date) {
        self.reference_date = reference_date;
    }
}

/// # ForwardRateRequest
/// Meta data for a forward rate. Holds the forward curve id and the start and end dates required
/// to fetch the forward rate.
///
/// ## Parameters
/// * `forward_curve_id` - The forward curve id of the forward rate.
/// * `start_date` - The start date of the forward rate.
/// * `end_date` - The end date of the forward rate.
/// * `compounding` - The compounding of the forward rate.
/// * `frequency` - The frequency of the forward rate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ForwardRateRequest {
    provider_id: usize,
    start_date: Date,
    end_date: Date,
    fixing_date: Date,
    compounding: Compounding,
    frequency: Frequency,
}

impl ForwardRateRequest {
    pub fn new(
        provider_id: usize,
        start_date: Date,
        end_date: Date,
        fixing_date: Date,
        compounding: Compounding,
        frequency: Frequency,
    ) -> ForwardRateRequest {
        ForwardRateRequest {
            provider_id,
            start_date,
            end_date,
            fixing_date,
            compounding,
            frequency,
        }
    }

    pub fn provider_id(&self) -> usize {
        self.provider_id
    }

    pub fn start_date(&self) -> Date {
        self.start_date
    }

    pub fn end_date(&self) -> Date {
        self.end_date
    }

    pub fn fixing_date(&self) -> Date {
        self.fixing_date
    }

    pub fn compounding(&self) -> Compounding {
        self.compounding
    }

    pub fn frequency(&self) -> Frequency {
        self.frequency
    }
}

/// # MarketRequest
/// Meta data for market data. Holds all the meta data required to fetch the market data.
///
/// ## Parameters
/// * `id` - The id of the market data.
/// * `df` - The discount factor meta data.
/// * `fwd` - The forward rate meta data.
/// * `fx` - The exchange rate meta data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MarketRequest {
    id: usize,
    df: Option<DiscountFactorRequest>,
    fwd: Option<ForwardRateRequest>,
    fx: Option<ExchangeRateRequest>,
}

impl MarketRequest {
    pub fn new(
        id: usize,
        df: Option<DiscountFactorRequest>,
        fwd: Option<ForwardRateRequest>,
        fx: Option<ExchangeRateRequest>,
    ) -> MarketRequest {
        MarketRequest { id, df, fwd, fx }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn df(&self) -> Option<DiscountFactorRequest> {
        self.df
    }

    pub fn fwd(&self) -> Option<ForwardRateRequest> {
        self.fwd
    }

    pub fn fx(&self) -> Option<ExchangeRateRequest> {
        self.fx
    }
}

/// # MarketDataNode
/// Market data. Holds all the data required to price a cashflow.
///
/// ## Parameters
/// * `id` - The id of the market data.
/// * `df` - The discount factor.
/// * `fwd` - The forward rate.
/// * `fx` - The exchange rate.
pub struct MarketData {
    id: usize,
    df: Option<f64>,
    fwd: Option<f64>,
    fx: Option<f64>,
}

impl MarketData {
    pub fn new(id: usize, df: Option<f64>, fwd: Option<f64>, fx: Option<f64>) -> MarketData {
        MarketData { id, df, fwd, fx }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn df(&self) -> Option<f64> {
        self.df
    }

    pub fn fwd(&self) -> Option<f64> {
        self.fwd
    }

    pub fn fx(&self) -> Option<f64> {
        self.fx
    }
}
