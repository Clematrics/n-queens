use std::io::Write;

use crate::driver::*;
use crate::solver::Solver;
use crate::strategy::Strategy;

pub struct ConsoleDriver {
    board_size: usize,
}

impl ConsoleDriver {
    pub fn new(board_size: usize) -> Self {
        Self { board_size }
    }
}

impl<T> Driver<T> for ConsoleDriver
where
    T: Strategy,
{
    fn execute(&mut self, solver: Solver, strategy: T, interaction_mode: InteractionMode) {
        let interaction = match interaction_mode {
            InteractionMode::NoInteraction => || {
                std::io::stdout().flush().unwrap();
                println!();
            },
            InteractionMode::WaitUser | InteractionMode::WaitOrTimeout(_) => || {
                std::io::stdout().flush().unwrap();
                let mut input = String::new();
                let _ = std::io::stdin().read_line(&mut input);
            },
        };

        println!("Searching for size {}...", self.board_size);

        let mut solver = solver;
        let mut strategy = strategy;
        while strategy.has_next_batch(&mut solver) {
            while let Some(config) = strategy.next_step(&mut solver) {
                print!("{:?}", config.configuration);
                if config.is_valid {
                    print!("\nSolution found!");
                }
                println!()
            }
            print!("\x1BA");
            interaction();
        }

        if solver.solutions_found() == 0 {
            println!("No solution!");
        } else {
            println!("Number of solutions found: {}", solver.solutions_found());
        }
        std::io::stdout().flush().unwrap();
    }
}
