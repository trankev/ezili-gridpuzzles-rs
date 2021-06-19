use crate::dancing_links::cells::Cell;

#[derive(Debug)]
pub struct Link {
    pub prev: Cell,
    pub next: Cell,
}

#[derive(Debug)]
pub struct LinkedList {
    pub data: Vec<Link>,
}

impl std::ops::Index<Cell> for LinkedList {
    type Output = Link;

    fn index(&self, index: Cell) -> &Link {
        &self.data[index.0]
    }
}

impl std::ops::IndexMut<Cell> for LinkedList {
    fn index_mut(&mut self, index: Cell) -> &mut Link {
        &mut self.data[index.0]
    }
}

impl LinkedList {
    pub fn with_capacity(cap: usize) -> LinkedList {
        LinkedList {data: Vec::with_capacity(cap)}
    }

    pub fn alloc(&mut self) -> Cell {
        let cell = Cell(self.data.len());
        self.data.push(Link{prev: cell, next: cell});
        cell
    }

    /// Inserts `b` into `a <-> c` to get `a <-> b <-> c`
    pub fn insert(&mut self, a: Cell, b: Cell) {
        let c = self[a].next;
        self[b].prev = a;
        self[b].next = c;

        self[a].next = b;
        self[c].prev = b;
    }

    /// Removes `b` from `a <-> b <-> c` to get `a <-> c`
    pub fn remove(&mut self, b: Cell) {
        let a = self[b].prev;
        let c = self[b].next;

        self[a].next = self[b].next;
        self[c].prev = self[b].prev;
    }

    /// Restores previously removed `b` to get `a <-> b <-> c`
    pub fn restore(&mut self, b: Cell) {
        let a = self[b].prev;
        let c = self[b].next;
        self[a].next = b;
        self[c].prev = b;
    }
}
