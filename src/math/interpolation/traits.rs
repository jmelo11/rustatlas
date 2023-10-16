/// # Interpolation trait
pub trait Interpolate {
    fn interpolate(x: f64, x_: &Vec<f64>, y_: &Vec<f64>, enable_extrapolation: bool) -> f64;
}
