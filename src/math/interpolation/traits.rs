/// # Interpolation trait
pub trait Interpolate {
    fn new(x_: Vec<f64>, y_: Vec<f64>, allow_extrapolation: Option<bool>) -> Self;
    fn interpolate(&self, x: f64) -> f64;
    fn enable_extrapolation(&mut self, enable: bool);
    fn lower_bound(&self) -> f64;
    fn upper_bound(&self) -> f64;
}
