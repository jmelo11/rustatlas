use std::{
    fmt::Display,
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, Sub, SubAssign},
};

use num_traits::{real::Real, Num, NumCast, One, ToPrimitive, Zero};
use serde::{Deserialize, Serialize};

use crate::core::meta::NewValue;

use super::tape::TAPE;

/// f64 implementations
impl Add<ADNum> for f64 {
    type Output = ADNum;

    fn add(self, other: ADNum) -> ADNum {
        ADNum::new(self) + other
    }
}

impl Sub<ADNum> for f64 {
    type Output = ADNum;

    fn sub(self, other: ADNum) -> ADNum {
        ADNum::new(self) - other
    }
}

impl Mul<ADNum> for f64 {
    type Output = ADNum;

    fn mul(self, other: ADNum) -> ADNum {
        ADNum::new(self) * other
    }
}

impl Div<ADNum> for f64 {
    type Output = ADNum;

    fn div(self, other: ADNum) -> ADNum {
        ADNum::new(self) / other
    }
}

impl AddAssign<ADNum> for f64 {
    fn add_assign(&mut self, other: ADNum) {
        *self = *self + other.value;
    }
}

impl SubAssign<ADNum> for f64 {
    fn sub_assign(&mut self, other: ADNum) {
        *self = *self - other.value;
    }
}

impl MulAssign<ADNum> for f64 {
    fn mul_assign(&mut self, other: ADNum) {
        *self = *self * other.value;
    }
}

impl DivAssign<ADNum> for f64 {
    fn div_assign(&mut self, other: ADNum) {
        *self = *self / other.value;
    }
}

impl PartialEq<ADNum> for f64 {
    fn eq(&self, other: &ADNum) -> bool {
        *self == other.value
    }
}

impl PartialOrd<ADNum> for f64 {
    fn partial_cmp(&self, other: &ADNum) -> Option<std::cmp::Ordering> {
        self.partial_cmp(&other.value)
    }
}

/// # ADNode
/// A node that represents the differential operation performed on variables inside the computation.
#[derive(Debug, Clone)]
pub struct ADNode {
    der: [f64; 2],
    ids: [Option<usize>; 2],
    n_args: usize,
}

impl ADNode {
    pub fn new(der: [f64; 2], ids: [Option<usize>; 2], n_args: usize) -> ADNode {
        ADNode { der, ids, n_args }
    }

    pub fn lhs_der(&self) -> f64 {
        self.der[0]
    }

    pub fn rhs_der(&self) -> f64 {
        self.der[1]
    }

    pub fn lhs_id(&self) -> Option<usize> {
        self.ids[0]
    }

    pub fn rhs_id(&self) -> Option<usize> {
        self.ids[1]
    }

    pub fn n_args(&self) -> usize {
        self.n_args
    }
}

/// # ADNum
/// A number that supports automatic differentiation.
#[derive(Debug, Clone, Copy)]
pub struct ADNum {
    value: f64,
    id: usize,
}

impl NewValue for ADNum {
    fn new(value: f64) -> ADNum {
        let id = TAPE.with(|tape| tape.tape_size());
        // Create a new node for the new variable
        let node = ADNode::new([1.0, 0.0], [None, None], 0);
        TAPE.with(|tape| tape.push_node(node));
        ADNum { value, id }
    }
}

impl ADNum {
    pub fn id(&self) -> usize {
        self.id
    }
}

/// ADNum implementations
impl Serialize for ADNum {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.value.serialize(serializer)
    }
}

impl<'a> Deserialize<'a> for ADNum {
    fn deserialize<D>(deserializer: D) -> Result<ADNum, D::Error>
    where
        D: serde::de::Deserializer<'a>,
    {
        let value = f64::deserialize(deserializer)?;
        Ok(ADNum::new(value))
    }
}

/// Float traits
impl Zero for ADNum {
    fn zero() -> Self {
        ADNum::new(0.0)
    }

    fn is_zero(&self) -> bool {
        self.value.is_zero()
    }
}

impl One for ADNum {
    fn one() -> Self {
        ADNum::new(1.0)
    }
}

impl Rem for ADNum {
    type Output = ADNum;

    fn rem(self, other: ADNum) -> ADNum {
        self.value.rem(other.value).into()
    }
}

