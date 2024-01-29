use crate::{
    currencies::enums::Currency,
    instruments::{
        instrument::{Instrument, PositionType, RateType},
        traits::Structure,
    },
};
use crate::utils::errors::{AtlasError, Result};
use serde::{Deserialize, Serialize};

/// # Portfolio
/// A struct that contains the information needed to define a portfolio.
/// Optional fields are used to filter the portfolio.
#[derive(Clone)]
pub struct Portfolio {
    client_id: Option<usize>,
    segment: Option<Segment>,
    product_family: Option<ProductFamily>,
    postion_type: Option<PositionType>,
    rate_type: Option<RateType>,
    currency: Option<Currency>,
    instruments: Vec<Instrument>,
}

impl Portfolio {
    pub fn new() -> Self {
        Portfolio {
            client_id: None,
            segment: None,
            product_family: None,
            postion_type: None,
            rate_type: None,
            currency: None,
            instruments: Vec::new(),
        }
    }

    pub fn client_id(&self) -> Option<usize> {
        self.client_id
    }

    pub fn segment(&self) -> Option<Segment> {
        self.segment
    }

    pub fn product_family(&self) -> Option<ProductFamily> {
        self.product_family
    }

    pub fn position_type(&self) -> Option<PositionType> {
        self.postion_type
    }

    pub fn rate_type(&self) -> Option<RateType> {
        self.rate_type
    }

    pub fn currency(&self) -> Option<Currency> {
        self.currency
    }

    pub fn with_currency(mut self, currency: Currency) -> Self {
        self.currency = Some(currency);
        self
    }

    pub fn with_rate_type(mut self, rate_type: RateType) -> Self {
        self.rate_type = Some(rate_type);
        self
    }

    pub fn with_client_id(mut self, client_id: usize) -> Self {
        self.client_id = Some(client_id);
        self
    }

    pub fn with_segment(mut self, segment: Segment) -> Self {
        self.segment = Some(segment);
        self
    }

    pub fn with_product_family(mut self, product_family: ProductFamily) -> Self {
        self.product_family = Some(product_family);
        self
    }

    pub fn with_position_type(mut self, position_type: PositionType) -> Self {
        self.postion_type = Some(position_type);
        self
    }

    pub fn with_instruments(mut self, instruments: Vec<Instrument>) -> Self {
        self.instruments = instruments;
        self
    }

    pub fn add_instrument(&mut self, instrument: Instrument) {
        self.instruments.push(instrument);
    }

    pub fn instruments(&self) -> &[Instrument] {
        &self.instruments
    }

    pub fn instruments_mut(&mut self) -> &mut [Instrument] {
        &mut self.instruments
    }
}

/// # PortfolioFilter
/// A struct that contains the information needed to define a portfolio filter.
/// Optional fields are used to filter the portfolio.
#[derive(Clone)]
pub struct PortfolioFilter {
    client_id: Option<usize>,
    segment: Option<Segment>,
    product_family: Option<ProductFamily>,

    currency: Option<Currency>,
    rate_type: Option<RateType>,
    structure: Option<Structure>,
    position_type: Option<PositionType>,
}

impl PortfolioFilter {
    pub fn new() -> Self {
        PortfolioFilter {
            client_id: None,
            segment: None,
            product_family: None,
            currency: None,
            rate_type: None,
            structure: None,
            position_type: None,
        }
    }

    pub fn with_client_id(mut self, client_id: usize) -> Self {
        self.client_id = Some(client_id);
        self
    }

    pub fn with_segment(mut self, segment: Segment) -> Self {
        self.segment = Some(segment);
        self
    }

    pub fn with_product_family(mut self, product_family: ProductFamily) -> Self {
        self.product_family = Some(product_family);
        self
    }

    pub fn with_currency(mut self, currency: Currency) -> Self {
        self.currency = Some(currency);
        self
    }

    pub fn with_rate_type(mut self, rate_type: RateType) -> Self {
        self.rate_type = Some(rate_type);
        self
    }

    pub fn with_structure(mut self, structure: Structure) -> Self {
        self.structure = Some(structure);
        self
    }

    pub fn with_position_type(mut self, position_type: PositionType) -> Self {
        self.position_type = Some(position_type);
        self
    }

    pub fn client_id(&self) -> Option<usize> {
        self.client_id
    }

    pub fn segment(&self) -> Option<Segment> {
        self.segment
    }

    pub fn product_family(&self) -> Option<ProductFamily> {
        self.product_family
    }

    pub fn currency(&self) -> Option<Currency> {
        self.currency
    }

