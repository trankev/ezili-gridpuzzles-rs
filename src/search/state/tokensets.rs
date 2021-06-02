use crate::settings;
use crate::shapes;
use crate::types;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CellState {
    Set(types::SymbolType),
    Candidates(String)
}

type Grid = Vec<Vec<CellState>>;

impl std::ops::Index<&shapes::Cell> for Grid {
    type Output = CellState;

    fn index(&self, cell: &shapes::Cell) -> &Self::Output {
        &self[cell.0 as usize][cell.1 as usize]
    }
}

impl std::ops::IndexMut<&shapes::Cell> for Grid {
    fn index_mut(&mut self, cell: &shapes::Cell) -> &mut Self::Output {
        &mut self[cell.0 as usize][cell.1 as usize]
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TokenSet {
    Symbolset{symbols: Grid},
}

impl TokenSet {
    pub fn from_rule(grid: &settings::Grid, rule: &settings::Rule) -> TokenSet {
        let candidates = (0..grid.rows)
            .map(|digit| digit.to_string())
            .collect::<Vec<_>>().join("");
        match rule {
            settings::Rule::Sudoku{region_config: _, givens, variants: _} => {
                let mut symbols = (0..grid.columns)
                    .map(|_| (0..grid.rows)
                        .map(|_| CellState::Candidates(candidates.clone()))
                        .collect()
                    )
                    .collect::<Grid>();
                for given in givens {
                    symbols[&given.cell] = CellState::Set(given.symbol);
                }
                TokenSet::Symbolset{symbols}
            }
        }
    }
}