impl Num for ADNum {
    type FromStrRadixErr = <f64 as Num>::FromStrRadixErr;
    fn from_str_radix(str: &str, radix: u32) -> std::result::Result<Self, Self::FromStrRadixErr> {
        f64::from_str_radix(str, radix).map(|v| ADNum::new(v))
    }
}

impl ToPrimitive for ADNum {
    fn to_f32(&self) -> Option<f32> {
        self.value.to_f32()
    }
    fn to_f64(&self) -> Option<f64> {
        self.value.to_f64()
    }

    fn to_isize(&self) -> Option<isize> {
        self.value.to_isize()
    }

    fn to_i8(&self) -> Option<i8> {
        self.value.to_i8()
    }

    fn to_i16(&self) -> Option<i16> {
        self.value.to_i16()
    }

    fn to_i32(&self) -> Option<i32> {
        self.value.to_i32()
    }

    fn to_i128(&self) -> Option<i128> {
        self.value.to_i128()
    }

    fn to_usize(&self) -> Option<usize> {
        self.value.to_usize()
    }

    fn to_u8(&self) -> Option<u8> {
        self.value.to_u8()
    }

    fn to_u16(&self) -> Option<u16> {
        self.value.to_u16()
    }

    fn to_u32(&self) -> Option<u32> {
        self.value.to_u32()
    }

    fn to_u128(&self) -> Option<u128> {
        self.value.to_u128()
    }

    fn to_i64(&self) -> Option<i64> {
        self.value.to_i64()
    }

    fn to_u64(&self) -> Option<u64> {
        self.value.to_u64()
    }
}

impl NumCast for ADNum {
    fn from<T: num_traits::ToPrimitive>(n: T) -> Option<Self> {
        n.to_f64().map(|v| ADNum::new(v))
    }
}

#[allow(unused)]
impl Real for ADNum {
    fn min_value() -> Self {
        todo!()
    }

    fn min_positive_value() -> Self {
        todo!()
    }

    fn max_value() -> Self {
        todo!()
    }

    fn floor(self) -> Self {
        todo!()
    }

    fn ceil(self) -> Self {
        todo!()
    }

    fn round(self) -> Self {
        todo!()
    }

    fn trunc(self) -> Self {
        todo!()
    }

    fn fract(self) -> Self {
        todo!()
    }

    fn abs(self) -> Self {
        let result = self.value.abs();
        let der = self.value.signum();
        let id = TAPE.with(|tape| tape.tape_size());
        let node = ADNode::new([der, 0.0], [Some(self.id), None], 1);
        TAPE.with(|tape| tape.push_node(node));
        ADNum { value: result, id }
    }

    fn signum(self) -> Self {
        self.value.signum().into()
    }

    fn is_sign_positive(self) -> bool {
        self.value.is_sign_positive()
    }

    fn is_sign_negative(self) -> bool {
        self.value.is_sign_negative()
    }

    fn mul_add(self, a: Self, b: Self) -> Self {
        let result = self.value.mul_add(a.value, b.value);
        let der = a.value;
        let id = TAPE.with(|tape| tape.tape_size());
        let node = ADNode::new([der, 0.0], [Some(self.id), None], 1);
        TAPE.with(|tape| tape.push_node(node));
        ADNum { value: result, id }
    }

    fn recip(self) -> Self {
        let result = self.value.recip();
        let der = -1.0 / self.value.powi(2);
        let id = TAPE.with(|tape| tape.tape_size());
        let node = ADNode::new([der, 0.0], [Some(self.id), None], 1);
        TAPE.with(|tape| tape.push_node(node));
        ADNum { value: result, id }
    }

    fn powi(self, n: i32) -> Self {
        let result = self.value.powi(n);
        let der = n as f64 * self.value.powi(n - 1);
        let id = TAPE.with(|tape| tape.tape_size());
        let node = ADNode::new([der, 0.0], [Some(self.id), None], 1);
        TAPE.with(|tape| tape.push_node(node));
        ADNum { value: result, id }
    }

    fn powf(self, n: Self) -> Self {
        let result = self.value.powf(n.value);
        let der = n.value * self.value.powf(n.value - 1.0);
        let id = TAPE.with(|tape| tape.tape_size());
        let node = ADNode::new([der, 0.0], [Some(self.id), None], 1);
        TAPE.with(|tape| tape.push_node(node));
        ADNum { value: result, id }
    }

