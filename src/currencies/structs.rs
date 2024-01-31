use super::traits::CurrencyDetails;

/// # USD
/// Struct for USD currency
pub struct USD;

/// # EUR
/// Struct for EUR currency
pub struct EUR;

/// # JPY
/// Struct for JPY currency
pub struct JPY;

/// # ZAR
/// Struct for ZAR currency
pub struct ZAR;

/// # CLP
/// Struct for CLP currency
pub struct CLP;

/// # CLF
/// Struct for CLF currency
pub struct CLF;

/// # CHF
/// Struct for CHF currency
pub struct CHF;

/// # BRL
/// Struct for BRL currency
pub struct BRL;

/// # COP
/// Struct for COP currency
pub struct COP;


impl CurrencyDetails for USD {
    fn code(&self) -> String {
        return "USD".to_string();
    }
    fn name(&self) -> String {
        return "US Dollar".to_string();
    }
    fn symbol(&self) -> String {
        return "$".to_string();
    }
    fn precision(&self) -> u8 {
        return 2;
    }
    fn numeric_code(&self) -> u16 {
        return 840;
    }
}

impl CurrencyDetails for EUR {
    fn code(&self) -> String {
        return "EUR".to_string();
    }
    fn name(&self) -> String {
        return "Euro".to_string();
    }
    fn symbol(&self) -> String {
        return "€".to_string();
    }
    fn precision(&self) -> u8 {
        return 2;
    }
    fn numeric_code(&self) -> u16 {
        return 978;
    }
}

impl CurrencyDetails for JPY {
    fn code(&self) -> String {
        return "JPY".to_string();
    }
    fn name(&self) -> String {
        return "Japanese Yen".to_string();
    }
    fn symbol(&self) -> String {
        return "¥".to_string();
    }
    fn precision(&self) -> u8 {
        return 0;
    }
    fn numeric_code(&self) -> u16 {
        return 392;
    }
}

impl CurrencyDetails for ZAR {
    fn code(&self) -> String {
        return "ZAR".to_string();
    }
    fn name(&self) -> String {
        return "South African Rand".to_string();
    }
    fn symbol(&self) -> String {
        return "R".to_string();
    }
    fn precision(&self) -> u8 {
        return 2;
    }
    fn numeric_code(&self) -> u16 {
        return 710;
    }
}

impl CurrencyDetails for CLP {
    fn code(&self) -> String {
        return "CLP".to_string();
    }
    fn name(&self) -> String {
        return "Chilean Peso".to_string();
    }
    fn symbol(&self) -> String {
        return "$".to_string();
    }
    fn precision(&self) -> u8 {
        return 0;
    }
    fn numeric_code(&self) -> u16 {
        return 152;
    }
}

impl CurrencyDetails for CLF {
    fn code(&self) -> String {
        return "CLF".to_string();
    }
    fn name(&self) -> String {
        return "Chilean Unidad de Fomento".to_string();
    }
    fn symbol(&self) -> String {
        return "UF".to_string();
    }
    fn precision(&self) -> u8 {
        return 4;
    }
    fn numeric_code(&self) -> u16 {
        return 990;
    }
}

impl CurrencyDetails for CHF {
    fn code(&self) -> String {
        return "CHF".to_string();
    }
    fn name(&self) -> String {
        return "Swiss Franc".to_string();
    }
    fn symbol(&self) -> String {
        return "Fr".to_string();
    }
    fn precision(&self) -> u8 {
        return 2;
    }
    fn numeric_code(&self) -> u16 {
        return 756;
    }
}


impl  CurrencyDetails for BRL {
    fn code(&self) -> String {
        return "BRL".to_string();
    }
    fn name(&self) -> String {
        return "Brazilian Real".to_string();
    }
    fn symbol(&self) -> String {
        return "R$".to_string();
    }
    fn precision(&self) -> u8 {
        return 2;
    }
    fn numeric_code(&self) -> u16 {
        return 986;
    }
}

impl CurrencyDetails for COP {
    fn code(&self) -> String {
        return "COP".to_string();
    }
    fn name(&self) -> String {
        return "Colombian Peso".to_string();
    }
    fn symbol(&self) -> String {
        return "$".to_string();
    }
    fn precision(&self) -> u8 {
        return 2;
    }
    fn numeric_code(&self) -> u16 {
        return 170;
    }
}