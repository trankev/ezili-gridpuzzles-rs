use crate::settings;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct PuzzleSetting {
    pub grids: Vec<settings::Grid>,
    pub tokensets: Vec<settings::TokenSet>,
    pub rules: Vec<settings::Rule>,
}

impl PuzzleSetting {
    pub fn add_grid(&mut self, rows: usize, columns: usize) -> settings::GridIndex {
        let index = self.grids.len() as settings::GridIndex;
        self.grids.push(settings::Grid { rows, columns });
        index
    }
    pub fn add_symbolset(&mut self, tokenset: settings::TokenSet) -> settings::TokenSetIndex {
        let index = self.tokensets.len() as settings::TokenSetIndex;
        self.tokensets.push(tokenset);
        index
    }
}

impl Default for PuzzleSetting {
    fn default() -> PuzzleSetting {
        PuzzleSetting {
            grids: Vec::new(),
            tokensets: Vec::new(),
            rules: Vec::new(),
        }
    }
}