    fn sqrt(self) -> Self {
        let result = self.value.sqrt();
        let der = 0.5 / self.value.sqrt();
        let id = TAPE.with(|tape| tape.tape_size());
        let node = ADNode::new([der, 0.0], [Some(self.id), None], 1);
        TAPE.with(|tape| tape.push_node(node));
        ADNum { value: result, id }
    }

    fn exp(self) -> Self {
        let result = self.value.exp();
        let der = self.value.exp();
        let id = TAPE.with(|tape| tape.tape_size());
        let node = ADNode::new([der, 0.0], [Some(self.id), None], 1);
        TAPE.with(|tape| tape.push_node(node));
        ADNum { value: result, id }
    }

    fn exp2(self) -> Self {
        todo!()
    }

    fn ln(self) -> Self {
        let result = self.value.ln();
        let der = 1.0 / self.value;
        let id = TAPE.with(|tape| tape.tape_size());
        let node = ADNode::new([der, 0.0], [Some(self.id), None], 1);
        TAPE.with(|tape| tape.push_node(node));
        ADNum { value: result, id }
    }

    fn log(self, base: Self) -> Self {
        todo!()
    }

    fn log2(self) -> Self {
        todo!()
    }

    fn log10(self) -> Self {
        todo!()
    }

    fn max(self, other: Self) -> Self {
        todo!()
    }

    fn min(self, other: Self) -> Self {
        todo!()
    }

    fn abs_sub(self, other: Self) -> Self {
        todo!()
    }

    fn cbrt(self) -> Self {
        todo!()
    }

    fn hypot(self, other: Self) -> Self {
        todo!()
    }

    fn sin(self) -> Self {
        let result = self.value.sin();
        let der = self.value.cos();
        let id = TAPE.with(|tape| tape.tape_size());
        let node = ADNode::new([der, 0.0], [Some(self.id), None], 1);
        TAPE.with(|tape| tape.push_node(node));
        ADNum { value: result, id }
    }

    fn cos(self) -> Self {
        let result = self.value.cos();
        let der = -self.value.sin();
        let id = TAPE.with(|tape| tape.tape_size());
        let node = ADNode::new([der, 0.0], [Some(self.id), None], 1);
        TAPE.with(|tape| tape.push_node(node));
        ADNum { value: result, id }
    }

    fn tan(self) -> Self {
        todo!()
    }

    fn asin(self) -> Self {
        todo!()
    }

    fn acos(self) -> Self {
        todo!()
    }

    fn atan(self) -> Self {
        todo!()
    }

    fn atan2(self, other: Self) -> Self {
        todo!()
    }

    fn sin_cos(self) -> (Self, Self) {
        todo!()
    }

    fn exp_m1(self) -> Self {
        todo!()
    }

    fn ln_1p(self) -> Self {
        todo!()
    }

    fn sinh(self) -> Self {
        todo!()
    }

    fn cosh(self) -> Self {
        todo!()
    }

    fn tanh(self) -> Self {
        let result = self.value.tanh();
        let der = 1.0 - self.value.tanh().powi(2);
        let id = TAPE.with(|tape| tape.tape_size());
        let node = ADNode::new([der, 0.0], [Some(self.id), None], 1);
        TAPE.with(|tape| tape.push_node(node));
        ADNum { value: result, id }
    }

    fn asinh(self) -> Self {
        let result = self.value.asinh();
        let der = 1.0 / (self.value.powi(2) + 1.0).sqrt();
        let id = TAPE.with(|tape| tape.tape_size());
        let node = ADNode::new([der, 0.0], [Some(self.id), None], 1);
        TAPE.with(|tape| tape.push_node(node));
        ADNum { value: result, id }
    }

    fn acosh(self) -> Self {
        let result = self.value.acosh();
        let der = 1.0 / (self.value.powi(2) - 1.0).sqrt();
        let id = TAPE.with(|tape| tape.tape_size());
        let node = ADNode::new([der, 0.0], [Some(self.id), None], 1);
        TAPE.with(|tape| tape.push_node(node));
        ADNum { value: result, id }
    }

    fn atanh(self) -> Self {
        let result = self.value.atanh();
        let der = 1.0 / (1.0 - self.value.powi(2));
        let id = TAPE.with(|tape| tape.tape_size());
        let node = ADNode::new([der, 0.0], [Some(self.id), None], 1);
        TAPE.with(|tape| tape.push_node(node));
        ADNum { value: result, id }
    }