    pub fn rate_type(&self) -> Option<RateType> {
        self.rate_type
    }

    pub fn structure(&self) -> Option<Structure> {
        self.structure
    }

    pub fn position_type(&self) -> Option<PositionType> {
        self.position_type
    }
}

/// # AccountType
/// A struct that contains the information needed to define an account type.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AccountType {
    Asset,
    Liability,
    Equity,
    Revenue,
    Expense,
}

impl TryFrom<String> for AccountType {
    type Error = AtlasError;

    fn try_from(s: String) -> Result<Self> {
        match s.as_str() {
            "Asset" => Ok(AccountType::Asset),
            "Liability" => Ok(AccountType::Liability),
            "Equity" => Ok(AccountType::Equity),
            "Revenue" => Ok(AccountType::Revenue),
            "Expense" => Ok(AccountType::Expense),
            _ => Err(AtlasError::InvalidValueErr(format!(
                "Invalid account type: {}",
                s
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




/// # EvaluationMode
/// A struct that contains the information needed to define
/// an evaluation mode when running simulations and building instruments.
#[derive(Deserialize, Clone, Copy)]
pub enum EvaluationMode {
    FTPRate,
    ClientRate,
}

impl TryFrom<String> for EvaluationMode {
    type Error = AtlasError;

    fn try_from(s: String) -> Result<Self> {
        match s.as_str() {
            "FTPRate" => Ok(EvaluationMode::FTPRate),
            "ClientRate" => Ok(EvaluationMode::ClientRate),
            _ => Err(AtlasError::InvalidValueErr(format!(
                "Invalid evaluation mode: {}",
                s
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

/// # Segment
/// A struct that contains the information needed to define a segment.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Segment {
    Minorista,
    Mayorista,
    Tesoreria,
}

impl TryFrom<String> for Segment {
    type Error = AtlasError;

    fn try_from(s: String) -> Result<Self> {
        match s.as_str() {
            "Minorista" => Ok(Segment::Minorista),
            "Mayorista" => Ok(Segment::Mayorista),
            "Tesoreria" => Ok(Segment::Tesoreria),
            _ => Err(AtlasError::InvalidValueErr(format!(
                "Invalid segment: {}",
                s
            ))),
        }
    }
}

impl From<Segment> for String {
    fn from(segment: Segment) -> Self {
        match segment {
            Segment::Minorista => "Minorista".to_string(),
            Segment::Mayorista => "Mayorista".to_string(),
            Segment::Tesoreria => "Tesoreria".to_string(),
        }
    }
}

/// # ProductFamily
/// A struct that contains the information needed to define a product family.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProductFamily {
    Comercial,
    Comex,
    Consumo,
    Deposito,
    Hipotecario,
    Bono,
    CAE,
    Leasing,
    Fogape,
    Corfo,
    Factoring
}

impl TryFrom<String> for ProductFamily {
    type Error = AtlasError;

    fn try_from(s: String) -> Result<Self> {
        match s.as_str() {
            "Comercial" => Ok(ProductFamily::Comercial),
            "Comex" => Ok(ProductFamily::Comex),
            "Consumo" => Ok(ProductFamily::Consumo),
            "Deposito" => Ok(ProductFamily::Deposito),
            "Hipotecario" => Ok(ProductFamily::Hipotecario),
            "Bono" => Ok(ProductFamily::Bono),
            "CAE" => Ok(ProductFamily::CAE),
            "Leasing" => Ok(ProductFamily::Leasing),
            "Fogape" => Ok(ProductFamily::Fogape),
            "Corfo" => Ok(ProductFamily::Corfo),
            "Factoring" => Ok(ProductFamily::Factoring),
            _ => Err(AtlasError::InvalidValueErr(format!(
                "Invalid product family: {}",
                s
            ))),
        }
    }
}

impl From<ProductFamily> for String {
    fn from(product_family: ProductFamily) -> Self {
        match product_family {
            ProductFamily::Comercial => "Comercial".to_string(),
            ProductFamily::Comex => "Comex".to_string(),
            ProductFamily::Consumo => "Consumo".to_string(),
            ProductFamily::Deposito => "Deposito".to_string(),
            ProductFamily::Hipotecario => "Hipotecario".to_string(),
            ProductFamily::Bono => "Bono".to_string(),
            ProductFamily::CAE => "CAE".to_string(),
            ProductFamily::Leasing => "Leasing".to_string(),
            ProductFamily::Fogape => "Fogape".to_string(),
            ProductFamily::Corfo => "Corfo".to_string(),
            ProductFamily::Factoring => "Factoring".to_string(),
        }
    }
}
