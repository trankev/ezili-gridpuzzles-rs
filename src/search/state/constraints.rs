use crate::shapes;
use crate::settings;
use crate::types;

pub type SymbolRepartition = std::collections::HashMap<types::SymbolType, usize>;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Constraint {
    GivenSymbol{symbol: types::SymbolType, cell: shapes::Cell},
    SymbolRepartition{repartition: SymbolRepartition, region: shapes::Region},
}

impl Constraint {
    pub fn from_settings(grid: &settings::Grid, rule: &settings::Rule) -> Vec<Constraint> {
        let mut result = Vec::new();
        match rule {
            settings::Rule::Sudoku{region_config, givens, variants} => {
                for given in givens {
                    let constraint = Constraint::GivenSymbol{
                        symbol: given.symbol,
                        cell: given.cell.clone(),
                    };
                    result.push(constraint);
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
                            result.push(constraint);
                        }
                    },
                    settings::RegionConfig::Irregular{regions} => {
                        for region in regions {
                            let constraint = Constraint::SymbolRepartition{
                                repartition: repartition.clone(),
                                region: region.clone(),
                            };
                            result.push(constraint);
                        }
                    },
                    settings::RegionConfig::None => (),
                }
            }
        }
        result
    }
}
