extern crate rustatlas;

use rustatlas::math::ad::{backward, reset_tape, Var};
use rustatlas::utils::num::FloatOps;

fn main() {
    reset_tape();
    let r = Var::new(0.05);
    let t = Var::new(2.0);
    // discount factor = exp(-r * t)
    let df = (-r * t).exp();
    let grad = backward(&df);

    println!("discount factor: {}", df.value());
    println!("d(df)/d(r): {}", grad[r.id()]);
    println!("d(df)/d(t): {}", grad[t.id()]);
}
