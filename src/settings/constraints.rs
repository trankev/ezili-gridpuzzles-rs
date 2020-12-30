use crate::settings;
use crate::shapes;

#[derive(serde::Serialize)]
pub struct GivenSymbol {
    pub symbol: char,
    pub cell: shapes::Cell,
}

#[derive(serde::Serialize)]
#[serde(tag="type")]
pub enum Constraint{
    SudokuConstraints{
        symbolset: settings::TokenSetIndex,
        regions: Vec<shapes::Region>,
        givens: Vec<GivenSymbol>,
    },
}
