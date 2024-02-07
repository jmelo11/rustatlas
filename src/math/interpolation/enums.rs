use serde::{Deserialize, Serialize};

use super::{linear::LinearInterpolator, loglinear::LogLinearInterpolator, traits::Interpolate};

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum Interpolator {
    Linear,
    LogLinear,
}

impl Interpolator {
    pub fn interpolate(
        &self,
        x: f64,
        x_: &Vec<f64>,
        y_: &Vec<f64>,
        enable_extrapolation: bool,
    ) -> f64 {
        match self {
            Interpolator::Linear => {
                LinearInterpolator::interpolate(x, x_, y_, enable_extrapolation)
            }
            Interpolator::LogLinear => {
                LogLinearInterpolator::interpolate(x, x_, y_, enable_extrapolation)
            }
        }
    }
}
