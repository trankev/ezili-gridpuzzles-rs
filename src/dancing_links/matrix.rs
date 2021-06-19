use crate::dancing_links::cursors::Cursor;
use crate::dancing_links::links::LinkedList;
use std::collections::HashMap;

use crate::dancing_links::cells::Cell;

pub const HEAD: Cell = Cell(0);

#[derive(Debug)]
pub struct Matrix
{
    pub hlinks: LinkedList,
    pub vlinks: LinkedList,
    pub headers: Vec<Cell>,
    pub sizes: Vec<usize>,
}

impl Matrix {
    pub fn new(n_cols: usize) -> Matrix {
        let mut res = Matrix {
            hlinks: LinkedList::with_capacity(n_cols + 1),
            vlinks: LinkedList::with_capacity(n_cols + 1),
            headers: Vec::with_capacity(n_cols + 1),
            sizes: Vec::with_capacity(n_cols + 1),
        };
        assert_eq!(res.alloc_column(), HEAD);
        for _ in 0..n_cols {
            res.add_column();
        }
        res
    }

    fn add_column(&mut self) {
        let new_col = self.alloc_column();
        self.vlinks.insert(self.vlinks[HEAD].prev, new_col);
    }

    fn alloc_column(&mut self) -> Cell {
        let cell = self.alloc(HEAD);
        self.headers[cell] = cell;
        self.sizes.push(0);
        cell
    }

    fn alloc(&mut self, c: Cell) -> Cell {
        self.headers.push(c);
        let cell = self.vlinks.alloc();
        assert_eq!(self.hlinks.alloc(), cell);
        cell
    }

    pub fn add_row(&mut self, row: &[bool]) {
        assert_eq!(row.len(), self.sizes.len() - 1);
        let mut cell = HEAD;
        let mut prev = None;
        for &is_filled in row {
            cell = self.vlinks[cell].next;
            if is_filled {
                self.sizes[cell] += 1;
                let new_cell = self.alloc(cell);
                self.hlinks.insert(self.hlinks[cell].prev, new_cell);
                if let Some(prev) = prev {
                    self.vlinks.insert(prev, new_cell);
                }
                prev = Some(new_cell);
            }
        }
    }

    pub fn cover(&mut self, cell: Cell) {
        self.vlinks.remove(cell);
        let mut i = Cursor::from_head(cell);
        while let Some(i) = i.next(&self.hlinks) {
            let mut j = Cursor::from_head(i);
            while let Some(j) = j.next(&self.vlinks) {
                self.hlinks.remove(j);
                self.sizes[self.headers[j]] -= 1;
            }
        }
    }

    pub fn uncover(&mut self, cell: Cell) {
        let mut i = Cursor::from_head(cell);
        while let Some(i) = i.prev(&self.hlinks) {
            let mut j = Cursor::from_head(i);
            while let Some(j) = j.prev(&self.vlinks) {
                self.sizes[self.headers[j]] += 1;
                self.hlinks.restore(j);
            }
        }
        self.vlinks.restore(cell);
    }
}

impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "s: ")?;
        for s in &self.sizes {
            write!(f, "{:^5}", s)?;
        }
        writeln!(f)?;

        write!(f, "c: ")?;
        for &Cell(c) in &self.headers {
            write!(f, "{:^5}", c.saturating_sub(1))?;
        }
        writeln!(f)?;

        write!(f, "x: ")?;
        for link in &self.vlinks.data {
            write!(f, " {:>1}|{:<1} ", link.prev.0, link.next.0)?
        }
        writeln!(f)?;

        write!(f, "y: ")?;
        for link in &self.hlinks.data {
            write!(f, " {:>1}|{:<1} ", link.prev.0, link.next.0)?
        }
        writeln!(f)?;

        write!(f, "i: ")?;
        for i in 0..self.hlinks.data.len() {
            write!(f, "{:^5}", i)?;
        }
        writeln!(f)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let mut m = Matrix::new(3);
        m.add_row(&[true, false, true]);
        println!("{}", m);
        assert!(false);
    }
}
