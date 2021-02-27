use super::types;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TokenSet {
    Symbols {
        grid: types::GridIndex,
        candidates: Vec<types::SymbolType>,
    },
}
