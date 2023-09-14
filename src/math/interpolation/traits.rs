pub trait Interpolate {
    type Output;

    fn initialize(x_: Vec<f64>, y_: Vec<f64>, allow_extrapolation: Option<bool>) -> Self::Output;
    fn interpolate(&self, x: f64) -> f64;
    fn enable_extrapolation(&mut self, enable: bool);
    fn lower_bound(&self) -> f64;
    fn upper_bound(&self) -> f64;
}
