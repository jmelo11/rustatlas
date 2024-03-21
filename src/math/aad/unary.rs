use crate::math::aad::expression::{ApplyUnaryOp, Derivative, EvalExpr, Expr, UnaryOp};

/// # SinOp
/// This type is used by unary operations.
#[derive(Debug, Clone, Copy)]
pub struct SinOp;

impl<T> ApplyUnaryOp<T> for SinOp
where
    T: EvalExpr + Derivative,
{
    fn apply(expr: &T) -> f64 {
        expr.evaluate().sin()
    }
}

impl Derivative for SinOp {
    fn derivative(&self, v: f64) -> f64 {
        v.cos()
    }
}

/// # ExpOp
/// This type is used by unary operations.
#[derive(Debug, Clone, Copy)]
pub struct ExpOp;

impl<T> ApplyUnaryOp<T> for ExpOp
where
    T: EvalExpr + Derivative,
{
    fn apply(expr: &T) -> f64 {
        expr.evaluate().exp()
    }
}

impl Derivative for ExpOp {
    fn derivative(&self, v: f64) -> f64 {
        v.exp()
    }
}

pub fn sin<T: EvalExpr + Derivative>(expr: Expr<T>) -> Expr<UnaryOp<T, SinOp>> {
    Expr::new(UnaryOp::new(expr.expr()))
}

pub fn exp<T: EvalExpr + Derivative>(expr: Expr<T>) -> Expr<UnaryOp<T, ExpOp>> {
    Expr::new(UnaryOp::new(expr.expr()))
}

#[cfg(test)]
mod tests {
    use crate::math::aad::expression::Value;

    use super::*;

    #[test]
    fn test_sin() {
        let x = Value::new(1.0);
        let y = sin(x);
        assert_eq!(y.evaluate(), 1.0_f64.sin());
    }

    #[test]
    fn test_exp() {
        let x = Value::new(1.0);
        let y = exp(x);
        assert_eq!(y.evaluate(), 1.0_f64.exp());
    }
}
