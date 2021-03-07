use crate::settings;
use crate::shapes;
use crate::states;

enum Iterator {
    Symbolset {
        tokenset: usize,
        cell: shapes::Cell,
        candidates: String,
    },
    Unset,
}

pub fn count_solutions(
    setting: &settings::PuzzleSetting,
    state: &mut states::State,
) -> Result<usize, Box<dyn std::error::Error>> {
    let constraints = settings::list_constraints(&setting)?;
    do_count_solutions(setting, state, &constraints)
}

fn do_count_solutions(
    setting: &settings::PuzzleSetting,
    state: &mut states::State,
    constraints: &[settings::Constraint],
) -> Result<usize, Box<dyn std::error::Error>> {
    let mut result = 0;
    match states::compute_status(state, constraints)? {
        states::Status::Valid => result += 1,
        states::Status::Invalid => return Ok(0),
        states::Status::Ongoing => (),
    }
    let iterator = find_pivot(&setting, &state);
    match iterator {
        Iterator::Symbolset {
            tokenset,
            cell,
            candidates,
        } => {
            let previous = {
                let tokenset = &mut state.tokensets[tokenset];
                let states::Tokenset::Symbols(cells) = tokenset;
                cells[&cell].clone()
            };
            for candidate in candidates.chars() {
                let tokenset = &mut state.tokensets[tokenset];
                let states::Tokenset::Symbols(cells) = tokenset;
                cells[&cell] = states::CellState::Set(candidate);
                result += do_count_solutions(setting, state, constraints)?;
            }
            let tokenset = &mut state.tokensets[tokenset];
            let states::Tokenset::Symbols(cells) = tokenset;
            cells[&cell] = previous;
        }
        Iterator::Unset => (),
    }
    Ok(result)
}

fn find_pivot(setting: &settings::PuzzleSetting, state: &states::State) -> Iterator {
    for (index, tokenset) in state.tokensets.iter().enumerate() {
        match tokenset {
            states::Tokenset::Symbols(cells) => {
                for row in 0..cells.len() {
                    for column in 0..cells[row].len() {
                        let cell = shapes::Cell(column, row);
                        if let states::CellState::Set(_) = &cells[&cell] {
                            continue;
                        }
                        let setting_tokenset = &setting.tokensets[index];
                        let candidates = match setting_tokenset {
                            settings::TokenSet::Symbols {
                                grid: _,
                                candidates,
                            } => candidates.clone(),
                        };
                        return Iterator::Symbolset {
                            tokenset: index,
                            cell,
                            candidates,
                        };
                    }
                }
            }
        }
    }
    Iterator::Unset
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::presets::sudoku;
    use crate::states;

    #[test]
    fn test_finished_position() -> Result<(), Box<dyn std::error::Error>> {
        let grid = vec![
            "2...".to_string(),
            "..3.".to_string(),
            ".1..".to_string(),
            "...4".to_string(),
        ];
        let setting = sudoku::setting(2, &grid);
        let mut state = states::State {
            tokensets: vec![states::symbolset(
                &grid,
                states::CellState::Candidates("1234".chars().collect()),
            )],
        };
        let result = count_solutions(&setting, &mut state)?;
        assert_eq!(result, 1);
        Ok(())
    }

    #[test]
    fn test_start_position() -> Result<(), Box<dyn std::error::Error>> {
        let grid = vec![
            "2...".to_string(),
            "..3.".to_string(),
            ".1..".to_string(),
            "...4".to_string(),
        ];
        let setting = sudoku::setting(2, &grid);
        let mut state = states::State {
            tokensets: vec![states::symbolset(&grid, states::CellState::Empty)],
        };
        let starting_state = state.clone();
        let result = count_solutions(&setting, &mut state)?;
        assert_eq!(result, 1);
        assert_eq!(state, starting_state);
        Ok(())
    }
}
