use serde::{Deserialize, Serialize};

use super::{linear::LinearInterpolator, loglinear::LogLinearInterpolator, traits::Interpolate};

/// # Interpolator
/// Enum that represents the type of interpolation.
///
/// ## Example
/// ```
/// use rustatlas::prelude::*;
/// let x = 1.0;
/// let x_ = vec![0.0, 1.0, 2.0];
/// let y_ = vec![0.0, 1.0, 4.0];
/// let interpolator = Interpolator::Linear;
/// let y = interpolator.interpolate(x, &x_, &y_, true);
/// assert_eq!(y, 1.0);
/// ```
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
