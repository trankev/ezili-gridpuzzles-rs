use super::types;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(tag="type", rename_all="snake_case")]
pub enum TokenSet {
    Symbols{grid: types::GridIndex, candidates: Vec<char>},
}
