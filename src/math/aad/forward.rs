use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy)]
pub struct ADNum {
    v: f64,
    dv: f64,
}

impl ADNum {
    pub fn new(v: f64, dv: f64) -> ADNum {
        ADNum { v, dv }
    }

    pub fn value(&self) -> f64 {
        self.v
    }

    pub fn derivative(&self) -> f64 {
        self.dv
    }
}

impl Add for ADNum {
    type Output = ADNum;

    fn add(self, other: ADNum) -> ADNum {
        let result = self.v + other.v;
        let lhs_der = 1.0;
        let rhs_der = 1.0;
        ADNum {
            v: result,
            dv: lhs_der * self.dv + rhs_der * other.dv,
        }
    }
}

impl Mul for ADNum {
    type Output = ADNum;

    fn mul(self, other: ADNum) -> ADNum {
        let result = self.v * other.v;
        let lhs_der = other.v;
        let rhs_der = self.v;
        ADNum {
            v: result,
            dv: lhs_der * self.dv + rhs_der * other.dv,
        }
    }
}

impl Sub for ADNum {
    type Output = ADNum;

    fn sub(self, other: ADNum) -> ADNum {
        let result = self.v - other.v;
        let lhs_der = 1.0;
        let rhs_der = -1.0;
        ADNum {
            v: result,
            dv: lhs_der * self.dv + rhs_der * other.dv,
        }
    }
}

impl Div for ADNum {
    type Output = ADNum;

    fn div(self, other: ADNum) -> ADNum {
        let result = self.v / other.v;
        let lhs_der = 1.0 / other.v;
        let rhs_der = -self.v / (other.v * other.v);
        let div_deriv = (lhs_der * self.dv - rhs_der * other.dv) / (other.v * other.v);
        ADNum {
            v: result,
            dv: div_deriv,
        }
    }
}

impl Mul<f64> for ADNum {
    type Output = ADNum;

    fn mul(self, other: f64) -> ADNum {
        let result = self.v * other;
        let lhs_der = other;
        ADNum {
            v: result,
            dv: lhs_der * self.dv,
        }
    }
}

impl Mul<ADNum> for f64 {
    type Output = ADNum;

    fn mul(self, other: ADNum) -> ADNum {
        let result = self * other.v;
        let rhs_der = self;
        ADNum {
            v: result,
            dv: rhs_der * other.dv,
        }
    }
}

pub fn cos(x: ADNum) -> ADNum {
    let result = x.v.cos();
    let lhs_der = -x.v.sin();
    ADNum {
        v: result,
        dv: lhs_der * x.dv,
    }
}

pub fn powf(x: ADNum, y: f64) -> ADNum {
    let result = x.v.powf(y);
    let lhs_der = y * x.v.powf(y - 1.0);
    ADNum {
        v: result,
        dv: lhs_der * x.dv,
    }
}

pub fn derivative<F>(func: F, x0: f64) -> (f64, f64)
where
    F: Fn(ADNum) -> ADNum,
{
    let eval = func(ADNum { v: x0, dv: 1.0 });
    let val = eval.value();
    let der = eval.derivative();
    (val, der)
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn ad_f(x: ADNum) -> ADNum {
        2.0 * x * x + 2.0 * cos(x)
    }

    pub fn f(x: f64) -> f64 {
        2.0 * x * x + 2.0 * x.cos()
    }

    pub fn f_dx(x: f64) -> f64 {
        4.0 * x - 2.0 * x.sin()
    }

    #[test]
    fn example() {
        let x0 = 2.0;
        let (ad_f, ad_dx) = derivative(ad_f, x0);
        let (f, dx) = (f(x0), f_dx(x0));

        println!("ad_f: {}, ad_dx: {}", ad_f, ad_dx);
        println!("f: {}, dx: {}", f, dx);
    }
}
