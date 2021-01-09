use crate::settings;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Rules {
    pub grids: Vec<settings::Grid>,
    pub tokensets: Vec<settings::TokenSet>,
    pub constraints: Vec<settings::Constraint>,
}

impl Rules {
    pub fn add_grid(&mut self, rows: usize, columns: usize) -> settings::GridIndex {
        let index = self.grids.len() as settings::GridIndex;
        self.grids.push(settings::Grid{rows, columns});
        index
    }
    pub fn add_symbolset(&mut self, tokenset: settings::TokenSet ) -> settings::TokenSetIndex {
        let index = self.tokensets.len() as settings::TokenSetIndex;
        self.tokensets.push(tokenset);
        index
    }
}

impl Default for Rules {
    fn default() -> Rules {
        Rules {
            grids: Vec::new(),
            tokensets: Vec::new(),
            constraints: Vec::new(),
        }
    }
}
