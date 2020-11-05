use crate::solver::*;
use crate::strategy::*;

pub struct EachSolution {
    in_batch: bool,
    params: StrategyParameters,
}

impl Strategy for EachSolution {
    fn new(params: StrategyParameters) -> Self {
        Self {
            in_batch: false,
            params,
        }
    }

    fn has_next_batch(&mut self, solver: &mut Solver) -> bool {
        self.in_batch = false;
        !(self.params.stop_after_first_solution && solver.solutions_found() > 0)
            && !solver.is_finished()
    }

    fn next_step(&mut self, solver: &mut Solver) -> Option<Configuration> {
        if !self.in_batch {
            self.in_batch = true;
            while let Some(config) = solver.search() {
                if config.is_valid == true {
                    return Some(config);
                }
            }
            return None;
        } else {
            None
        }
    }
}
