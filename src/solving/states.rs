#[derive(serde::Deserialize, serde::Serialize)]
pub enum CellState {
    Set(char),
    Candidates(Vec<char>),
}

#[derive(serde::Deserialize, serde::Serialize)]
pub enum Tokenset {
    Symbols(Vec<Vec<CellState>>),
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct State {
    pub tokensets: Vec<Tokenset>,
}
