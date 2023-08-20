/// # YieldTermStructure
/// Enum for YieldTermStructure
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum YieldTermStructure {
    FlatForwardTermStructure,
    Other,
}