    fn epsilon() -> Self {
        f64::epsilon().into()
    }

    fn to_degrees(self) -> Self {
        self.value.to_degrees().into()
    }

    fn to_radians(self) -> Self {
        self.value.to_radians().into()
    }
}

impl From<f64> for ADNum {
    fn from(value: f64) -> Self {
        ADNum::new(value)
    }
}

impl Sum for ADNum {
    fn sum<I: Iterator<Item = ADNum>>(iter: I) -> Self {
        iter.fold(ADNum::new(0.0), |acc, x| acc + x)
    }
}
/// Basic operations
impl Add for ADNum {
    type Output = ADNum;

    fn add(self, other: ADNum) -> ADNum {
        let result = self.value + other.value;
        let lhs_der = 1.0;
        let rhs_der = 1.0;
        let lhs_id = Some(self.id);
        let rhs_id = Some(other.id);

        let node = ADNode::new([lhs_der.into(), rhs_der.into()], [lhs_id, rhs_id], 2);
        let id = TAPE.with(|tape| tape.tape_size());
        TAPE.with(|tape| tape.push_node(node));
        ADNum {
            value: result,
            id: id,
        }
    }
}

impl Add<f64> for ADNum {
    type Output = ADNum;

    fn add(self, other: f64) -> ADNum {
        let result = self.value + other;
        let lhs_der = 1.0;
        let rhs_der = 0.0;
        let lhs_id = Some(self.id);
        let rhs_id = None;

        let node = ADNode::new([lhs_der, rhs_der], [lhs_id, rhs_id], 1);
        let id = TAPE.with(|tape| tape.tape_size());
        TAPE.with(|tape| tape.push_node(node));
        ADNum {
            value: result,
            id: id,
        }
    }
}

impl Sub for ADNum {
    type Output = ADNum;

    fn sub(self, other: ADNum) -> ADNum {
        let result = self.value - other.value;
        let lhs_der = 1.0;
        let rhs_der = -1.0;
        let lhs_id = Some(self.id);
        let rhs_id = Some(other.id);

        let node = ADNode::new([lhs_der.into(), rhs_der.into()], [lhs_id, rhs_id], 2);
        let id = TAPE.with(|tape| tape.tape_size());
        TAPE.with(|tape| tape.push_node(node));
        ADNum {
            value: result,
            id: id,
        }
    }
}

impl Sub<f64> for ADNum {
    type Output = ADNum;

    fn sub(self, other: f64) -> ADNum {
        let result = self.value - other;
        let lhs_der = 1.0;
        let rhs_der = 0.0;
        let lhs_id = Some(self.id);
        let rhs_id = None;

        let node = ADNode::new([lhs_der, rhs_der], [lhs_id, rhs_id], 1);
        let id = TAPE.with(|tape| tape.tape_size());
        TAPE.with(|tape| tape.push_node(node));
        ADNum {
            value: result,
            id: id,
        }
    }
}

impl Mul for ADNum {
    type Output = ADNum;

    fn mul(self, other: ADNum) -> ADNum {
        let result = self.value * other.value;
        let lhs_der = other.value;
        let rhs_der = self.value;
        let lhs_id = Some(self.id);
        let rhs_id = Some(other.id);

        let node = ADNode::new([lhs_der, rhs_der], [lhs_id, rhs_id], 2);
        let id = TAPE.with(|tape| tape.tape_size());
        TAPE.with(|tape| tape.push_node(node));

        ADNum {
            value: result,
            id: id,
        }
    }
}

impl Mul<f64> for ADNum {
    type Output = ADNum;

    fn mul(self, other: f64) -> ADNum {
        let result = self.value * other;
        let lhs_der = other;
        let rhs_der = self.value;
        let lhs_id = Some(self.id);
        let rhs_id = None;

        let node = ADNode::new([lhs_der, rhs_der], [lhs_id, rhs_id], 1);
        let id = TAPE.with(|tape| tape.tape_size());
        TAPE.with(|tape| tape.push_node(node));

        ADNum {
            value: result,
            id: id,
        }
    }
}

impl Div for ADNum {
    type Output = ADNum;

