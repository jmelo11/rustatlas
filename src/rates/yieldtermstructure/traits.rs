use crate::rates::traits::{HasReferenceDate, YieldProvider};

pub trait YieldTermStructureTrait: YieldProvider + HasReferenceDate {}
