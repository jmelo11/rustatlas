pub trait CurrencyDetails {
    fn code(&self) -> String;
    fn name(&self) -> String;
    fn symbol(&self) -> String;
    fn precision(&self) -> u8;
    fn numeric_code(&self) -> u16;
}
