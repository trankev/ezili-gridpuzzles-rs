#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
pub struct Cell {
    pub x: usize,
    pub y: usize,
}
