use crate::solver::*;
use crate::strategy::*;

pub struct EachSolutionPartialSteps {
    batch_ended: bool,
    params: StrategyParameters,
}

impl Strategy for EachSolutionPartialSteps {
    fn new(params: StrategyParameters) -> Self {
        Self {
            batch_ended: false,
            params,
        }
    }

    fn has_next_batch(&mut self, solver: &mut Solver) -> bool {
        self.batch_ended = false;
        !(self.params.stop_after_first_solution && solver.solutions_found() > 0)
            && !solver.is_finished()
    }

    fn next_step(&mut self, solver: &mut Solver) -> Option<Configuration> {
        if self.batch_ended {
            return None;
        }

        let result = solver.search();
        if let Some(config) = result {
            if config.is_valid {
                self.batch_ended = true;
            }
            Some(config)
        } else {
            None
        }
    }
}
