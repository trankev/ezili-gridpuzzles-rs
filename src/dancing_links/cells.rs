#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Cell(pub usize);

impl<Item> std::ops::Index<Cell> for Vec<Item> {
    type Output = Item;
    fn index(&self, index: Cell) -> &Item {
        &self[index.0]
    }
}

impl<Item> std::ops::IndexMut<Cell> for Vec<Item> {
    fn index_mut(&mut self, index: Cell) -> &mut Item {
        &mut self[index.0]
    }
}
