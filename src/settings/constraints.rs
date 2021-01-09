use crate::settings;
use crate::shapes;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct GivenSymbol {
    pub symbol: char,
    pub cell: shapes::Cell,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(tag="type", rename_all="snake_case")]
pub enum Constraint{
    SudokuConstraints{
        symbolset: settings::TokenSetIndex,
        regions: Vec<shapes::Region>,
        givens: Vec<GivenSymbol>,
    },
}
