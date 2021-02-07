use crate::settings;
use crate::shapes;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Constraint {
    SymbolRepartition {
        tokenset: settings::TokenSetIndex,
        repartition: std::collections::HashMap<settings::SymbolType, usize>,
        region: shapes::Region,
    },
    GivenSymbol {
        tokenset: settings::TokenSetIndex,
        cell: shapes::Cell,
        symbol: settings::SymbolType,
    },
}
