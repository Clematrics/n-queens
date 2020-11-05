use n_queens::driver::*;
use n_queens::drivers::*;
use n_queens::solver::Solver;
use n_queens::strategies::*;
use n_queens::strategy::*;

#[macro_use]
extern crate clap;
use clap::{App, Arg};

const DEFAULT_BOARD_SIZE: usize = 7;

fn main() {
    let matches = App::new("N-Queens solver")
        .version("0.3.14.15...")
        .author("Clematrics")
		.about("Solve the N-Queens problem with different modes of interaction and visualisation")
		.arg(
			Arg::with_name("size")
				.index(1)
				.help(&format!("The size of the board, and so the number of queens. Default is {}", DEFAULT_BOARD_SIZE)[..])
		)
        .arg(
            Arg::with_name("driver")
                .long("-driver")
                .value_name("Driver")
                .possible_values(
                    &["Console", "3D"],
				)
                .help("Select the driver.\n'Console' writes in the console.\n3D will display a 3D scene to visualise the board.\nDefault is 'Console'.\n"),
        )
        .arg(
            Arg::with_name("strategy")
                .long("-strat")
                .value_name("Strategy")
                .possible_values(
                    &vec![
                        StrategySelection::AllSteps,
						StrategySelection::SolutionsWithSteps,
						StrategySelection::OnlySolutions,
						StrategySelection::OnlyCount,
                    ]
					.iter()
					.map(|x| x.to_str())
                    .collect::<Vec<&str>>()[..],
				)
                .help(&format!("Select the strategy. A strategy defines when to stop for the user to interact and which intermediate steps are shown.\nDefault is '{}'.\n", StrategySelection::AllSteps.to_str())[..]),
        )
        .arg(
            Arg::with_name("interaction")
                .long("-interaction")
                .value_name("InterationMode")
                .possible_values(
                    &vec![
                        InteractionMode::NoInteraction,
                        InteractionMode::WaitUser,
                        InteractionMode::WaitOrTimeout(0.),
                    ]
					.iter()
					.map(|x| x.to_str())
                    .collect::<Vec<&str>>()[..],
				)
                .help(&format!("Select the Interaction Mode. WaitOrTimeout is not yet implemented.\nDefault is '{}'.\n", InteractionMode::NoInteraction.to_str())[..]),
        )
        .arg(
			Arg::with_name("stop-after-first")
                .long("-stop-after-first")
                .help("If enabled, the solver will not output any more steps as soon as a solution is found. Has no effect if the strategy is set to OnlyCount (it will still count all solutions)."),
        )
		.get_matches();

    let board_size = value_t!(matches, "size", usize).unwrap_or(DEFAULT_BOARD_SIZE);
    let solver = Solver::new(board_size);

    let param = StrategyParameters {
        stop_after_first_solution: matches.is_present("stop-after-first"),
    };

    let interaction =
        value_t!(matches, "interaction", InteractionMode).unwrap_or(InteractionMode::NoInteraction);

    let strategy_selection =
        value_t!(matches, "strategy", StrategySelection).unwrap_or(StrategySelection::AllSteps);
    match strategy_selection {
        StrategySelection::AllSteps => {
            let driver_selection = matches.value_of("driver").unwrap_or("Console");
            match driver_selection {
                "3D" => {
                    let mut driver = Driver3D::new(board_size);
                    driver.execute(solver, EachPartialStep::new(param), interaction);
                }
                "Console" | _ => {
                    let mut driver = ConsoleDriver::new(board_size);
                    driver.execute(solver, EachPartialStep::new(param), interaction);
                }
            }
        }
        StrategySelection::SolutionsWithSteps => {
            let driver_selection = matches.value_of("driver").unwrap_or("Console");
            match driver_selection {
                "3D" => {
                    let mut driver = Driver3D::new(board_size);
                    driver.execute(solver, EachSolutionPartialSteps::new(param), interaction);
                }
                "Console" | _ => {
                    let mut driver = ConsoleDriver::new(board_size);
                    driver.execute(solver, EachSolutionPartialSteps::new(param), interaction);
                }
            };
        }
        StrategySelection::OnlySolutions => {
            let driver_selection = matches.value_of("driver").unwrap_or("Console");
            match driver_selection {
                "3D" => {
                    let mut driver = Driver3D::new(board_size);
                    driver.execute(solver, EachSolution::new(param), interaction);
                }
                "Console" | _ => {
                    let mut driver = ConsoleDriver::new(board_size);
                    driver.execute(solver, EachSolution::new(param), interaction);
                }
            };
        }
        StrategySelection::OnlyCount => {
            let driver_selection = matches.value_of("driver").unwrap_or("Console");
            match driver_selection {
                "3D" => {
                    let mut driver = Driver3D::new(board_size);
                    driver.execute(solver, CountSolutions::new(param), interaction);
                }
                "Console" | _ => {
                    let mut driver = ConsoleDriver::new(board_size);
                    driver.execute(solver, CountSolutions::new(param), interaction);
                }
            };
        }
    };
}
