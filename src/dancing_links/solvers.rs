use crate::dancing_links::cursors::Cursor;
use crate::dancing_links::matrix::HEAD;
use crate::dancing_links::matrix::Matrix;

pub fn solve(matrix: &mut Matrix) -> usize {
    let mut n_answers = 0;
    let cell = {
        let mut i = Cursor::from_head(HEAD);
        let mut cell = match i.next(&matrix.vlinks) {
            Some(it) => it,
            None => {
                return 1;
            }
        };
        while let Some(next_cell) = i.next(&matrix.vlinks) {
            if matrix.sizes[next_cell] < matrix.sizes[cell] {
                cell = next_cell;
            }
        }
        cell
    };
    matrix.cover(cell);
    let mut i = Cursor::from_head(cell);
    while let Some(i) = i.next(&matrix.hlinks) {
        let mut j = Cursor::from_head(i);
        while let Some(j) = j.next(&matrix.vlinks) {
            matrix.cover(matrix.headers[j]);
        }
        n_answers += solve(matrix);
        let mut j = Cursor::from_head(i);
        while let Some(j) = j.next(&matrix.vlinks) {
            matrix.uncover(matrix.headers[j]);
        }
    }
    matrix.uncover(cell);
    n_answers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_problem() {
        let f = false;
        let t = true;

        let mut m = Matrix::new(7);
        m.add_row(&[f, f, t, f, t, t, f]);
        m.add_row(&[t, f, f, t, f, f, t]);
        m.add_row(&[f, t, t, f, f, t, f]);
        m.add_row(&[t, f, f, t, f, f, f]);
        m.add_row(&[f, t, f, f, f, f, t]);
        m.add_row(&[f, f, f, t, t, f, t]);
        println!("{}", m);

        let solutions = solve(&mut m);
        assert_eq!(solutions, 1);
        assert!(false);
    }
}
