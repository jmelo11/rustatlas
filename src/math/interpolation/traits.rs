use crate::core::meta::Number;

/// # Interpolation trait
/// A trait that defines the interpolation of a function.
pub trait Interpolate {
    fn interpolate(
        x: Number,
        x_: &Vec<Number>,
        y_: &Vec<Number>,
        enable_extrapolation: bool,
    ) -> Number;
}
