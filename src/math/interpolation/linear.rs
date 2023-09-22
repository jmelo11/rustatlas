use std::cmp::Ordering;

use super::traits::Interpolate;

#[derive(Clone)]
pub struct LinearInterpolator {
    x: Vec<f64>,
    y: Vec<f64>,
    enable_extrapolation: bool,
}

impl Interpolate for LinearInterpolator {
    fn new(x: Vec<f64>, y: Vec<f64>, allow_extrapolation: Option<bool>) -> LinearInterpolator {
        let enable_extrapolation = allow_extrapolation.unwrap_or(false);
        if x.len() != y.len() {
            panic!("x and y should have the same size.");
        }
        LinearInterpolator {
            x,
            y,
            enable_extrapolation,
        }
    }

    fn interpolate(&self, x: f64) -> f64 {
        let index = match self
            .x
            .binary_search_by(|&probe| probe.partial_cmp(&x).unwrap_or(Ordering::Equal))
        {
            Ok(index) => index,
            Err(index) => index,
        };

        if !self.enable_extrapolation {
            if x < *self.x.first().unwrap() || x > *self.x.last().unwrap() {
                panic!(
                    "Extrapolation is not enabled, and the provided value is outside the range."
                );
            }
        }

        match index {
            0 => self.y[0] + (x - self.x[0]) * (self.y[1] - self.y[0]) / (self.x[1] - self.x[0]),
            index if index == self.x.len() => {
                self.y[index - 1]
                    + (x - self.x[index - 1]) * (self.y[index - 1] - self.y[index - 2])
                        / (self.x[index - 1] - self.x[index - 2])
            }
            _ => {
                self.y[index - 1]
                    + (x - self.x[index - 1]) * (self.y[index] - self.y[index - 1])
                        / (self.x[index] - self.x[index - 1])
            }
        }
    }

    fn enable_extrapolation(&mut self, enable: bool) {
        self.enable_extrapolation = enable;
    }

    fn lower_bound(&self) -> f64 {
        self.x.first().copied().unwrap_or(f64::NAN)
    }

    fn upper_bound(&self) -> f64 {
        self.x.last().copied().unwrap_or(f64::NAN)
    }
}

#[cfg(test)]
mod tests {
    use super::Interpolate;
    use super::LinearInterpolator;

    #[test]
    fn test_initialize() {
        let interpolator =
            LinearInterpolator::new(vec![0.0, 1.0, 2.0], vec![0.0, 1.0, 4.0], Some(true));
        assert_eq!(interpolator.x, vec![0.0, 1.0, 2.0]);
        assert_eq!(interpolator.y, vec![0.0, 1.0, 4.0]);
        assert_eq!(interpolator.enable_extrapolation, true);
    }

    #[test]
    fn test_interpolation() {
        let interpolator =
            LinearInterpolator::new(vec![0.0, 1.0, 2.0], vec![0.0, 1.0, 4.0], Some(true));
        assert_eq!(interpolator.interpolate(0.0), 0.0);
        assert_eq!(interpolator.interpolate(1.0), 1.0);
        assert_eq!(interpolator.interpolate(2.0), 4.0);
        assert!((interpolator.interpolate(0.5) - 0.5).abs() < 1e-8);
        assert!((interpolator.interpolate(1.5) - 2.5).abs() < 1e-8);
    }

    #[test]
    #[should_panic(
        expected = "Extrapolation is not enabled, and the provided value is outside the range."
    )]
    fn test_interpolation_no_extrapolation() {
        let interpolator =
            LinearInterpolator::new(vec![0.0, 1.0, 2.0], vec![0.0, 1.0, 4.0], Some(false));
        interpolator.interpolate(3.0);
    }
}
