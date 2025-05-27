use std::cell::RefCell;
use std::ops::{Add, Div, Mul, Neg, Sub};

thread_local! {
    static TAPE: RefCell<Vec<Node>> = RefCell::new(Vec::new());
}

#[derive(Clone, Copy)]
enum Op {
    Input,
    Add,
    Sub,
    Mul,
    Div,
    Neg,
    Exp,
    Ln,
}

#[derive(Clone, Copy)]
struct Node {
    value: f64,
    op: Op,
    lhs: Option<usize>,
    rhs: Option<usize>,
}

fn push(node: Node) -> usize {
    TAPE.with(|t| {
        let mut t = t.borrow_mut();
        t.push(node);
        t.len() - 1
    })
}

fn value_of(id: usize) -> f64 {
    TAPE.with(|t| t.borrow()[id].value)
}

pub fn reset_tape() {
    TAPE.with(|t| t.borrow_mut().clear())
}

#[derive(Clone, Copy, Debug)]
pub struct Var {
    id: usize,
}

impl Var {
    pub fn new(value: f64) -> Var {
        let id = push(Node {
            value,
            op: Op::Input,
            lhs: None,
            rhs: None,
        });
        Var { id }
    }

    pub fn value(&self) -> f64 {
        value_of(self.id)
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn exp(self) -> Var {
        let v = self.value().exp();
        let id = push(Node {
            value: v,
            op: Op::Exp,
            lhs: Some(self.id),
            rhs: None,
        });
        Var { id }
    }

    pub fn ln(self) -> Var {
        let v = self.value().ln();
        let id = push(Node {
            value: v,
            op: Op::Ln,
            lhs: Some(self.id),
            rhs: None,
        });
        Var { id }
    }

    pub fn powf(self, n: f64) -> Var {
        (self.ln() * Var::from(n)).exp()
    }
}

impl From<f64> for Var {
    fn from(value: f64) -> Self {
        Var::new(value)
    }
}

impl From<Var> for f64 {
    fn from(v: Var) -> Self {
        v.value()
    }
}

impl crate::utils::num::FloatOps for Var {
    fn exp(self) -> Self {
        Var::exp(self)
    }

    fn ln(self) -> Self {
        Var::ln(self)
    }

    fn powf(self, n: f64) -> Self {
        Var::powf(self, n)
    }
}

impl Add for Var {
    type Output = Var;
    fn add(self, rhs: Var) -> Var {
        let v = self.value() + rhs.value();
        let id = push(Node {
            value: v,
            op: Op::Add,
            lhs: Some(self.id),
            rhs: Some(rhs.id),
        });
        Var { id }
    }
}

impl Sub for Var {
    type Output = Var;
    fn sub(self, rhs: Var) -> Var {
        let v = self.value() - rhs.value();
        let id = push(Node {
            value: v,
            op: Op::Sub,
            lhs: Some(self.id),
            rhs: Some(rhs.id),
        });
        Var { id }
    }
}

impl Mul for Var {
    type Output = Var;
    fn mul(self, rhs: Var) -> Var {
        let v = self.value() * rhs.value();
        let id = push(Node {
            value: v,
            op: Op::Mul,
            lhs: Some(self.id),
            rhs: Some(rhs.id),
        });
        Var { id }
    }
}

impl Div for Var {
    type Output = Var;
    fn div(self, rhs: Var) -> Var {
        let v = self.value() / rhs.value();
        let id = push(Node {
            value: v,
            op: Op::Div,
            lhs: Some(self.id),
            rhs: Some(rhs.id),
        });
        Var { id }
    }
}

impl Neg for Var {
    type Output = Var;
    fn neg(self) -> Var {
        let v = -self.value();
        let id = push(Node {
            value: v,
            op: Op::Neg,
            lhs: Some(self.id),
            rhs: None,
        });
        Var { id }
    }
}

pub fn backward(result: &Var) -> Vec<f64> {
    TAPE.with(|t| {
        let tape = t.borrow();
        let mut grad = vec![0.0; tape.len()];
        grad[result.id] = 1.0;
        for i in (0..=result.id).rev() {
            let node = &tape[i];
            match node.op {
                Op::Input => {}
                Op::Add => {
                    let l = node.lhs.unwrap();
                    let r = node.rhs.unwrap();
                    grad[l] += grad[i];
                    grad[r] += grad[i];
                }
                Op::Sub => {
                    let l = node.lhs.unwrap();
                    let r = node.rhs.unwrap();
                    grad[l] += grad[i];
                    grad[r] -= grad[i];
                }
                Op::Mul => {
                    let l = node.lhs.unwrap();
                    let r = node.rhs.unwrap();
                    let lv = tape[l].value;
                    let rv = tape[r].value;
                    grad[l] += grad[i] * rv;
                    grad[r] += grad[i] * lv;
                }
                Op::Div => {
                    let l = node.lhs.unwrap();
                    let r = node.rhs.unwrap();
                    let lv = tape[l].value;
                    let rv = tape[r].value;
                    grad[l] += grad[i] / rv;
                    grad[r] -= grad[i] * lv / (rv * rv);
                }
                Op::Neg => {
                    let l = node.lhs.unwrap();
                    grad[l] -= grad[i];
                }
                Op::Exp => {
                    let l = node.lhs.unwrap();
                    let v = tape[i].value;
                    grad[l] += grad[i] * v;
                }
                Op::Ln => {
                    let l = node.lhs.unwrap();
                    let v = tape[l].value;
                    grad[l] += grad[i] / v;
                }
            }
        }
        grad
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn square_plus_test() {
        reset_tape();
        let x = Var::new(2.0);
        let y = x * x + x;
        let grad = backward(&y);
        assert!((grad[x.id()] - 5.0).abs() < 1e-12);
    }

    #[test]
    fn multivar_test() {
        reset_tape();
        let x = Var::new(3.0);
        let y = Var::new(4.0);
        let z = x * y + y * y;
        let grad = backward(&z);
        assert!((grad[x.id()] - 4.0).abs() < 1e-12);
        assert!((grad[y.id()] - 11.0).abs() < 1e-12);
    }

    #[test]
    fn div_sub_test() {
        reset_tape();
        let x = Var::new(5.0);
        let y = Var::new(2.0);
        let z = (x / y) - x;
        let grad = backward(&z);
        assert!((grad[x.id()] + 0.5).abs() < 1e-12);
        assert!((grad[y.id()] + 1.25).abs() < 1e-12);
    }

    #[test]
    fn exp_ln_powf_test() {
        reset_tape();
        let x = Var::new(2.0);
        let y = x.ln().exp() + x.powf(2.0);
        let grad = backward(&y);
        // derivative of ln(x).exp() is 1/x * exp(ln(x)) = 1/x * x = 1
        // derivative of x.powf(2) is 2*x
        assert!((grad[x.id()] - (1.0 + 2.0 * 2.0)).abs() < 1e-12);
    }
}

