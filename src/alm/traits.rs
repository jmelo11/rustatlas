use crate::time::period::Period;

/// # AdvanceInTime
/// Trait for advancing in time a given object. Returns a represation of the object
/// as it would be after the given period.
pub trait AdvanceInTime {
    type Output;
    fn advance(&self, period: Period) -> Self::Output;
}
