use crate::types;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CellState {
    Set(types::SymbolType),
    Candidates(String)
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Tokenset {
    Symbolset{symbols: Vec<Vec<CellState>>},
}
