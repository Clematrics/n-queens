use crate::solver::{Configuration, Solver};

#[derive(Copy, Clone)]
pub struct StrategyParameters {
    pub stop_after_first_solution: bool,
}

pub trait Strategy {
    fn new(param: StrategyParameters) -> Self;

    fn has_next_batch(&mut self, solver: &mut Solver) -> bool;

    fn next_step(&mut self, solver: &mut Solver) -> Option<Configuration>;
}
