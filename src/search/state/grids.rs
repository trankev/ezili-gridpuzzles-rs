use crate::search::state;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Grid {
    tokensets: Vec<state::Tokenset>,
}
