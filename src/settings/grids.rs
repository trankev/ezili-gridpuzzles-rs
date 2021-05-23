use crate::settings;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Grid {
    pub rows: usize,
    pub columns: usize,
    pub rules: Vec<settings::Rule>,
}
