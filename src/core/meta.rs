use crate::time::date::Date;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MetaExchangeRate {
    pub currency: u16,
    pub reference_date: Date,
}

impl MetaExchangeRate {
    pub fn new(currency: u16, reference_date: Date) -> MetaExchangeRate {
        MetaExchangeRate {
            currency,
            reference_date,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MetaDiscountFactor {
    pub discount_curve_id: usize,
    pub reference_date: Date,
}

impl MetaDiscountFactor {
    pub fn new(discount_curve_id: usize, reference_date: Date) -> MetaDiscountFactor {
        MetaDiscountFactor {
            discount_curve_id,
            reference_date,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MetaForwardRate {
    pub forward_curve_id: usize,
    pub start_date: Date,
    pub end_date: Date,
}

impl MetaForwardRate {
    pub fn new(forward_curve_id: usize, start_date: Date, end_date: Date) -> MetaForwardRate {
        MetaForwardRate {
            forward_curve_id,
            start_date,
            end_date,
        }
    }
}

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
