use crate::settings;
use crate::shapes;

pub fn givens(string_grid: Vec<String>) -> Vec<settings::GivenSymbol> {
    string_grid
        .iter()
        .enumerate()
        .flat_map(|(row, row_contents)| {
            row_contents
                .chars()
                .enumerate()
                .filter_map(move |(column, symbol)| match symbol {
                    '.' | ' ' => None,
                    _ => Some(settings::GivenSymbol {
                        symbol,
                        cell: shapes::Cell {
                            x: column as isize,
                            y: row as isize,
                        },
                    }),
                })
        })
        .collect::<Vec<_>>()
}
