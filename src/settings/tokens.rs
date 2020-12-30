use super::types;

#[derive(serde::Serialize)]
#[serde(tag="type")]
pub enum TokenSet {
    Symbols{grid: types::GridIndex, candidates: Vec<char>},
}
