use crate::shapes;
use crate::settings;
use crate::types;

pub type SymbolRepartition = std::collections::HashMap<types::SymbolType, usize>;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Constraint {
    GivenSymbol{symbol: types::SymbolType, cell: shapes::Cell},
    SymbolRepartition{repartition: SymbolRepartition, region: shapes::Region},
    AntiMove{moves: Vec<shapes::Cell>},
}

impl Constraint {
    pub fn from_settings(grid: &settings::Grid, rule: &settings::Rule) -> Vec<Constraint> {
        let mut constraints = Vec::new();
        match rule {
            settings::Rule::Sudoku{region_config, givens, variants} => {
                for given in givens {
                    let constraint = Constraint::GivenSymbol{
                        symbol: given.symbol,
                        cell: given.cell.clone(),
                    };
                    constraints.push(constraint);
                }
                let repartition = (1u32..=(grid.rows as u32))
                    .map(|digit| (std::char::from_digit(digit, 10).unwrap(), 1))
                    .collect::<SymbolRepartition>();
                match region_config {
                    settings::RegionConfig::Regular{box_width, box_height} => {
                        let regions = shapes::Region::grid_boxes(
                            /* v_count */ *box_width,
                            /* h_count */ *box_height,
                            *box_width,
                            *box_height,
                        );
                        for region in regions {
                            let constraint = Constraint::SymbolRepartition{
                                repartition: repartition.clone(),
                                region: region.clone(),
                            };
                            constraints.push(constraint);
                        }
                    },
                    settings::RegionConfig::Irregular{regions} => {
                        for region in regions {
                            let constraint = Constraint::SymbolRepartition{
                                repartition: repartition.clone(),
                                region: region.clone(),
                            };
                            constraints.push(constraint);
                        }
                    },
                    settings::RegionConfig::None => (),
                }
                for variant in variants {
                    match variant {
                        settings::SudokuVariant::Diagonal => {
                            let constraint = Constraint::SymbolRepartition{
                                repartition: repartition.clone(),
                                region: shapes::Region::diagonal(grid.columns),
                            };
                            constraints.push(constraint);
                        },
                        settings::SudokuVariant::ReverseDiagonal => {
                            let constraint = Constraint::SymbolRepartition{
                                repartition: repartition.clone(),
                                region: shapes::Region::reverse_diagonal(grid.columns),
                            };
                            constraints.push(constraint);
                        },
                        settings::SudokuVariant::DisjointGroup => {
                            if let settings::RegionConfig::Regular{box_width, box_height} = region_config {
                                let regions = shapes::Region::offset_boxes(
                                    /* v_count */ *box_width,
                                    /* h_count */ *box_height,
                                    *box_width,
                                    *box_height,
                                );
                                for region in regions {
                                    let constraint = Constraint::SymbolRepartition{
                                        repartition: repartition.clone(),
                                        region: region.clone(),
                                    };
                                    constraints.push(constraint);
                                }
                            }
                        },
                        settings::SudokuVariant::AntiKnight => {
                            let moves = shapes::DirectionGenerator::new(1, 2, 1)
                                .chain(shapes::DirectionGenerator::new(2, 1, 1))
                                .collect();
                            let constraint = Constraint::AntiMove{moves};
                            constraints.push(constraint);
                        },
                        settings::SudokuVariant::AntiKing => {
                            let moves = shapes::DirectionGenerator::new(1, 0, 1)
                                .chain(shapes::DirectionGenerator::new(1, 1, 1))
                                .collect();
                            let constraint = Constraint::AntiMove{moves};
                            constraints.push(constraint);
                        },
                        settings::SudokuVariant::AntiQueen => {
                            let moves = shapes::DirectionGenerator::new(1, 0, grid.columns as isize)
                                .chain(shapes::DirectionGenerator::new(1, 1, grid.columns as isize))
                                .collect();
                            let constraint = Constraint::AntiMove{moves};
                            constraints.push(constraint);
                        },
                        _ => unreachable!(),
                    }
                }
            }
        }
        constraints
    }
}
