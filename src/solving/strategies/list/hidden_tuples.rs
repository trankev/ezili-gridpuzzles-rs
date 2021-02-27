use crate::settings;
use crate::shapes;
use crate::solving;
use crate::solving::strategies;
use crate::states;

pub struct HiddenTuples {
    min_size: usize,
    max_size: usize,
}

impl strategies::Strategy for HiddenTuples {
    fn search(
        &self,
        setting: &settings::PuzzleSetting,
        constraints: &Vec<settings::Constraint>,
        state: &states::State,
    ) -> Vec<strategies::Deduction> {
        let mut result = Vec::new();
        for constraint in constraints {
            if let settings::Constraint::SymbolRepartition {
                tokenset,
                repartition,
                region,
            } = constraint
            {
                let states::Tokenset::Symbols(cells) = &state.tokensets[*tokenset];
                result.extend(self.search_for_constraint(repartition, region, cells));
            }
        }
        result
    }
}

impl HiddenTuples {
    fn search_for_constraint(
        &self,
        repartition: &settings::SymbolRepartition,
        region: &shapes::Region,
        cells: &states::CellGrid,
    ) -> Vec<strategies::Deduction> {
        let mut result = Vec::new();
        let (set_positions, candidate_positions) = solving::symbol_positions(cells, region);
        result
    }
}
