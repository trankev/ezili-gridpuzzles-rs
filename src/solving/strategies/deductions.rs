use crate::settings;
use crate::shapes;

#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Category {
    HiddenTuple {
        symbols: Vec<settings::SymbolType>,
        region: shapes::Region,
        positions: Vec<shapes::Cell>,
    },
}

#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Action {
    SetSymbol {
        tokenset: settings::TokenSetIndex,
        cell: shapes::Cell,
        symbol: settings::SymbolType,
    },
    RemoveSymbolCandidate {
        tokenset: settings::TokenSetIndex,
        cell: shapes::Cell,
        symbol: settings::SymbolType,
    },
}

#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Deduction {
    pub category: Category,
    pub actions: Vec<Action>,
}
