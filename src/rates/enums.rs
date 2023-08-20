/// # Compounding
/// Enumerate the different compounding methods.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Compounding {
    Simple,
    Compounded,
    Continuous,
    SimpleThenCompounded,
    CompoundedThenSimple,
}
