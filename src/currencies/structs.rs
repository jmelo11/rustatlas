use super::traits::CurrencyDetails;

pub struct USD;
pub struct EUR;
pub struct JPY;
pub struct ZAR;
pub struct CLP;
pub struct CLF;

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
