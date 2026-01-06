/// # Interpolation trait
/// A trait that defines the interpolation of a function.
pub trait Interpolate {
    fn interpolate(x: f64, x_: &[f64], y_: &[f64], enable_extrapolation: bool) -> f64;
}
