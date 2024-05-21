use std::cmp::Ordering;

use super::traits::Interpolate;

/// # Log-Linear Interpolator
/// Log-linear interpolator.
#[derive(Clone)]
pub struct LogLinearInterpolator {}

impl Interpolate for LogLinearInterpolator {
    fn interpolate(x: f64, x_: &Vec<f64>, y_: &Vec<f64>, enable_extrapolation: bool) -> f64 {
        let index =
            match x_.binary_search_by(|&probe| probe.partial_cmp(&x).unwrap_or(Ordering::Less)) {
                Ok(index) => index,
                Err(index) => index,
            };

        if !enable_extrapolation && (x < *x_.first().unwrap() || x > *x_.last().unwrap()) {
            panic!("Extrapolation is not enabled, and the provided value is outside the range.");
        }

        match index {
            0 => y_[0] * (y_[1] / y_[0]).powf((x - x_[0]) / (x_[1] - x_[0])),
            idx if idx == x_.len() => {
                y_[idx - 1]
                    * (y_[idx - 1] / y_[idx - 2])
                        .powf((x - x_[idx - 1]) / (x_[idx - 1] - x_[idx - 2]))
            }
            _ => {
                y_[index - 1]
                    * (y_[index] / y_[index - 1])
                        .powf((x - x_[index - 1]) / (x_[index] - x_[index - 1]))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loglinear_interpolation() {
        let x = 0.5;
        let x_ = vec![0.0, 1.0];
        let y_ = vec![0.1, 1.0]; // Change from 0.0 to 0.1
        let y = LogLinearInterpolator::interpolate(x, &x_, &y_, true);
        // Adjust the expected value accordingly
        assert!((y - 0.31622776601683794).abs() < 1e-10);
    }
}
