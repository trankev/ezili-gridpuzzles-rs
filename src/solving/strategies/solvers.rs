use crate::settings;
use crate::solving::strategies;
use crate::solving::strategies::list;
use crate::states;

pub struct Solver {
    strategies: Vec<Box<dyn strategies::Strategy>>,
}

impl Default for Solver {
    fn default() -> Solver {
        Solver {
            strategies: vec![
                Box::new(list::HiddenTuples::with_fixed_size(1)),
                Box::new(list::HiddenTuples::with_fixed_size(2)),
                Box::new(list::HiddenTuples::with_fixed_size(3)),
                Box::new(list::HiddenTuples::with_min_size(4)),
            ],
        }
    }
}

impl Solver {
    pub fn iterate(
        &self,
        constraints: &Vec<settings::Constraint>,
        state: &states::State,
    ) -> Result<Vec<strategies::Deduction>, Box<dyn std::error::Error>> {
        for strategy in &self.strategies {
            let deductions = strategy.search(constraints, state)?;
            if !deductions.is_empty() {
                return Ok(deductions);
            }
        }
        return Ok(Vec::new());
    }
}
