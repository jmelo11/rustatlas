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

/// # AUD
/// Struct for AUD currency
pub struct AUD;

/// # CAD
/// Struct for CAD currency
pub struct CAD;

/// # CNY
/// Struct for CNY currency
pub struct CNY;

/// # GBP
/// Struct for GBP currency
pub struct GBP;

/// # MXN
/// Struct for MXN currency
pub struct MXN;

/// # NZD
/// Struct for NZD currency
pub struct NZD;

/// # PEN
/// Struct for PEN currency
pub struct PEN;

/// # NOK
/// Struct for NOK currency
pub struct NOK;

/// # SEK
/// Struct for SEK currency
pub struct SEK;

/// # CNH
/// Struct for CNH currency
pub struct CNH;

/// # INR
/// Struct for INR currency
pub struct INR;

/// # TWD
/// Struct for TWD currency
pub struct TWD;

/// # KRW
/// Struct for KRW currency
pub struct KRW;

/// # HKD
/// Struct for HKD currency
pub struct HKD;

/// # DKK
/// Struct for DKK currency
pub struct DKK;

/// # IDR
/// Struct for IDR currency
pub struct IDR;


impl CurrencyDetails for IDR {
    fn code(&self) -> String {
        return "IDR".to_string();
    }
    fn name(&self) -> String {
        return "Indonesian Rupiah".to_string();
    }
    fn symbol(&self) -> String {
        return "Rp".to_string();
    }
    fn precision(&self) -> u8 {
        return 2;
    }
    fn numeric_code(&self) -> u16 {
        return 360;
    }
}


impl CurrencyDetails for HKD {
    fn code(&self) -> String {
        return "HKD".to_string();
    }
    fn name(&self) -> String {
        return "Hong Kong Dollar".to_string();
    }
    fn symbol(&self) -> String {
        return "HK$".to_string();
    }
    fn precision(&self) -> u8 {
        return 2;
    }
    fn numeric_code(&self) -> u16 {
        return 344;
    }
}

impl CurrencyDetails for KRW {
    fn code(&self) -> String {
        return "KRW".to_string();
    }
    fn name(&self) -> String {
        return "South Korean Won".to_string();
    }
    fn symbol(&self) -> String {
        return "₩".to_string();
    }
    fn precision(&self) -> u8 {
        return 0;
    }
    fn numeric_code(&self) -> u16 {
        return 410;
    }
}

impl CurrencyDetails for TWD {
    fn code(&self) -> String {
        return "TWD".to_string();
    }
    fn name(&self) -> String {
        return "New Taiwan Dollar".to_string();
    }
    fn symbol(&self) -> String {
        return "NT$".to_string();
    }
    fn precision(&self) -> u8 {
        return 2;
    }
    fn numeric_code(&self) -> u16 {
        return 901;
    }
}

impl CurrencyDetails for INR {
    fn code(&self) -> String {
        return "INR".to_string();
    }
    fn name(&self) -> String {
        return "Indian Rupee".to_string();
    }
    fn symbol(&self) -> String {
        return "₹".to_string();
    }
    fn precision(&self) -> u8 {
        return 2;
    }
    fn numeric_code(&self) -> u16 {
        return 356;
    }
}

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

impl CurrencyDetails for BRL {
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

impl CurrencyDetails for AUD {
    fn code(&self) -> String {
        return "AUD".to_string();
    }
    fn name(&self) -> String {
        return "Australian Dollar".to_string();
    }
    fn symbol(&self) -> String {
        return "A$".to_string();
    }
    fn precision(&self) -> u8 {
        return 2;
    }
    fn numeric_code(&self) -> u16 {
        return 36;
    }
}

impl CurrencyDetails for NZD {
    fn code(&self) -> String {
        return "NZD".to_string();
    }
    fn name(&self) -> String {
        return "New Zealand Dollar".to_string();
    }
    fn symbol(&self) -> String {
        return "NZ$".to_string();
    }
    fn precision(&self) -> u8 {
        return 2;
    }
    fn numeric_code(&self) -> u16 {
        return 554;
    }
}

impl CurrencyDetails for CAD {
    fn code(&self) -> String {
        return "CAD".to_string();
    }
    fn name(&self) -> String {
        return "Canadian Dollar".to_string();
    }
    fn symbol(&self) -> String {
        return "Can$".to_string();
    }
    fn precision(&self) -> u8 {
        return 2;
    }
    fn numeric_code(&self) -> u16 {
        return 124;
    }
}

impl CurrencyDetails for MXN {
    fn code(&self) -> String {
        return "MXN".to_string();
    }
    fn name(&self) -> String {
        return "Mexican Peso".to_string();
    }
    fn symbol(&self) -> String {
        return "Mex$".to_string();
    }
    fn precision(&self) -> u8 {
        return 2;
    }
    fn numeric_code(&self) -> u16 {
        return 484;
    }
}

impl CurrencyDetails for PEN {
    fn code(&self) -> String {
        return "PEN".to_string();
    }
    fn name(&self) -> String {
        return "Peruvian Sol".to_string();
    }
    fn symbol(&self) -> String {
        return "S/.".to_string();
    }
    fn precision(&self) -> u8 {
        return 2;
    }
    fn numeric_code(&self) -> u16 {
        return 604;
    }
}

impl CurrencyDetails for GBP {
    fn code(&self) -> String {
        return "GBP".to_string();
    }
    fn name(&self) -> String {
        return "British Pound".to_string();
    }
    fn symbol(&self) -> String {
        return "£".to_string();
    }
    fn precision(&self) -> u8 {
        return 2;
    }
    fn numeric_code(&self) -> u16 {
        return 826;
    }
}

impl CurrencyDetails for CNY {
    fn code(&self) -> String {
        return "CNY".to_string();
    }
    fn name(&self) -> String {
        return "Chinese Yuan".to_string();
    }
    fn symbol(&self) -> String {
        return "¥".to_string();
    }
    fn precision(&self) -> u8 {
        return 2;
    }
    fn numeric_code(&self) -> u16 {
        return 156;
    }
}

impl CurrencyDetails for NOK {
    fn code(&self) -> String {
        return "NOK".to_string();
    }
    fn name(&self) -> String {
        return "Norwegian Krone".to_string();
    }
    fn symbol(&self) -> String {
        return "kr".to_string();
    }
    fn precision(&self) -> u8 {
        return 2;
    }
    fn numeric_code(&self) -> u16 {
        return 578;
    }
}

impl CurrencyDetails for SEK {
    fn code(&self) -> String {
        return "SEK".to_string();
    }
    fn name(&self) -> String {
        return "Swedish Krona".to_string();
    }
    fn symbol(&self) -> String {
        return "kr".to_string();
    }
    fn precision(&self) -> u8 {
        return 2;
    }
    fn numeric_code(&self) -> u16 {
        return 752;
    }
}

impl CurrencyDetails for CNH {
    fn code(&self) -> String {
        return "CNH".to_string();
    }
    fn name(&self) -> String {
        return "Chinese Yuan (offshore)".to_string();
    }
    fn symbol(&self) -> String {
        return "¥".to_string();
    }
    fn precision(&self) -> u8 {
        return 2;
    }
    fn numeric_code(&self) -> u16 {
        return 156;
    }
}

impl CurrencyDetails for DKK {
    fn code(&self) -> String {
        return "DKK".to_string();
    }
    fn name(&self) -> String {
        return "Danish Krone".to_string();
    }
    fn symbol(&self) -> String {
        return "kr".to_string();
    }
    fn precision(&self) -> u8 {
        return 2;
    }
    fn numeric_code(&self) -> u16 {
        return 208;
    }
}

