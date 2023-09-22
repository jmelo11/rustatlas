use thiserror::Error;

#[derive(Error, Debug)]
pub enum TermStructureConstructorError {
    #[error("Dates and discount factors should have the same size.")]
    DatesAndDiscountFactorsSize,
    #[error("Dates and rates should have the same size.")]
    DatesAndRatesSize,
    #[error("First date needs to be reference_date")]
    FirstDateNeedsToBeReferenceDate,
    #[error("First discount factor needs to be 1.0")]
    FirstDiscountFactorsNeedsToBeOne,
}
