use std::str::FromStr;

use crate::solver::Solver;
use crate::strategy::Strategy;

pub enum StrategySelection {
    AllSteps,
    SolutionsWithSteps,
    OnlySolutions,
    OnlyCount,
}

pub enum InteractionMode {
    NoInteraction,
    WaitUser,
    WaitOrTimeout(f32),
}

impl StrategySelection {
    pub fn to_str(&self) -> &'static str {
        match self {
            StrategySelection::AllSteps => "AllSteps",
            StrategySelection::SolutionsWithSteps => "SolutionsWithSteps",
            StrategySelection::OnlySolutions => "OnlySolutions",
            StrategySelection::OnlyCount => "OnlyCount",
        }
    }
}

impl FromStr for StrategySelection {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AllSteps" => Ok(StrategySelection::AllSteps),
            "SolutionsWithSteps" => Ok(StrategySelection::SolutionsWithSteps),
            "OnlySolutions" => Ok(StrategySelection::OnlySolutions),
            "OnlyCount" => Ok(StrategySelection::OnlyCount),
            _ => Err("no match"),
        }
    }
}

impl InteractionMode {
    pub fn to_str(&self) -> &'static str {
        match self {
            InteractionMode::NoInteraction => "NoInteraction",
            InteractionMode::WaitUser => "WaitUser",
            InteractionMode::WaitOrTimeout(_) => "WaitOrTimeout",
        }
    }
}

impl FromStr for InteractionMode {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NoInteraction" => Ok(InteractionMode::NoInteraction),
            "WaitUser" => Ok(InteractionMode::WaitUser),
            "WaitOrTimeout" => Ok(InteractionMode::WaitOrTimeout(0.)),
            _ => Err("no match"),
        }
    }
}

pub trait Driver<T>
where
    T: Strategy,
{
    fn execute(&mut self, solver: Solver, strategy: T, interaction_mode: InteractionMode);
}
