#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum CellState {
    Set(char),
    Candidates(Vec<char>),
    Empty,
}

#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Tokenset {
    Symbols(Vec<Vec<CellState>>),
}

#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct State {
    pub tokensets: Vec<Tokenset>,
}

pub fn symbolset(grid: Vec<String>, default: CellState) -> Tokenset {
    let cells = grid.iter().map(|row_contents| row_contents.chars().map(|symbol| match symbol {
            '.' | ' ' => default.clone(),
            _ => CellState::Set(symbol),
        }).collect::<Vec<_>>()
    ).collect::<Vec<_>>();
    Tokenset::Symbols(cells)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symbolset() {
        let grid = vec![
            "12..".to_string(),
            ".1..".to_string(),
            "....".to_string(),
            "4567".to_string(),
        ];
        let result = symbolset(grid, CellState::Empty);
        let expected_grid = vec![
            vec![
                CellState::Set('1'),
                CellState::Set('2'),
                CellState::Empty,
                CellState::Empty,
            ],
            vec![
                CellState::Empty,
                CellState::Set('1'),
                CellState::Empty,
                CellState::Empty,
            ],
            vec![
                CellState::Empty,
                CellState::Empty,
                CellState::Empty,
                CellState::Empty,
            ],
            vec![
                CellState::Set('4'),
                CellState::Set('5'),
                CellState::Set('6'),
                CellState::Set('7'),
            ],
        ];
        let expected = Tokenset::Symbols(expected_grid);
        assert_eq!(result, expected);
    }
}
