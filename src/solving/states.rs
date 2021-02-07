use crate::settings;

#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum CellState {
    Set(char),
    Candidates(Vec<char>),
}

#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Tokenset {
    Symbols(Vec<Vec<CellState>>),
}

#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct State {
    pub tokensets: Vec<Tokenset>,
    pub constraints: Vec<settings::Constraint>,
}
