use crate::dancing_links::links::LinkedList;
use crate::dancing_links::cells::Cell;

pub struct Cursor {
    head: Cell,
    curr: Cell,
}

impl Cursor {
    pub fn from_head(head: Cell) -> Cursor {
        Cursor {head, curr: head}
    }

    pub fn next(&mut self, list: &LinkedList) -> Option<Cell> {
        self.curr = list[self.curr].next;
        if self.curr == self.head {
        return None;
        }
        Some(self.curr)
    }

    pub fn prev(&mut self, list: &LinkedList) -> Option<Cell> {
        self.curr = list[self.curr].prev;
        if self.curr == self.head {
        return None;
        }
        Some(self.curr)
    }
}
