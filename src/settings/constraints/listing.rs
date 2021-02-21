use crate::settings;
use crate::shapes;

pub fn list_constraints(
    setting: &settings::PuzzleSetting,
) -> Result<Vec<settings::Constraint>, Box<dyn std::error::Error>> {
    let mut result = Vec::new();
    for rule in &setting.rules {
        match rule {
            settings::Rule::Sudoku {
                tokenset,
                regions,
                givens,
            } => {
                let constraints = iter_sudoku_constraints(setting, *tokenset, regions, givens)?;
                result.extend(constraints);
            }
        }
    }
    Ok(result)
}

fn iter_sudoku_constraints(
    setting: &settings::PuzzleSetting,
    tokenset_index: settings::TokenSetIndex,
    regions: &[shapes::Region],
    givens: &[settings::GivenSymbol],
) -> Result<Vec<settings::Constraint>, Box<dyn std::error::Error>> {
    let tokenset = setting
        .tokensets
        .get(tokenset_index as usize)
        .ok_or("tokenset index out of range")?;
    let grid_index;
    let repartition;
    match tokenset {
        settings::TokenSet::Symbols { grid, candidates } => {
            grid_index = grid;
            repartition = candidates
                .iter()
                .map(|symbol| (*symbol, 1))
                .collect::<std::collections::HashMap<_, _>>();
        }
    };
    let grid = setting
        .grids
        .get(*grid_index as usize)
        .ok_or("grid index out of range")?;
    let rows = shapes::Region::rows(grid.rows as isize, grid.columns as isize);
    let columns = shapes::Region::columns(grid.columns as isize, grid.rows as isize);
    let all_regions = regions.iter().cloned().chain(rows).chain(columns);
    let repartition_constraints =
        all_regions.map(move |region| settings::Constraint::SymbolRepartition {
            tokenset: tokenset_index,
            repartition: repartition.clone(),
            region,
        });
    let given_contraints = givens
        .iter()
        .map(move |given| settings::Constraint::GivenSymbol {
            tokenset: tokenset_index,
            cell: given.cell.clone(),
            symbol: given.symbol,
        });
    Ok(repartition_constraints.chain(given_contraints).collect())
}

#[cfg(test)]
mod tests {
    use super::list_constraints;
    use crate::settings;
    use crate::settings::presets;
    use crate::settings::presets::sudoku;
    use crate::shapes;

    #[test]
    fn test_sudoku() -> Result<(), Box<dyn std::error::Error>> {
        let mut setting = settings::PuzzleSetting::default();
        let grid = setting.add_grid(4, 4);
        let regions = shapes::Region::grid_boxes(2, 2, 2, 2).collect();
        let givens = presets::givens(vec![
            "5...".to_string(),
            "..6.".to_string(),
            ".1..".to_string(),
            "....".to_string(),
        ]);
        sudoku::add_symbolset(&mut setting, grid, 4, regions, givens);

        let result = list_constraints(&setting)?;

        let repartition = [('1', 1), ('2', 1), ('3', 1), ('4', 1)]
            .iter()
            .cloned()
            .collect::<std::collections::HashMap<char, usize>>();
        let mut expected = Vec::new();
        expected.extend(shapes::Region::grid_boxes(2, 2, 2, 2).map(|region| {
            settings::Constraint::SymbolRepartition {
                tokenset: 0,
                repartition: repartition.clone(),
                region,
            }
        }));
        expected.extend(shapes::Region::rows(4, 4).map(|region| {
            settings::Constraint::SymbolRepartition {
                tokenset: 0,
                repartition: repartition.clone(),
                region,
            }
        }));
        expected.extend(shapes::Region::columns(4, 4).map(|region| {
            settings::Constraint::SymbolRepartition {
                tokenset: 0,
                repartition: repartition.clone(),
                region,
            }
        }));
        expected.extend(
            [
                settings::Constraint::GivenSymbol {
                    tokenset: 0,
                    cell: shapes::Cell { x: 0, y: 0 },
                    symbol: '5',
                },
                settings::Constraint::GivenSymbol {
                    tokenset: 0,
                    cell: shapes::Cell { x: 2, y: 1 },
                    symbol: '6',
                },
                settings::Constraint::GivenSymbol {
                    tokenset: 0,
                    cell: shapes::Cell { x: 1, y: 2 },
                    symbol: '1',
                },
            ]
            .iter()
            .cloned(),
        );

        assert_eq!(result, expected);
        Ok(())
    }
}
