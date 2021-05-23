use crate::settings;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct PuzzleSetting {
    pub grids: Vec<settings::Grid>,
}

impl Default for PuzzleSetting {
    fn default() -> PuzzleSetting {
        PuzzleSetting {
            grids: Vec::new(),
        }
    }
}
