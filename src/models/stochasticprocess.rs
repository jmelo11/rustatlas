/// StochasticProcess
pub trait StochasticProcess {
    fn driff(&self) -> f64;
    fn diffusion(&self) -> f64;
    fn expectation(&self) -> f64;
    fn variance(&self) -> f64;
}
