use crate::{currencies::enums::Currency, time::date::Date};

/// # MetaExchangeRate
/// Meta data for an exchange rate. Holds the currency and the reference date required to fetch the
/// exchange rate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MetaExchangeRate {
    currency: Currency,
    reference_date: Date,
}

impl MetaExchangeRate {
    pub fn new(currency: Currency, reference_date: Date) -> MetaExchangeRate {
        MetaExchangeRate {
            currency,
            reference_date,
        }
    }

    pub fn currency(&self) -> Currency {
        self.currency
    }

    pub fn reference_date(&self) -> Date {
        self.reference_date
    }
}

/// # MetaDiscountFactor
/// Meta data for a discount factor. Holds the discount curve id and the reference date required to
/// fetch the discount factor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MetaDiscountFactor {
    discount_curve_id: usize,
    reference_date: Date,
}

impl MetaDiscountFactor {
    pub fn new(discount_curve_id: usize, reference_date: Date) -> MetaDiscountFactor {
        MetaDiscountFactor {
            discount_curve_id,
            reference_date,
        }
    }

    pub fn discount_curve_id(&self) -> usize {
        self.discount_curve_id
    }

    pub fn reference_date(&self) -> Date {
        self.reference_date
    }

    pub fn set_reference_date(&mut self, reference_date: Date) {
        self.reference_date = reference_date;
    }
}

/// # MetaForwardRate
/// Meta data for a forward rate. Holds the forward curve id and the start and end dates required
/// to fetch the forward rate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MetaForwardRate {
    forward_curve_id: usize,
    start_date: Date,
    end_date: Date,
}

impl MetaForwardRate {
    pub fn new(forward_curve_id: usize, start_date: Date, end_date: Date) -> MetaForwardRate {
        MetaForwardRate {
            forward_curve_id,
            start_date,
            end_date,
        }
    }

    pub fn forward_curve_id(&self) -> usize {
        self.forward_curve_id
    }

    pub fn start_date(&self) -> Date {
        self.start_date
    }

    pub fn end_date(&self) -> Date {
        self.end_date
    }
}

/// # MetaMarketData
/// Meta data for market data. Holds all the meta data required to fetch the market data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MetaMarketData {
    id: usize,
    df: Option<MetaDiscountFactor>,
    fwd: Option<MetaForwardRate>,
    fx: Option<MetaExchangeRate>,
}

impl MetaMarketData {
    pub fn new(
        id: usize,
        df: Option<MetaDiscountFactor>,
        fwd: Option<MetaForwardRate>,
        fx: Option<MetaExchangeRate>,
    ) -> MetaMarketData {
        MetaMarketData { id, df, fwd, fx }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn df(&self) -> Option<MetaDiscountFactor> {
        self.df
    }

    pub fn fwd(&self) -> Option<MetaForwardRate> {
        self.fwd
    }

    pub fn fx(&self) -> Option<MetaExchangeRate> {
        self.fx
    }
}

/// # MarketData
/// Market data. Holds all the data required to price a cashflow.
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
