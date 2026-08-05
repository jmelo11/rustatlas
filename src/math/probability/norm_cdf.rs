use crate::ad::{dual::DualFwd, expr::FloatExt, scalar::Scalar};

/// Generic `norm_cdf` implementation - works for any type supporting the needed operations.
/// This is the single entry point used everywhere.
#[must_use]
pub fn norm_cdf<T: NormCDF>(x: T) -> T {
    x.norm_cdf()
}

/// Trait for types that can compute `norm_cdf` using Hart approximation.
pub trait NormCDF: Scalar + Clone {
    /// Computes the Hart approximation of the standard normal CDF.
    #[must_use]
    fn norm_cdf(self) -> Self;
}

/// Implementation for f64
impl NormCDF for f64 {
    fn norm_cdf(self) -> Self {
        let one = 1.0;
        let l = self.abs();
        let k = one / l.mul_add(0.231_641_9, one);
        let poly = (k * 1.330_274_429 - 1.821_255_978)
            .mul_add(k, 1.781_477_937)
            .mul_add(k, -0.356_563_782)
            .mul_add(k, 0.319_381_530)
            * k;
        let pdf = (-(l * l) * 0.5).exp() * 0.398_942_280_401_432_7;
        let w = one - pdf * poly;

        if self < 0.0 {
            one - w
        } else {
            w
        }
    }
}

/// Implementation for [`DualFwd`]
impl NormCDF for DualFwd {
    fn norm_cdf(self) -> Self {
        let one: Self = 1.0.into();
        let l = self.abs();
        let k = one / (one + l * 0.231_641_9);
        let poly =
            ((((k * 1.330_274_429 - 1.821_255_978) * k + 1.781_477_937) * k - 0.356_563_782) * k
                + 0.319_381_530)
                * k;
        let pdf = (-(l * l) * 0.5).exp() * 0.398_942_280_401_432_7;
        let w = one - pdf * poly;

        if self.value() < 0.0 {
            (one - w).into()
        } else {
            w.into()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ad::dual::DualFwd;
    use crate::ad::tape::Tape;

    #[test]
    fn dualfwd_norm_cdf_derivative_matches_pdf() {
        Tape::start_recording_fwd();

        let x = DualFwd::new(0.5);
        let y = norm_cdf(x);

        y.backward().unwrap();

        let ad = x.adjoint().unwrap().value();

        let expected =
            (-0.5_f64 * 0.5_f64 * 0.5_f64).exp() * 0.398_942_280_401_432_7;

        assert!((ad - expected).abs() < 1e-6);

        Tape::stop_recording_fwd();
    }
}
