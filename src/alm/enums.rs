use crate::utils::errors::{AtlasError, Result};
use crate::{
    currencies::enums::Currency,
    instruments::instrument::{Instrument, RateType},
};
use serde::{Deserialize, Serialize};

/// # `PositionType`
/// This enum is used to differentiate between base and simulated positions
#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum PositionType {
    /// Base position type
    Base,
    /// Simulated position type
    Simulated,
}

impl TryFrom<String> for PositionType {
    type Error = AtlasError;

    fn try_from(s: String) -> Result<Self> {
        match s.as_str() {
            "Base" => Ok(Self::Base),
            "Simulated" => Ok(Self::Simulated),
            _ => Err(AtlasError::InvalidValueErr(format!(
                "Invalid position type: {s}",
            ))),
        }
    }
}

impl From<PositionType> for String {
    fn from(position_type: PositionType) -> Self {
        match position_type {
            PositionType::Base => "Base".to_string(),
            PositionType::Simulated => "Simulated".to_string(),
        }
    }
}

/// # `Portfolio`
/// A struct that contains the information needed to define a portfolio.
/// Optional fields are used to filter the portfolio.
#[derive(Clone, Debug)]
pub struct Portfolio {
    id: Option<usize>,
    segment: Option<String>,
    area: Option<String>,
    product_family: Option<String>,
    position_type: Option<PositionType>,
    rate_type: Option<RateType>,
    currency: Option<Currency>,
    instruments: Vec<Instrument>,
}

impl Portfolio {
    /// Creates a new Portfolio with default empty values.
    #[allow(clippy::missing_const_for_fn)]
    #[must_use]
    pub fn new() -> Self {
        Self {
            id: None,
            segment: None,
            product_family: None,
            area: None,
            position_type: None,
            rate_type: None,
            currency: None,
            instruments: Vec::new(),
        }
    }

    /// Returns the portfolio id.
    #[must_use]
    pub const fn id(&self) -> Option<usize> {
        self.id
    }

    /// Returns the portfolio segment.
    #[must_use]
    pub fn segment(&self) -> Option<String> {
        self.segment.clone()
    }

    /// Returns the portfolio product family.
    #[must_use]
    pub fn product_family(&self) -> Option<String> {
        self.product_family.clone()
    }

    /// Returns the portfolio area.
    #[must_use]
    pub fn area(&self) -> Option<String> {
        self.area.clone()
    }

    /// Returns the portfolio position type.
    #[must_use]
    pub const fn position_type(&self) -> Option<PositionType> {
        self.position_type
    }

    /// Returns the portfolio rate type.
    #[must_use]
    pub const fn rate_type(&self) -> Option<RateType> {
        self.rate_type
    }

    /// Returns the portfolio currency.
    #[must_use]
    pub const fn currency(&self) -> Option<Currency> {
        self.currency
    }

    /// Sets the portfolio currency.
    #[must_use]
    pub const fn with_currency(mut self, currency: Currency) -> Self {
        self.currency = Some(currency);
        self
    }

    /// Sets the portfolio rate type.
    #[must_use]
    pub const fn with_rate_type(mut self, rate_type: RateType) -> Self {
        self.rate_type = Some(rate_type);
        self
    }

    /// Sets the portfolio id.
    #[must_use]
    pub const fn with_id(mut self, id: usize) -> Self {
        self.id = Some(id);
        self
    }

    /// Sets the portfolio segment.
    #[must_use]
    pub fn with_segment(mut self, segment: String) -> Self {
        self.segment = Some(segment);
        self
    }

    /// Sets the portfolio product family.
    #[must_use]
    pub fn with_product_family(mut self, product_family: String) -> Self {
        self.product_family = Some(product_family);
        self
    }

    /// Sets the portfolio area.
    #[must_use]
    pub fn with_area(mut self, area: String) -> Self {
        self.area = Some(area);
        self
    }

    /// Sets the portfolio position type.
    #[must_use]
    pub const fn with_position_type(mut self, position_type: PositionType) -> Self {
        self.position_type = Some(position_type);
        self
    }

    /// Sets the portfolio instruments.
    #[must_use]
    pub fn with_instruments(mut self, instruments: Vec<Instrument>) -> Self {
        self.instruments = instruments;
        self
    }

    /// Adds an instrument to the portfolio.
    pub fn add_instrument(&mut self, instrument: Instrument) {
        self.instruments.push(instrument);
    }

    /// Returns a reference to the portfolio instruments.
    #[must_use]
    pub fn instruments(&self) -> &[Instrument] {
        &self.instruments
    }

    /// Returns a mutable reference to the portfolio instruments.
    pub fn instruments_mut(&mut self) -> &mut [Instrument] {
        &mut self.instruments
    }
}

impl Default for Portfolio {
    fn default() -> Self {
        Self::new()
    }
}

/// # `AccountType`
/// A struct that contains the information needed to define an account type.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AccountType {
    /// Asset account type
    Asset,
    /// Liability account type
    Liability,
    /// Equity account type
    Equity,
    /// Revenue account type
    Revenue,
    /// Expense account type
    Expense,
}

impl TryFrom<String> for AccountType {
    type Error = AtlasError;

    fn try_from(s: String) -> Result<Self> {
        match s.as_str() {
            "Asset" => Ok(Self::Asset),
            "Liability" => Ok(Self::Liability),
            "Equity" => Ok(Self::Equity),
            "Revenue" => Ok(Self::Revenue),
            "Expense" => Ok(Self::Expense),
            _ => Err(AtlasError::InvalidValueErr(format!(
                "Invalid account type: {s}",
            ))),
        }
    }
}

impl From<AccountType> for String {
    fn from(account_type: AccountType) -> Self {
        match account_type {
            AccountType::Asset => "Asset".to_string(),
            AccountType::Liability => "Liability".to_string(),
            AccountType::Equity => "Equity".to_string(),
            AccountType::Revenue => "Revenue".to_string(),
            AccountType::Expense => "Expense".to_string(),
        }
    }
}

/// # `EvaluationMode`
/// A struct that contains the information needed to define
/// an evaluation mode when running simulations and building instruments.
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum EvaluationMode {
    /// FTP rate evaluation mode
    FTPRate,
    /// Client rate evaluation mode
    ClientRate,
}

impl TryFrom<String> for EvaluationMode {
    type Error = AtlasError;

    fn try_from(s: String) -> Result<Self> {
        match s.as_str() {
            "FTPRate" => Ok(Self::FTPRate),
            "ClientRate" => Ok(Self::ClientRate),
            _ => Err(AtlasError::InvalidValueErr(format!(
                "Invalid evaluation mode: {s}",
            ))),
        }
    }
}

impl From<EvaluationMode> for String {
    fn from(evaluation_mode: EvaluationMode) -> Self {
        match evaluation_mode {
            EvaluationMode::FTPRate => "FTPRate".to_string(),
            EvaluationMode::ClientRate => "ClientRate".to_string(),
        }
    }
}
