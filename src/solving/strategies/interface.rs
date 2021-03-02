use crate::settings;
use crate::solving::strategies;
use crate::states;

pub trait Strategy {
    fn search(
        &self,
        constraints: &Vec<settings::Constraint>,
        state: &states::State,
    ) -> Result<Vec<strategies::Deduction>, Box<dyn std::error::Error>>;
}
