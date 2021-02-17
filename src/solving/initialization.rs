use crate::settings;
use crate::states;

pub fn initialize(
    setting: &settings::PuzzleSetting,
) -> Result<states::State, Box<dyn std::error::Error>> {
    let tokensets = setting
        .tokensets
        .iter()
        .map(|tokenset| match tokenset {
            settings::TokenSet::Symbols { grid, candidates } => {
                let grid = &setting.grids[*grid];
                let candidates = (0..grid.columns)
                    .map(|_| {
                        (0..grid.rows)
                            .map(|_| states::CellState::Candidates(candidates.clone()))
                            .collect()
                    })
                    .collect();
                states::Tokenset::Symbols(candidates)
            }
        })
        .collect();
    let result = states::State { tokensets, };
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::initialize;
    use crate::settings;
    use crate::settings::presets;
    use crate::settings::presets::sudoku;
    use crate::shapes;
    use crate::states;

    #[test]
    fn test_nominal() -> Result<(), Box<dyn std::error::Error>> {
        let mut setting = settings::PuzzleSetting::default();
        let grid = setting.add_grid(4, 4);
        let regions = shapes::Region::grid_boxes(2, 2, 2, 2).collect();
        let givens = presets::givens(vec![
            "5...".to_string(),
            "..6.".to_string(),
            ".1..".to_string(),
            "....".to_string(),
        ]);
        sudoku::add_symbolset(&mut setting, grid, 4, regions, givens);

        let result = initialize(&setting)?;
        let tokensets = vec![states::Tokenset::Symbols(vec![
            vec![
                states::CellState::Candidates(vec!['1', '2', '3', '4']),
                states::CellState::Candidates(vec!['1', '2', '3', '4']),
                states::CellState::Candidates(vec!['1', '2', '3', '4']),
                states::CellState::Candidates(vec!['1', '2', '3', '4']),
            ],
            vec![
                states::CellState::Candidates(vec!['1', '2', '3', '4']),
                states::CellState::Candidates(vec!['1', '2', '3', '4']),
                states::CellState::Candidates(vec!['1', '2', '3', '4']),
                states::CellState::Candidates(vec!['1', '2', '3', '4']),
            ],
            vec![
                states::CellState::Candidates(vec!['1', '2', '3', '4']),
                states::CellState::Candidates(vec!['1', '2', '3', '4']),
                states::CellState::Candidates(vec!['1', '2', '3', '4']),
                states::CellState::Candidates(vec!['1', '2', '3', '4']),
            ],
            vec![
                states::CellState::Candidates(vec!['1', '2', '3', '4']),
                states::CellState::Candidates(vec!['1', '2', '3', '4']),
                states::CellState::Candidates(vec!['1', '2', '3', '4']),
                states::CellState::Candidates(vec!['1', '2', '3', '4']),
            ],
        ])];
        let repartition = [('1', 1), ('2', 1), ('3', 1), ('4', 1)]
            .iter()
            .cloned()
            .collect::<std::collections::HashMap<char, usize>>();
        let expected = states::State { tokensets };
        assert_eq!(result, expected);
        Ok(())
    }
}
