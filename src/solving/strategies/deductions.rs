use crate::settings;
use crate::shapes;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum Category {
    HiddenTuple,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum Action {
    SetSymbol {
        tokenset: settings::TokenSet,
        cell: shapes::Cell,
        symbol: settings::SymbolType,
    },
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Deduction {
    category: Category,
    actions: Vec<Action>,
}
