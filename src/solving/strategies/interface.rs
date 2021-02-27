use crate::settings;
use crate::solving::strategies;
use crate::states;

pub trait Strategy {
    fn search(
        &self,
        setting: &settings::PuzzleSetting,
        constraints: &Vec<settings::Constraint>,
        state: &states::State,
    ) -> Vec<strategies::Deduction>;
}
