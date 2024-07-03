use crate::core::meta::Numeric;

/// # Interpolation trait
/// A trait that defines the interpolation of a function.
pub trait Interpolate {
    fn interpolate(
        x: Numeric,
        x_: &Vec<Numeric>,
        y_: &Vec<Numeric>,
        enable_extrapolation: bool,
    ) -> Numeric;
}
