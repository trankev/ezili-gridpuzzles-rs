use crate::search::state;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct SolveState {
    grids: Vec<state::Grid>,
}
