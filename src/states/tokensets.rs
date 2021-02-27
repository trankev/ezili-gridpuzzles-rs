use crate::settings;
use crate::shapes;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum CellState {
    Set(settings::SymbolType),
    Candidates(std::collections::HashSet<settings::SymbolType>),
    Empty,
}

type CellGrid = Vec<Vec<CellState>>;

impl std::ops::Index<&shapes::Cell> for CellGrid {
    type Output = CellState;

    fn index(&self, cell: &shapes::Cell) -> &Self::Output {
        &self[cell.y][cell.x]
    }
}

impl std::ops::IndexMut<&shapes::Cell> for CellGrid {
    fn index_mut(&mut self, cell: &shapes::Cell) -> &mut Self::Output {
        &mut self[cell.y][cell.x]
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Tokenset {
    Symbols(CellGrid),
}

pub fn symbolset(grid: &[String], default: CellState) -> Tokenset {
    let cells = grid
        .iter()
        .map(|row_contents| {
            row_contents
                .chars()
                .map(|symbol| match symbol {
                    '.' | ' ' => default.clone(),
                    _ => CellState::Set(symbol),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    Tokenset::Symbols(cells)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symbolset() {
        let grid = [
            "12..".to_string(),
            ".1..".to_string(),
            "....".to_string(),
            "4567".to_string(),
        ];
        let result = symbolset(&grid, CellState::Empty);
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
