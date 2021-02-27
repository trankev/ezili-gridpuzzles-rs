use crate::settings;
use crate::shapes;
use crate::states;

pub fn apply_constraints(state: &mut states::State, constraints: &[settings::Constraint]) {
    states::apply_constraints(state, constraints);
    for constraint in constraints {
        match constraint {
            settings::Constraint::SymbolRepartition {
                tokenset,
                region,
                repartition,
            } => apply_symbol_repartition(state, *tokenset, region, repartition),
            _ => (),
        }
    }
}

fn apply_symbol_repartition(
    state: &mut states::State,
    tokenset: usize,
    region: &shapes::Region,
    repartition: &settings::SymbolRepartition,
) {
    let tokenset = &mut state.tokensets[tokenset];
    match tokenset {
        states::Tokenset::Symbols(cells) => {
            let mut found = settings::SymbolRepartition::new();
            for cell in &region.cells {
                let cell_state = &cells[cell.y as usize][cell.x as usize];
                if let states::CellState::Set(value) = cell_state {
                    found
                        .entry(*value)
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                }
            }
            let to_remove = repartition
                .iter()
                .filter_map(|(symbol, expected_count)| {
                    let count = found.get(symbol).unwrap_or(&0);
                    if count == expected_count {
                        Some(symbol)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            for cell in &region.cells {
                let cell_state = &mut cells[cell.y as usize][cell.x as usize];
                if let states::CellState::Candidates(candidates) = cell_state {
                    for value in &to_remove {
                        candidates.remove(value);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings;
    use crate::settings::presets;
    use crate::settings::presets::sudoku;
    use crate::shapes;
    use crate::states;

    fn sample_setting() -> settings::PuzzleSetting {
        let mut setting = settings::PuzzleSetting::default();
        let grid = setting.add_grid(4, 4);
        let regions = shapes::Region::grid_boxes(2, 2, 2, 2).collect();
        let givens = presets::givens(&[
            "2...".to_string(),
            "..3.".to_string(),
            ".1..".to_string(),
            "....".to_string(),
        ]);
        sudoku::add_symbolset(&mut setting, grid, 4, regions, givens);
        setting
    }

    fn candidates(symbols: &str) -> states::CellState {
        states::CellState::Candidates(symbols.chars().collect())
    }

    #[test]
    fn test_start_position() -> Result<(), Box<dyn std::error::Error>> {
        let setting = sample_setting();
        let constraints = settings::list_constraints(&setting)?;
        let mut state = states::State {
            tokensets: vec![states::symbolset(
                &[
                    "2...".to_string(),
                    "..3.".to_string(),
                    ".1..".to_string(),
                    "....".to_string(),
                ],
                candidates("1234"),
            )],
        };
        apply_constraints(&mut state, &constraints);
        let tokensets = vec![states::Tokenset::Symbols(vec![
            vec![
                states::CellState::Set('2'),
                candidates("34"),
                candidates("14"),
                candidates("14"),
            ],
            vec![
                candidates("14"),
                candidates("4"),
                states::CellState::Set('3'),
                candidates("124"),
            ],
            vec![
                candidates("34"),
                states::CellState::Set('1'),
                candidates("24"),
                candidates("234"),
            ],
            vec![
                candidates("34"),
                candidates("234"),
                candidates("124"),
                candidates("1234"),
            ],
        ])];
        let expected = states::State { tokensets };
        assert_eq!(state, expected);
        Ok(())
    }
}
