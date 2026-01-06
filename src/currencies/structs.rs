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
        "IDR".to_string()
    }
    fn name(&self) -> String {
        "Indonesian Rupiah".to_string()
    }
    fn symbol(&self) -> String {
        "Rp".to_string()
    }
    fn precision(&self) -> u8 {
        2
    }
    fn numeric_code(&self) -> u16 {
        360
    }
}

impl CurrencyDetails for HKD {
    fn code(&self) -> String {
        "HKD".to_string()
    }
    fn name(&self) -> String {
        "Hong Kong Dollar".to_string()
    }
    fn symbol(&self) -> String {
        "HK$".to_string()
    }
    fn precision(&self) -> u8 {
        2
    }
    fn numeric_code(&self) -> u16 {
        344
    }
}

impl CurrencyDetails for KRW {
    fn code(&self) -> String {
        "KRW".to_string()
    }
    fn name(&self) -> String {
        "South Korean Won".to_string()
    }
    fn symbol(&self) -> String {
        "₩".to_string()
    }
    fn precision(&self) -> u8 {
        0
    }
    fn numeric_code(&self) -> u16 {
        410
    }
}

impl CurrencyDetails for TWD {
    fn code(&self) -> String {
        "TWD".to_string()
    }
    fn name(&self) -> String {
        "New Taiwan Dollar".to_string()
    }
    fn symbol(&self) -> String {
        "NT$".to_string()
    }
    fn precision(&self) -> u8 {
        2
    }
    fn numeric_code(&self) -> u16 {
        901
    }
}

impl CurrencyDetails for INR {
    fn code(&self) -> String {
        "INR".to_string()
    }
    fn name(&self) -> String {
        "Indian Rupee".to_string()
    }
    fn symbol(&self) -> String {
        "₹".to_string()
    }
    fn precision(&self) -> u8 {
        2
    }
    fn numeric_code(&self) -> u16 {
        356
    }
}

impl CurrencyDetails for USD {
    fn code(&self) -> String {
        "USD".to_string()
    }
    fn name(&self) -> String {
        "US Dollar".to_string()
    }
    fn symbol(&self) -> String {
        "$".to_string()
    }
    fn precision(&self) -> u8 {
        2
    }
    fn numeric_code(&self) -> u16 {
        840
    }
}

impl CurrencyDetails for EUR {
    fn code(&self) -> String {
        "EUR".to_string()
    }
    fn name(&self) -> String {
        "Euro".to_string()
    }
    fn symbol(&self) -> String {
        "€".to_string()
    }
    fn precision(&self) -> u8 {
        2
    }
    fn numeric_code(&self) -> u16 {
        978
    }
}

impl CurrencyDetails for JPY {
    fn code(&self) -> String {
        "JPY".to_string()
    }
    fn name(&self) -> String {
        "Japanese Yen".to_string()
    }
    fn symbol(&self) -> String {
        "¥".to_string()
    }
    fn precision(&self) -> u8 {
        0
    }
    fn numeric_code(&self) -> u16 {
        392
    }
}

impl CurrencyDetails for ZAR {
    fn code(&self) -> String {
        "ZAR".to_string()
    }
    fn name(&self) -> String {
        "South African Rand".to_string()
    }
    fn symbol(&self) -> String {
        "R".to_string()
    }
    fn precision(&self) -> u8 {
        2
    }
    fn numeric_code(&self) -> u16 {
        710
    }
}

impl CurrencyDetails for CLP {
    fn code(&self) -> String {
        "CLP".to_string()
    }
    fn name(&self) -> String {
        "Chilean Peso".to_string()
    }
    fn symbol(&self) -> String {
        "$".to_string()
    }
    fn precision(&self) -> u8 {
        0
    }
    fn numeric_code(&self) -> u16 {
        152
    }
}

impl CurrencyDetails for CLF {
    fn code(&self) -> String {
        "CLF".to_string()
    }
    fn name(&self) -> String {
        "Chilean Unidad de Fomento".to_string()
    }
    fn symbol(&self) -> String {
        "UF".to_string()
    }
    fn precision(&self) -> u8 {
        4
    }
    fn numeric_code(&self) -> u16 {
        990
    }
}

impl CurrencyDetails for CHF {
    fn code(&self) -> String {
        "CHF".to_string()
    }
    fn name(&self) -> String {
        "Swiss Franc".to_string()
    }
    fn symbol(&self) -> String {
        "Fr".to_string()
    }
    fn precision(&self) -> u8 {
        2
    }
    fn numeric_code(&self) -> u16 {
        756
    }
}

impl CurrencyDetails for BRL {
    fn code(&self) -> String {
        "BRL".to_string()
    }
    fn name(&self) -> String {
        "Brazilian Real".to_string()
    }
    fn symbol(&self) -> String {
        "R$".to_string()
    }
    fn precision(&self) -> u8 {
        2
    }
    fn numeric_code(&self) -> u16 {
        986
    }
}

