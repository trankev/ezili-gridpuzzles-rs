use crate::settings;
use crate::shapes;

pub fn givens(string_grid: &[String]) -> Vec<settings::GivenSymbol> {
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
                        cell: shapes::Cell(column, row),
                    }),
                })
        })
        .collect::<Vec<_>>()
}
