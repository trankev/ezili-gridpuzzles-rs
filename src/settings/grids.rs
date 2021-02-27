#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Grid {
    pub rows: usize,
    pub columns: usize,
}
