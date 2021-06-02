use crate::search::state;
use crate::settings;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct SolveState {
    grids: Vec<state::Grid>,
}

impl SolveState {
    pub fn from_settings(settings: &settings::PuzzleSetting) -> SolveState {
        let grids = settings.grids
            .iter()
            .map(|grid| state::Grid::from_settings(grid))
            .collect();
        SolveState{grids}
    }
}
