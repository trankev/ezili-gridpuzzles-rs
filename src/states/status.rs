use crate::settings;
use crate::shapes;
use crate::states;

#[derive(Debug, PartialEq)]
pub enum Status {
    Valid,
    Ongoing,
    Invalid,
}

type ResultStatus = Result<Status, Box<dyn std::error::Error>>;

pub fn compute_status(state: &states::State, constraints: &[settings::Constraint]) -> ResultStatus {
    let mut complete = true;
    for constraint in constraints {
        let status = match constraint {
            settings::Constraint::GivenSymbol {
                tokenset,
                cell,
                symbol,
            } => check_given_symbol(state, *tokenset, cell, *symbol)?,
            settings::Constraint::SymbolRepartition {
                tokenset,
                region,
                repartition,
            } => check_symbol_repartition(state, *tokenset, region, repartition)?,
        };
        match status {
            Status::Ongoing => complete = false,
            Status::Invalid => return Ok(Status::Invalid),
            Status::Valid => (),
        }
    }
    Ok(if complete {
        Status::Valid
    } else {
        Status::Ongoing
    })
}

fn check_given_symbol(
    state: &states::State,
    tokenset: usize,
    cell: &shapes::Cell,
    symbol: char,
) -> ResultStatus {
    let tokenset = &state.tokensets[tokenset];
    match tokenset {
        states::Tokenset::Symbols(cells) => {
            let cell_state = &cells[cell.y as usize][cell.x as usize];
            match cell_state {
                states::CellState::Set(value) => {
                    if *value == symbol {
                        Ok(Status::Valid)
                    } else {
                        Ok(Status::Invalid)
                    }
                }
                _ => Ok(Status::Ongoing),
            }
        }
    }
}

fn check_symbol_repartition(
    state: &states::State,
    tokenset: usize,
    region: &shapes::Region,
    repartition: &settings::SymbolRepartition,
) -> ResultStatus {
    let tokenset = &state.tokensets[tokenset];
    match tokenset {
        states::Tokenset::Symbols(cells) => {
            let mut found = settings::SymbolRepartition::new();
            let mut completed = true;
            for cell in &region.cells {
                let cell_state = &cells[cell.y as usize][cell.x as usize];
                if let states::CellState::Set(value) = cell_state {
                    found
                        .entry(*value)
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                }
            }
            for (symbol, expected_count) in repartition.iter() {
                let count = found.get(symbol).unwrap_or(&0);
                if count > expected_count {
                    return Ok(Status::Invalid);
                }
                if count < expected_count {
                    completed = false;
                }
            }
            Ok(if completed {
                Status::Valid
            } else {
                Status::Ongoing
            })
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
        let givens = presets::givens(vec![
            "2...".to_string(),
            "..3.".to_string(),
            ".1..".to_string(),
            "....".to_string(),
        ]);
        sudoku::add_symbolset(&mut setting, grid, 4, regions, givens);
        setting
    }

    #[test]
    fn test_start_position() -> Result<(), Box<dyn std::error::Error>> {
        let setting = sample_setting();
        let constraints = settings::list_constraints(&setting)?;
        let state = states::State {
            tokensets: vec![states::symbolset(
                &[
                    "2...".to_string(),
                    "..3.".to_string(),
                    ".1..".to_string(),
                    "....".to_string(),
                ],
                states::CellState::Empty,
            )],
        };
        let result = compute_status(&state, &constraints)?;
        assert_eq!(result, Status::Ongoing);
        Ok(())
    }

    #[test]
    fn test_invalid_given() -> Result<(), Box<dyn std::error::Error>> {
        let setting = sample_setting();
        let constraints = settings::list_constraints(&setting)?;
        let state = states::State {
            tokensets: vec![states::symbolset(
                &[
                    "2...".to_string(),
                    "..4.".to_string(),
                    ".1..".to_string(),
                    "....".to_string(),
                ],
                states::CellState::Empty,
            )],
        };
        let result = compute_status(&state, &constraints)?;
        assert_eq!(result, Status::Invalid);
        Ok(())
    }

    #[test]
    fn test_repartition() -> Result<(), Box<dyn std::error::Error>> {
        let setting = sample_setting();
        let constraints = settings::list_constraints(&setting)?;
        let state = states::State {
            tokensets: vec![states::symbolset(
                &[
                    "2...".to_string(),
                    "..4.".to_string(),
                    ".1..".to_string(),
                    "1...".to_string(),
                ],
                states::CellState::Empty,
            )],
        };
        let result = compute_status(&state, &constraints)?;
        assert_eq!(result, Status::Invalid);
        Ok(())
    }

    #[test]
    fn test_finished() -> Result<(), Box<dyn std::error::Error>> {
        let setting = sample_setting();
        let constraints = settings::list_constraints(&setting)?;
        let state = states::State {
            tokensets: vec![states::symbolset(
                &[
                    "2314".to_string(),
                    "1432".to_string(),
                    "4123".to_string(),
                    "3241".to_string(),
                ],
                states::CellState::Empty,
            )],
        };
        let result = compute_status(&state, &constraints)?;
        assert_eq!(result, Status::Valid);
        Ok(())
    }
}
