use thiserror::Error;

use crate::{
    rates::{
        traits::{HasReferenceDate, YieldProvider, YieldProviderError},
        yieldtermstructure::{
            discounttermstructure::DiscountTermStructure,
            errortermstructure::TermStructureConstructorError,
            flatforwardtermstructure::FlatForwardTermStructure,
            spreadtermstructure::SpreadedTermStructure, traits::YieldTermStructureTrait,
            zeroratetermstructure::ZeroRateTermStructure,
        },
    },
    time::{date::Date, enums::TimeUnit, period::Period},
};

/// # AdvanceTermStructureInTime
/// Trait for advancing in time a given object. Returns a represation of the object
/// as it would be after the given period.
pub trait AdvanceTermStructureInTime {
    fn advance_to_period(
        &self,
        period: Period,
    ) -> Result<Box<dyn YieldTermStructureTrait>, AdvanceInTimeError>;
    fn advance_to_date(
        &self,
        date: Date,
    ) -> Result<Box<dyn YieldTermStructureTrait>, AdvanceInTimeError>;
}

#[derive(Error, Debug)]
pub enum AdvanceInTimeError {
    #[error("Invalid date")]
    InvalidDate,
    #[error("YieldProviderError: {0}")]
    YieldProviderError(#[from] YieldProviderError),
    #[error("TermStructureConstructorError: {0}")]
    TermStructureConstructorError(#[from] TermStructureConstructorError),
}

/// # AdvanceTermStructureInTime for FlatForwardTermStructure
impl AdvanceTermStructureInTime for FlatForwardTermStructure {
    fn advance_to_period(
        &self,
        period: Period,
    ) -> Result<Box<dyn YieldTermStructureTrait>, AdvanceInTimeError> {
        let new_reference_date = self
            .reference_date()
            .advance(period.length(), period.units());
        return Ok(Box::new(FlatForwardTermStructure::new(
            new_reference_date,
            self.rate(),
        )));
    }

    fn advance_to_date(
        &self,
        date: Date,
    ) -> Result<Box<dyn YieldTermStructureTrait>, AdvanceInTimeError> {
        return Ok(Box::new(FlatForwardTermStructure::new(date, self.rate())));
    }
}

/// # AdvanceTermStructureInTime for DiscountTermStructure
impl AdvanceTermStructureInTime for DiscountTermStructure {
    fn advance_to_period(
        &self,
        period: Period,
    ) -> Result<Box<dyn YieldTermStructureTrait>, AdvanceInTimeError> {
        let new_reference_date = self
            .reference_date()
            .advance(period.length(), period.units());

        let new_dates: Vec<Date> = self
            .dates()
            .iter()
            .map(|x| x.advance(period.length(), period.units()))
            .collect();

        let start_df = self.discount_factor(new_dates[0])?;
        let shifted_dfs: Result<Vec<f64>, AdvanceInTimeError> = new_dates
            .iter()
            .map(|x| {
                let df = self.discount_factor(*x)?;
                Ok(df / start_df)
            })
            .collect();

        Ok(Box::new(DiscountTermStructure::new(
            new_reference_date,
            new_dates,
            shifted_dfs?,
            self.day_counter(),
            self.interpolator(),
            self.enable_extrapolation(),
        )?))
    }

    fn advance_to_date(
        &self,
        date: Date,
    ) -> Result<Box<dyn YieldTermStructureTrait>, AdvanceInTimeError> {
        let days = (date - self.reference_date()) as i32;
        if days < 0 {
            return Err(AdvanceInTimeError::InvalidDate);
        }
        let period = Period::new(days, TimeUnit::Days);
        return self.advance_to_period(period);
    }
}

/// # AdvanceTermStructureInTime for ZeroRateTermStructure
impl AdvanceTermStructureInTime for ZeroRateTermStructure {
    fn advance_to_period(
        &self,
        period: Period,
    ) -> Result<Box<dyn YieldTermStructureTrait>, AdvanceInTimeError> {
        let new_reference_date = self
            .reference_date()
            .advance(period.length(), period.units());

        let new_dates: Vec<Date> = self
            .dates()
            .iter()
            .map(|x| x.advance(period.length(), period.units()))
            .collect();

        let start_df = self.discount_factor(new_dates[0])?;
        let shifted_dfs: Result<Vec<f64>, AdvanceInTimeError> = new_dates
            .iter()
            .map(|x| {
                let df = self.discount_factor(*x)?;
                Ok(df / start_df)
            })
            .collect();

        Ok(Box::new(ZeroRateTermStructure::new(
            new_reference_date,
            new_dates,
            shifted_dfs?,
            self.rate_definition(),
            self.interpolator(),
            self.enable_extrapolation(),
        )?))
    }

    fn advance_to_date(
        &self,
        date: Date,
    ) -> Result<Box<dyn YieldTermStructureTrait>, AdvanceInTimeError> {
        let days = (date - self.reference_date()) as i32;
        if days < 0 {
            return Err(AdvanceInTimeError::InvalidDate);
        }
        let period = Period::new(days, TimeUnit::Days);
        return self.advance_to_period(period);
    }
}

/// # AdvanceTermStructureInTime for SpreadedTermStructure
impl AdvanceTermStructureInTime for SpreadedTermStructure {
    fn advance_to_date(
        &self,
        date: Date,
    ) -> Result<Box<dyn YieldTermStructureTrait>, AdvanceInTimeError> {
        let base = self.base_curve().advance_to_date(date)?;
        let spread = self.spread_curve().advance_to_date(date)?;
        Ok(Box::new(SpreadedTermStructure::new(spread, base)))
    }

    fn advance_to_period(
        &self,
        period: Period,
    ) -> Result<Box<dyn YieldTermStructureTrait>, AdvanceInTimeError> {
        let base = self.base_curve().advance_to_period(period)?;
        let spread = self.spread_curve().advance_to_period(period)?;
        Ok(Box::new(SpreadedTermStructure::new(spread, base)))
    }
}
