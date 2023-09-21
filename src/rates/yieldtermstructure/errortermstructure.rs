use thiserror::Error;

#[derive(Error, Debug)]
pub enum TermStructureConstructorError {
    #[error("Dates and discount factors should have the same size.")]
    DatesAndDiscountFactorsSize,
    #[error("Dates and rates should have the same size.")]
    DatesAndRatesSize,
    #[error("Dates[0] needs to be reference_date")]
    Dates0NeedsToBeReferenceDate,
    #[error("Discount factors[0] needs to be 1.0")]
    DiscountFactors0NeedsToBeOne,
}