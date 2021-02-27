use crate::settings;
use crate::shapes;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct GivenSymbol {
    pub symbol: settings::SymbolType,
    pub cell: shapes::Cell,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Rule {
    Sudoku {
        tokenset: settings::TokenSetIndex,
        regions: Vec<shapes::Region>,
        givens: Vec<GivenSymbol>,
    },
}
