use crate::solving::strategies;

pub struct Solver {
    strategies: Vec<Box<dyn strategies::Strategy>>,
}
