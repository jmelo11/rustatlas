use std::collections::HashMap;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::{
    quotes::{
        quote::{Quote, QuoteDetails, QuoteLevels},
        quoteselector::QuoteSelector,
    },
    time::date::Date,
    utils::errors::QSError,
};

/// Serde-friendly single quote record, e.g. `{"identifier": "OIS_USD_SOFR_1Y", "mid": 0.041}`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteRecord {
    /// Quote identifier parseable into [`QuoteDetails`].
    pub identifier: String,
    /// Mid level.
    #[serde(default)]
    pub mid: Option<f64>,
    /// Bid level.
    #[serde(default)]
    pub bid: Option<f64>,
    /// Ask level.
    #[serde(default)]
    pub ask: Option<f64>,
}

/// Serde-friendly quote store input:
/// `{"reference_date": "2025-11-11", "quotes": [{"identifier": ..., "mid": ...}, ...]}`.
///
/// Deserializable from JSON files or Python dicts, and convertible into a
/// [`QuoteStore`] via [`TryFrom`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteStoreRecords {
    /// Reference date of the quotes.
    pub reference_date: Date,
    /// The quote records.
    pub quotes: Vec<QuoteRecord>,
}

impl TryFrom<QuoteStoreRecords> for QuoteStore {
    type Error = QSError;

    fn try_from(records: QuoteStoreRecords) -> Result<Self, Self::Error> {
        let mut store = Self::new(records.reference_date);
        for rec in records.quotes {
            let details = QuoteDetails::from_str(&rec.identifier)?;
            let levels = QuoteLevels::new(rec.mid, rec.bid, rec.ask);
            store.add_quote(Quote::new(details, levels));
        }
        Ok(store)
    }
}

/// Provider of market data loaded from quotes.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QuoteStore {
    reference_date: Date,
    quotes: HashMap<String, Quote>,
}

impl QuoteStore {
    /// Creates an empty market data provider.
    #[must_use]
    pub fn new(reference_date: Date) -> Self {
        Self {
            reference_date,
            quotes: HashMap::new(),
        }
    }
    /// Returns the reference date for the provider.
    #[must_use]
    pub const fn reference_date(&self) -> Date {
        self.reference_date
    }

    /// Adds a market quote to the provider, indexed by its identifier.
    pub fn add_quote(&mut self, quote: Quote) {
        let id = quote.details().identifier();
        self.quotes.insert(id, quote);
    }

    /// Returns a quote by identifier.
    #[must_use]
    pub fn quote(&self, identifier: &str) -> Option<&Quote> {
        self.quotes.get(identifier)
    }

    /// Returns all stored quotes.
    #[must_use]
    pub const fn quotes(&self) -> &HashMap<String, Quote> {
        &self.quotes
    }
}

impl QuoteSelector for QuoteStore {
    fn select(&self, identifier: &str) -> Option<Quote> {
        self.quotes.get(identifier).cloned()
    }

    fn reference_date(&self) -> Date {
        self.reference_date
    }
}
