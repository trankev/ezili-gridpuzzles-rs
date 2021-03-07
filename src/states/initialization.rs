use crate::settings;
use crate::states;

pub enum Purpose {
    Playing,
    Solving,
}

pub fn initialize(
    setting: &settings::PuzzleSetting,
    purpose: Purpose,
) -> Result<states::State, Box<dyn std::error::Error>> {
    let tokensets = setting
        .tokensets
        .iter()
        .map(|tokenset| match tokenset {
            settings::TokenSet::Symbols { grid, candidates } => {
                let grid = &setting.grids[*grid];
                initialize_symbolset(grid, candidates.to_string(), &purpose)
            }
        })
        .collect();
    let mut result = states::State { tokensets };
    let constraints = settings::list_constraints(setting)?;
    states::apply_constraints(&mut result, &constraints);
    Ok(result)
}

fn initialize_symbolset(
    grid: &settings::Grid,
    candidates: String,
    purpose: &Purpose,
) -> states::Tokenset {
    let default = match purpose {
        Purpose::Playing => states::CellState::Empty,
        Purpose::Solving => states::CellState::Candidates(candidates.chars().collect()),
    };
    let candidates = (0..grid.columns)
        .map(|_| (0..grid.rows).map(|_| default.clone()).collect())
        .collect();
    states::Tokenset::Symbols(candidates)
}

#[cfg(test)]
mod tests {
    use super::initialize;
    use super::Purpose;
    use crate::settings;
    use crate::settings::presets;
    use crate::settings::presets::sudoku;
    use crate::shapes;
    use crate::states;

    #[test]
    fn test_nominal_solving() -> Result<(), Box<dyn std::error::Error>> {
        let mut setting = settings::PuzzleSetting::default();
        let grid = setting.add_grid(4, 4);
        let regions = shapes::Region::grid_boxes(2, 2, 2, 2).collect();
        let givens = presets::givens(&[
            "5...".to_string(),
            "..6.".to_string(),
            ".1..".to_string(),
            "....".to_string(),
        ]);
        sudoku::add_symbolset(&mut setting, grid, 4, regions, givens);

        let result = initialize(&setting, Purpose::Solving)?;
        let candidates = states::CellState::Candidates("1234".chars().collect());
        let tokensets = vec![states::Tokenset::Symbols(vec![
            vec![
                states::CellState::Set('5'),
                candidates.clone(),
                candidates.clone(),
                candidates.clone(),
            ],
            vec![
                candidates.clone(),
                candidates.clone(),
                states::CellState::Set('6'),
                candidates.clone(),
            ],
            vec![
                candidates.clone(),
                states::CellState::Set('1'),
                candidates.clone(),
                candidates.clone(),
            ],
            vec![
                candidates.clone(),
                candidates.clone(),
                candidates.clone(),
                candidates.clone(),
            ],
        ])];
        let expected = states::State { tokensets };
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_nominal_playing() -> Result<(), Box<dyn std::error::Error>> {
        let mut setting = settings::PuzzleSetting::default();
        let grid = setting.add_grid(4, 4);
        let regions = shapes::Region::grid_boxes(2, 2, 2, 2).collect();
        let givens = presets::givens(&[
            "5...".to_string(),
            "..6.".to_string(),
            ".1..".to_string(),
            "....".to_string(),
        ]);
        sudoku::add_symbolset(&mut setting, grid, 4, regions, givens);

        let result = initialize(&setting, Purpose::Playing)?;
        let tokensets = vec![states::Tokenset::Symbols(vec![
            vec![
                states::CellState::Set('5'),
                states::CellState::Empty,
                states::CellState::Empty,
                states::CellState::Empty,
            ],
            vec![
                states::CellState::Empty,
                states::CellState::Empty,
                states::CellState::Set('6'),
                states::CellState::Empty,
            ],
            vec![
                states::CellState::Empty,
                states::CellState::Set('1'),
                states::CellState::Empty,
                states::CellState::Empty,
            ],
            vec![
                states::CellState::Empty,
                states::CellState::Empty,
                states::CellState::Empty,
                states::CellState::Empty,
            ],
        ])];
        let expected = states::State { tokensets };
        assert_eq!(result, expected);
        Ok(())
    }
}
