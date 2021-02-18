use crate::settings;
use crate::shapes;

pub type SymbolRepartition = std::collections::HashMap<settings::SymbolType, usize>;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Constraint {
    SymbolRepartition {
        tokenset: settings::TokenSetIndex,
        repartition: SymbolRepartition,
        region: shapes::Region,
    },
    GivenSymbol {
        tokenset: settings::TokenSetIndex,
        cell: shapes::Cell,
        symbol: settings::SymbolType,
    },
}
