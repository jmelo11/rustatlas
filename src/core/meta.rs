use crate::time::date::Date;

#[derive(Debug, PartialEq, Eq)]
pub struct MetaExchangeRateMeta {
    pub currency: u16,
    pub reference_date: Date,
}

impl MetaExchangeRateMeta {
    pub fn new(currency: u16, reference_date: Date) -> MetaExchangeRateMeta {
        MetaExchangeRateMeta {
            currency,
            reference_date,
        }
    }
}
#[derive(Debug, PartialEq, Eq)]
pub struct MetaDiscountFactor {
    pub discount_curve_id: u16,
    pub reference_date: Date,
}

impl MetaDiscountFactor {
    pub fn new(discount_curve_id: u16, reference_date: Date) -> MetaDiscountFactor {
        MetaDiscountFactor {
            discount_curve_id,
            reference_date,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct MetaForwardRate {
    pub forward_curve_id: u16,
    pub start_date: Date,
    pub end_date: Date,
}

impl MetaForwardRate {
    pub fn new(forward_curve_id: u16, start_date: Date, end_date: Date) -> MetaForwardRate {
        MetaForwardRate {
            forward_curve_id,
            start_date,
            end_date,
        }
    }
}
#[derive(Debug, PartialEq, Eq)]
pub struct MetaMarketData {
    df: Option<MetaDiscountFactor>,
    fwd: Option<MetaForwardRate>,
    fx: Option<MetaExchangeRateMeta>,
}

impl MetaMarketData {
    pub fn new(
        df: Option<MetaDiscountFactor>,
        fwd: Option<MetaForwardRate>,
        fx: Option<MetaExchangeRateMeta>,
    ) -> MetaMarketData {
        MetaMarketData { df, fwd, fx }
    }
}

pub struct MarketData {
    df: Option<f64>,
    fwd: Option<f64>,
    fx: Option<f64>,
}

impl MarketData {
    pub fn new(df: Option<f64>, fwd: Option<f64>, fx: Option<f64>) -> MarketData {
        MarketData { df, fwd, fx }
    }
}
