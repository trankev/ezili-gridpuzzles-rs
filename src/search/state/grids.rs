use crate::search::state;
use crate::settings;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Grid {
    tokensets: Vec<state::TokenSet>,
}

impl Grid {
    pub fn from_settings(grid: &settings::Grid) -> Grid {
        let tokensets = grid.rules.iter()
            .map(|rule| state::TokenSet::from_rule(grid, rule))
            .collect();
        Grid{tokensets}
    }
}