    fn div(self, other: ADNum) -> ADNum {
        let result = self.value / other.value;
        let lhs_der = 1.0 / other.value;
        let rhs_der = -self.value / (other.value * other.value);
        let lhs_id = Some(self.id);
        let rhs_id = Some(other.id);

        let node = ADNode::new([lhs_der, rhs_der], [lhs_id, rhs_id], 2);
        let id = TAPE.with(|tape| tape.tape_size());
        TAPE.with(|tape| tape.push_node(node));

        ADNum {
            value: result,
            id: id,
        }
    }
}

impl Div<f64> for ADNum {
    type Output = ADNum;

    fn div(self, other: f64) -> ADNum {
        let result = self.value / other;
        let lhs_der = 1.0 / other;
        let rhs_der = 0.0;
        let lhs_id = Some(self.id);
        let rhs_id = None;

        let node = ADNode::new([lhs_der, rhs_der], [lhs_id, rhs_id], 1);
        let id = TAPE.with(|tape| tape.tape_size());
        TAPE.with(|tape| tape.push_node(node));

        ADNum {
            value: result,
            id: id,
        }
    }
}

/// Unary operations
impl Neg for ADNum {
    type Output = ADNum;

    fn neg(self) -> ADNum {
        let result = -self.value;
        let id = TAPE.with(|tape| tape.tape_size());
        ADNum { value: result, id }
    }
}

impl AddAssign for ADNum {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl SubAssign for ADNum {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl MulAssign for ADNum {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl DivAssign for ADNum {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

/// Logical operations
/// PartialEq
impl PartialEq for ADNum {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
/// PartialOrd
impl PartialOrd for ADNum {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

/// Cmp to f64
impl PartialEq<f64> for ADNum {
    fn eq(&self, other: &f64) -> bool {
        self.value == *other
    }
}

/// PartialOrd to f64
impl PartialOrd<f64> for ADNum {
    fn partial_cmp(&self, other: &f64) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(other)
    }
}

/// Display trait implementation for ADNum
impl Display for ADNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let x = ADNum::new(2.0);
        let y = ADNum::new(3.0);
        let z = x + y;
        assert_eq!(z.value, 5.0);
    }

    #[test]
    fn test_sub() {
        let x = ADNum::new(2.0);
        let y = ADNum::new(3.0);
        let z = x - y;
        assert_eq!(z.value, -1.0);
    }

    #[test]
    fn test_mul() {
        let x = ADNum::new(2.0);
        let y = ADNum::new(3.0);
        let z = x * y;
        assert_eq!(z.value, 6.0);
    }

    #[test]
    fn test_div() {
        let x = ADNum::new(2.0);
        let y = ADNum::new(3.0);
        let z = x / y;
        assert_eq!(z.value, 2.0 / 3.0);
    }

    #[test]
    fn test_add_assign() {
        let mut x = ADNum::new(2.0);
        let y = ADNum::new(3.0);
        x += y;
        assert_eq!(x.value, 5.0);
    }

    #[test]
    fn test_sub_assign() {
        let mut x = ADNum::new(2.0);
        let y = ADNum::new(3.0);
        x -= y;
        assert_eq!(x.value, -1.0);
    }

    #[test]
    fn test_mul_assign() {
        let mut x = ADNum::new(2.0);
        let y = ADNum::new(3.0);
        x *= y;
        assert_eq!(x.value, 6.0);
    }

    #[test]
    fn test_div_assign() {
        let mut x = ADNum::new(2.0);
        let y = ADNum::new(3.0);
        x /= y;
        assert_eq!(x.value, 2.0 / 3.0);
    }

    #[test]
    fn test_eq() {
        let x = ADNum::new(2.0);
        let y = ADNum::new(2.0);
        assert_eq!(x, y);
    }
}

#[cfg(test)]
mod math_ops_tests {
    use super::*;

    #[test]
    fn test_tape_size() {
        let x = ADNum::new(2.0);
        let y = ADNum::new(3.0);
        let _ = x + y;
        let tape_size = TAPE.with(|tape| tape.tape_size());
        assert_eq!(tape_size, 3);
    }

    #[test]
    fn test_derivative_add() {
        let x = ADNum::new(2.0);
        let y = ADNum::new(3.0);
        let z = x + y;
        let derivative = TAPE.with(|tape| tape.adjoints(&z));
        assert_eq!(derivative, vec![1.0, 1.0, 1.0]);
    }

