#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
pub struct Cell {
    pub x: isize,
    pub y: isize,
}
