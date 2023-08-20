use super::iborindex::IborIndex;

/// # InterestRateIndex
/// Enum that defines an interest rate index.
#[derive(Debug, Clone)]
pub enum InterestRateIndex {
    IborIndex(IborIndex),
    Other,
}