    #[test]
    fn test_derivative_div() {
        let x = ADNum::new(2.0);
        let y = ADNum::new(3.0);
        let z = x / y;
        let derivative: Vec<f64> = TAPE.with(|tape| tape.adjoints(&z));
        assert_eq!(derivative, vec![1.0 / 3.0, -2.0 / 9.0, 1.0]);
    }

    #[test]
    fn test_derivative_mul() {
        let x = ADNum::new(2.0);
        let y = ADNum::new(3.0);
        let z = x * y;
        let derivative = TAPE.with(|tape| tape.adjoints(&z));
        assert_eq!(derivative, vec![3.0, 2.0, 1.0]);
    }

    #[test]
    fn test_derivative_sub() {
        let x = ADNum::new(2.0);
        let y = ADNum::new(3.0);
        let z = x - y;
        let derivative = TAPE.with(|tape| tape.adjoints(&z));
        assert_eq!(derivative, vec![1.0, -1.0, 1.0]);
    }
}

#[cfg(test)]
mod unary_ops_tests {
    use super::*;

    #[test]
    fn test_derivative_sin() {
        TAPE.with(|tape| tape.activate());
        let v = 10.0;
        let x = ADNum::new(v);
        let y = x.sin();
        let derivative = TAPE.with(|tape| tape.adjoints(&y));
        assert_eq!(derivative, vec![v.cos(), 1.0]);
    }

    #[test]
    fn test_derivative_cos() {
        TAPE.with(|tape| tape.activate());
        let v = 10.0;
        let x = ADNum::new(v);
        let y = x.cos();
        let derivative = TAPE.with(|tape| tape.adjoints(&y));
        assert_eq!(derivative, vec![-v.sin(), 1.0]);
    }

    #[test]
    fn test_derivative_exp() {
        TAPE.with(|tape| tape.activate());
        let x = ADNum::new(0.0);
        let y = x.exp();
        let derivative = TAPE.with(|tape| tape.adjoints(&y));
        assert_eq!(derivative, vec![1.0, 1.0]);

        let v = 10.0;
        let x: ADNum = v.into();
        let y = x.exp();
        let derivative = TAPE.with(|tape| tape.adjoints(&y));
        assert_eq!(derivative, vec![0.0, 0.0, v.exp(), 1.0]);
    }

    #[test]
    fn test_derivative_log() {
        TAPE.with(|tape| tape.activate());
        let v = 10.0;
        let x = ADNum::new(v);
        let y = x.ln();
        let derivative = TAPE.with(|tape| tape.adjoints(&y));
        assert_eq!(derivative, vec![1.0 / v, 1.0]);
    }

    #[test]
    fn test_derivative_add() {
        TAPE.with(|tape| tape.activate());
        let x = ADNum::new(2.0);
        let mut y = ADNum::new(3.0);
        y += x;
        let derivative = TAPE.with(|tape| tape.adjoints(&y));
        assert_eq!(derivative, vec![1.0, 1.0, 1.0]);
    }

    #[test]
    fn test_derivative_sub() {
        TAPE.with(|tape| tape.activate());
        let x = ADNum::new(2.0);
        let mut y = ADNum::new(3.0);
        y -= x;
        let derivative = TAPE.with(|tape| tape.adjoints(&y));
        assert_eq!(derivative, vec![-1.0, 1.0, 1.0]);
    }

    #[test]
    fn test_derivative_mul() {
        TAPE.with(|tape| tape.activate());
        let x = ADNum::new(2.0);
        let mut y = ADNum::new(3.0);
        y *= x;
        let derivative = TAPE.with(|tape| tape.adjoints(&y));
        assert_eq!(derivative, vec![3.0, 2.0, 1.0]);
    }

    #[test]
    fn test_derivative_div() {
        TAPE.with(|tape| tape.activate());
        let x = ADNum::new(2.0);
        let mut y = ADNum::new(1.0);
        y /= x;
        let derivative = TAPE.with(|tape| tape.adjoints(&y));
        assert_eq!(derivative, vec![-1.0 / 4.0, 0.5, 1.0]);
        let dy_dx = derivative.get(x.id()).unwrap();
        assert_eq!(*dy_dx, -1.0 / 4.0);
    }
}
