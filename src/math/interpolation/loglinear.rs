use std::cmp::Ordering;

use super::traits::Interpolate;

#[derive(Clone)]
pub struct LogLinearInterpolator {
    x: Vec<f64>,
    y: Vec<f64>,
    enable_extrapolation: bool,
}

impl Interpolate for LogLinearInterpolator {

    fn new(x_: Vec<f64>, y_: Vec<f64>, allow_extrapolation: Option<bool>) -> LogLinearInterpolator {
        let extrapolation = allow_extrapolation.unwrap_or(false);
        LogLinearInterpolator {
            x: x_,
            y: y_,
            enable_extrapolation: extrapolation,
        }
    }

    fn interpolate(&self, x: f64) -> f64 {
        let index = match self
            .x
            .binary_search_by(|probe| probe.partial_cmp(&x).unwrap_or(Ordering::Less))
        {
            Ok(index) => index,
            Err(index) => index,
        };

        if !self.enable_extrapolation
            && (x < *self.x.first().unwrap() || x > *self.x.last().unwrap())
        {
            panic!("Extrapolation is not enabled, and the provided value is outside the range.");
        }

        match index {
            0 => {
                self.y[0]
                    * ((x - self.x[0]) * (self.y[1].ln() - self.y[0].ln())
                        / (self.x[1] - self.x[0]))
                        .exp()
            }
            idx if idx == self.x.len() => {
                self.y[idx - 1]
                    * ((x - self.x[idx - 1]) * (self.y[idx - 1].ln() - self.y[idx - 2].ln())
                        / (self.x[idx - 1] - self.x[idx - 2]))
                        .exp()
            }
            _ => {
                self.y[index - 1]
                    * ((x - self.x[index - 1]) * (self.y[index].ln() - self.y[index - 1].ln())
                        / (self.x[index] - self.x[index - 1]))
                        .exp()
            }
        }
    }

    fn enable_extrapolation(&mut self, enable: bool) {
        self.enable_extrapolation = enable;
    }

    fn lower_bound(&self) -> f64 {
        *self.x.first().unwrap()
    }

    fn upper_bound(&self) -> f64 {
        *self.x.last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize() {
        let interpolator = LogLinearInterpolator::new(
            vec![1.0, 2.0, 3.0],
            vec![2.0, 4.0, 8.0],
            Some(false),
        );
        assert_eq!(interpolator.x, vec![1.0, 2.0, 3.0]);
        assert_eq!(interpolator.y, vec![2.0, 4.0, 8.0]);
        assert_eq!(interpolator.enable_extrapolation, false);
    }

    #[test]
    fn test_interpolation() {
        let interpolator =
            LogLinearInterpolator::new(vec![1.0, 2.0, 3.0], vec![2.0, 4.0, 8.0], Some(true));
        assert!((interpolator.interpolate(1.5) - 2.8284).abs() < 1e-4);
        assert!((interpolator.interpolate(2.5) - 5.6568).abs() < 1e-4);
    }

    #[test]
    #[should_panic(
        expected = "Extrapolation is not enabled, and the provided value is outside the range."
    )]
    fn test_interpolation_no_extrapolation() {
        let interpolator = LogLinearInterpolator::new(
            vec![1.0, 2.0, 3.0],
            vec![2.0, 4.0, 8.0],
            Some(false),
        );
        interpolator.interpolate(0.5);
    }

    #[test]
    fn test_bounds() {
        let interpolator =
            LogLinearInterpolator::new(vec![1.0, 2.0, 3.0], vec![2.0, 4.0, 8.0], Some(true));
        assert_eq!(interpolator.lower_bound(), 1.0);
        assert_eq!(interpolator.upper_bound(), 3.0);
    }


    // prueba
}
