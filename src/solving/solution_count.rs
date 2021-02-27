use crate::settings;
use crate::shapes;
use crate::states;

enum Iterator {
    Symbolset {
        tokenset: usize,
        cell: shapes::Cell,
        candidates: std::collections::HashSet<char>,
    },
    Unset,
}

pub fn count_solutions(
    state: &mut states::State,
    constraints: &[settings::Constraint],
) -> Result<usize, Box<dyn std::error::Error>> {
    let mut result = 0;
    match states::compute_status(state, constraints)? {
        states::Status::Valid => result += 1,
        states::Status::Invalid => return Ok(0),
        states::Status::Ongoing => (),
    }
    let iterator = find_pivot(&state);
    match iterator {
        Iterator::Symbolset {
            tokenset,
            cell,
            candidates,
        } => {
            for candidate in &candidates {
                let tokenset = &mut state.tokensets[tokenset];
                let states::Tokenset::Symbols(cells) = tokenset;
                cells[cell.y as usize][cell.x as usize] = states::CellState::Set(*candidate);
                result += count_solutions(state, constraints)?;
            }
            let tokenset = &mut state.tokensets[tokenset];
            let states::Tokenset::Symbols(cells) = tokenset;
            cells[cell.y as usize][cell.x as usize] = states::CellState::Candidates(candidates);
        }
        Iterator::Unset => (),
    }
    Ok(result)
}

fn find_pivot(state: &states::State) -> Iterator {
    for (index, tokenset) in state.tokensets.iter().enumerate() {
        match tokenset {
            states::Tokenset::Symbols(cells) => {
                for row in 0..cells.len() {
                    for cell in 0..cells[row].len() {
                        if let states::CellState::Candidates(candidates) = &cells[row][cell] {
                            let cell = shapes::Cell {
                                x: cell as isize,
                                y: row as isize,
                            };
                            return Iterator::Symbolset {
                                tokenset: index,
                                cell,
                                candidates: candidates.clone(),
                            };
                        }
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
    use crate::settings;
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
        let constraints = settings::list_constraints(&setting)?;
        let mut state = states::State {
            tokensets: vec![states::symbolset(
                &grid,
                states::CellState::Candidates("1234".chars().collect()),
            )],
        };
        let result = count_solutions(&mut state, &constraints)?;
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
        let constraints = settings::list_constraints(&setting)?;
        let mut state = states::State {
            tokensets: vec![states::symbolset(
                &grid,
                states::CellState::Candidates("1234".chars().collect()),
            )],
        };
        let starting_state = state.clone();
        let result = count_solutions(&mut state, &constraints)?;
        assert_eq!(result, 1);
        assert_eq!(state, starting_state);
        Ok(())
    }
}
