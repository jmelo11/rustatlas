use std::ops::{Add, Div, Mul, Neg, Sub};

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

/// Floating point specific operations needed for differentiation
pub trait FloatOps: Real {
    fn exp(self) -> Self;
    fn ln(self) -> Self;
    fn powf(self, n: f64) -> Self;
}

impl FloatOps for f64 {
    fn exp(self) -> Self {
        f64::exp(self)
    }

    fn ln(self) -> Self {
        f64::ln(self)
    }

    fn powf(self, n: f64) -> Self {
        f64::powf(self, n)
    }
}
