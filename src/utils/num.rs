use std::ops::{Add, Sub, Mul, Div, Neg};

/// Trait implemented by numeric types used in pricing calculations.
pub trait Real:
    Copy + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self> + Neg<Output = Self> + From<f64>
{
}

impl<T> Real for T where
    T: Copy
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Neg<Output = T>
        + From<f64>
{
}
