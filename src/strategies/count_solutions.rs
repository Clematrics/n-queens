use crate::solver::*;
use crate::strategy::*;

pub struct CountSolutions {
    _params: StrategyParameters,
}

impl Strategy for CountSolutions {
    fn new(params: StrategyParameters) -> Self {
        Self { _params: params }
    }

    fn has_next_batch(&mut self, solver: &mut Solver) -> bool {
        !solver.is_finished()
    }

    fn next_step(&mut self, solver: &mut Solver) -> Option<Configuration> {
        while let Some(_) = solver.search() {}
        None
    }
}