impl CurrencyDetails for COP {
    fn code(&self) -> String {
        "COP".to_string()
    }
    fn name(&self) -> String {
        "Colombian Peso".to_string()
    }
    fn symbol(&self) -> String {
        "$".to_string()
    }
    fn precision(&self) -> u8 {
        2
    }
    fn numeric_code(&self) -> u16 {
        170
    }
}

impl CurrencyDetails for AUD {
    fn code(&self) -> String {
        "AUD".to_string()
    }
    fn name(&self) -> String {
        "Australian Dollar".to_string()
    }
    fn symbol(&self) -> String {
        "A$".to_string()
    }
    fn precision(&self) -> u8 {
        2
    }
    fn numeric_code(&self) -> u16 {
        36
    }
}

impl CurrencyDetails for NZD {
    fn code(&self) -> String {
        "NZD".to_string()
    }
    fn name(&self) -> String {
        "New Zealand Dollar".to_string()
    }
    fn symbol(&self) -> String {
        "NZ$".to_string()
    }
    fn precision(&self) -> u8 {
        2
    }
    fn numeric_code(&self) -> u16 {
        554
    }
}

impl CurrencyDetails for CAD {
    fn code(&self) -> String {
        "CAD".to_string()
    }
    fn name(&self) -> String {
        "Canadian Dollar".to_string()
    }
    fn symbol(&self) -> String {
        "Can$".to_string()
    }
    fn precision(&self) -> u8 {
        2
    }
    fn numeric_code(&self) -> u16 {
        124
    }
}

impl CurrencyDetails for MXN {
    fn code(&self) -> String {
        "MXN".to_string()
    }
    fn name(&self) -> String {
        "Mexican Peso".to_string()
    }
    fn symbol(&self) -> String {
        "Mex$".to_string()
    }
    fn precision(&self) -> u8 {
        2
    }
    fn numeric_code(&self) -> u16 {
        484
    }
}

impl CurrencyDetails for PEN {
    fn code(&self) -> String {
        "PEN".to_string()
    }
    fn name(&self) -> String {
        "Peruvian Sol".to_string()
    }
    fn symbol(&self) -> String {
        "S/.".to_string()
    }
    fn precision(&self) -> u8 {
        2
    }
    fn numeric_code(&self) -> u16 {
        604
    }
}

impl CurrencyDetails for GBP {
    fn code(&self) -> String {
        "GBP".to_string()
    }
    fn name(&self) -> String {
        "British Pound".to_string()
    }
    fn symbol(&self) -> String {
        "£".to_string()
    }
    fn precision(&self) -> u8 {
        2
    }
    fn numeric_code(&self) -> u16 {
        826
    }
}

impl CurrencyDetails for CNY {
    fn code(&self) -> String {
        "CNY".to_string()
    }
    fn name(&self) -> String {
        "Chinese Yuan".to_string()
    }
    fn symbol(&self) -> String {
        "¥".to_string()
    }
    fn precision(&self) -> u8 {
        2
    }
    fn numeric_code(&self) -> u16 {
        156
    }
}

impl CurrencyDetails for NOK {
    fn code(&self) -> String {
        "NOK".to_string()
    }
    fn name(&self) -> String {
        "Norwegian Krone".to_string()
    }
    fn symbol(&self) -> String {
        "kr".to_string()
    }
    fn precision(&self) -> u8 {
        2
    }
    fn numeric_code(&self) -> u16 {
        578
    }
}

impl CurrencyDetails for SEK {
    fn code(&self) -> String {
        "SEK".to_string()
    }
    fn name(&self) -> String {
        "Swedish Krona".to_string()
    }
    fn symbol(&self) -> String {
        "kr".to_string()
    }
    fn precision(&self) -> u8 {
        2
    }
    fn numeric_code(&self) -> u16 {
        752
    }
}

impl CurrencyDetails for CNH {
    fn code(&self) -> String {
        "CNH".to_string()
    }
    fn name(&self) -> String {
        "Chinese Yuan (offshore)".to_string()
    }
    fn symbol(&self) -> String {
        "¥".to_string()
    }
    fn precision(&self) -> u8 {
        2
    }
    fn numeric_code(&self) -> u16 {
        156
    }
}

impl CurrencyDetails for DKK {
    fn code(&self) -> String {
        "DKK".to_string()
    }
    fn name(&self) -> String {
        "Danish Krone".to_string()
    }
    fn symbol(&self) -> String {
        "kr".to_string()
    }
    fn precision(&self) -> u8 {
        2
    }
    fn numeric_code(&self) -> u16 {
        208
    }
}
