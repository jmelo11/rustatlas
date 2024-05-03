use std::ops::{Add, Div, Mul, Sub};

use crate::{
    math::aad::binary::{AddOp, DivOp, MulOp, SubOp},
    math::aad::expression::{BinaryOp, Derivatives, EvalExpr, Expr},
};

/// # Add
impl<T, U> Add<Expr<U>> for Expr<T>
where
    T: EvalExpr + Derivatives,
    U: EvalExpr + Derivatives,
{
    type Output = Expr<BinaryOp<T, U, AddOp>>;
    fn add(self, rhs: Expr<U>) -> Self::Output {
        Expr::new(BinaryOp::new(self.expr(), rhs.expr()))
    }
}

/// # Sub
impl<T, U> Sub<Expr<U>> for Expr<T>
where
    T: EvalExpr + Derivatives,
    U: EvalExpr + Derivatives,
{
    type Output = Expr<BinaryOp<T, U, SubOp>>;
    fn sub(self, rhs: Expr<U>) -> Self::Output {
        Expr::new(BinaryOp::new(self.expr(), rhs.expr()))
    }
}

/// # Mul
impl<T, U> Mul<Expr<U>> for Expr<T>
where
    T: EvalExpr + Derivatives,
    U: EvalExpr + Derivatives,
{
    type Output = Expr<BinaryOp<T, U, MulOp>>;
    fn mul(self, rhs: Expr<U>) -> Self::Output {
        Expr::new(BinaryOp::new(self.expr(), rhs.expr()))
    }
}

/// # Div
impl<T, U> Div<Expr<U>> for Expr<T>
where
    T: EvalExpr + Derivatives,
    U: EvalExpr + Derivatives,
{
    type Output = Expr<BinaryOp<T, U, DivOp>>;
    fn div(self, rhs: Expr<U>) -> Self::Output {
        Expr::new(BinaryOp::new(self.expr(), rhs.expr()))
    }
}
