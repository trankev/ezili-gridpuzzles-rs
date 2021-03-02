use crate::settings;
use crate::shapes;
use crate::solving;
use crate::solving::strategies;
use crate::states;
use itertools::Itertools;

pub struct HiddenTuples {
    min_size: usize,
    max_size: Option<usize>,
}

impl strategies::Strategy for HiddenTuples {
    fn search(
        &self,
        constraints: &Vec<settings::Constraint>,
        state: &states::State,
    ) -> Result<Vec<strategies::Deduction>, Box<dyn std::error::Error>> {
        let mut result = Vec::new();
        for constraint in constraints {
            if let settings::Constraint::SymbolRepartition {
                tokenset,
                repartition,
                region,
            } = constraint
            {
                let states::Tokenset::Symbols(cells) = &state.tokensets[*tokenset];
                result.extend(self.search_for_constraint(*tokenset, repartition, region, cells)?);
            }
        }
        Ok(result)
    }
}

impl HiddenTuples {
    pub fn with_fixed_size(size: usize) -> HiddenTuples {
        HiddenTuples {
            min_size: size,
            max_size: Some(size),
        }
    }

    pub fn with_min_size(size: usize) -> HiddenTuples {
        HiddenTuples {
            min_size: size,
            max_size: None,
        }
    }

    fn search_for_constraint(
        &self,
        tokenset: settings::TokenSetIndex,
        repartition: &settings::SymbolRepartition,
        region: &shapes::Region,
        cells: &states::CellGrid,
    ) -> Result<Vec<strategies::Deduction>, Box<dyn std::error::Error>> {
        let mut result = Vec::new();
        let (set_positions, mut candidate_positions) = solving::symbol_positions(cells, region);
        candidate_positions
            .iter_mut()
            .filter(|(symbol, _)| repartition.contains_key(symbol))
            .for_each(|(symbol, positions)| {
                if let Some(to_add) = set_positions.get(symbol) {
                    to_add.iter().for_each(|position| {
                        positions.insert(position.clone());
                    });
                }
            });
        let mut max_size = self.max_size.unwrap_or(candidate_positions.len() - 1);
        if max_size >= candidate_positions.len() {
            max_size = candidate_positions.len() - 1;
        }
        for size in self.min_size..=max_size {
            for combination in candidate_positions.keys().combinations(size) {
                let needed_count = combination
                    .iter()
                    .fold(0, |acc, symbol| acc + repartition.get(symbol).unwrap());
                let positions = combination
                    .iter()
                    .fold(std::collections::HashSet::new(), |acc, symbol| {
                        acc.union(&candidate_positions[symbol]).cloned().collect()
                    });
                if positions.len() < needed_count {
                    return Err("Not enough candidates".to_string().into());
                }
                if positions.len() > needed_count {
                    continue;
                }
                let actions = list_actions(tokenset, cells, &combination, &positions);
                if !actions.is_empty() {
                    let deduction = strategies::Deduction {
                        category: strategies::Category::HiddenTuple {
                            symbols: combination.iter().cloned().cloned().collect(),
                            region: region.clone(),
                            positions: positions.iter().cloned().collect(),
                        },
                        actions,
                    };
                    result.push(deduction);
                }
            }
        }
        Ok(result)
    }
}

fn list_actions(
    tokenset: settings::TokenSetIndex,
    cells: &states::CellGrid,
    symbols: &Vec<&settings::SymbolType>,
    positions: &std::collections::HashSet<shapes::Cell>,
) -> Vec<strategies::Action> {
    if symbols.len() == 1 {
        symbols
            .iter()
            .flat_map(|symbol| {
                positions
                    .iter()
                    .map(move |cell| strategies::Action::SetSymbol {
                        tokenset,
                        cell: cell.clone(),
                        symbol: **symbol,
                    })
            })
            .collect()
    } else {
        std::iter::Iterator::flatten(positions.iter().filter_map(|position| {
            if let states::CellState::Candidates(candidates) = &cells[position] {
                Some(
                    candidates
                        .iter()
                        .map(move |candidate| (position.clone(), candidate.clone())),
                )
            } else {
                None
            }
        }))
        .filter(|(_, candidate)| !symbols.contains(&candidate))
        .map(
            |(cell, candidate)| strategies::Action::RemoveSymbolCandidate {
                tokenset,
                cell: cell.clone(),
                symbol: candidate,
            },
        )
        .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solving::strategies::Strategy;

    #[test]
    fn test_hidden_pair() -> Result<(), Box<dyn std::error::Error>> {
        let state = states::State {
            tokensets: vec![states::Tokenset::Symbols(vec![vec![
                states::CellState::Candidates("134".chars().collect()),
                states::CellState::Candidates("234".chars().collect()),
                states::CellState::Candidates("12".chars().collect()),
                states::CellState::Candidates("12".chars().collect()),
            ]])],
        };

        let region = shapes::Region {
            cells: (0..4).map(|x| shapes::Cell { x, y: 0 }).collect(),
        };
        let constraints = vec![settings::Constraint::SymbolRepartition {
            tokenset: 0,
            repartition: "1234".chars().map(|symbol| (symbol, 1)).collect(),
            region: region.clone(),
        }];

        let technique = HiddenTuples::with_fixed_size(2);
        let result = technique.search(&constraints, &state)?;
        let expected = vec![strategies::Deduction {
            category: strategies::Category::HiddenTuple {
                symbols: "34".chars().collect(),
                region,
                positions: vec![shapes::Cell { x: 0, y: 0 }, shapes::Cell { x: 1, y: 0 }],
            },
            actions: vec![
                strategies::Action::RemoveSymbolCandidate {
                    tokenset: 0,
                    cell: shapes::Cell { x: 0, y: 0 },
                    symbol: '1',
                },
                strategies::Action::RemoveSymbolCandidate {
                    tokenset: 0,
                    cell: shapes::Cell { x: 1, y: 0 },
                    symbol: '2',
                },
            ],
        }];
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_hidden_single() -> Result<(), Box<dyn std::error::Error>> {
        let state = states::State {
            tokensets: vec![states::Tokenset::Symbols(vec![vec![
                states::CellState::Candidates("124".chars().collect()),
                states::CellState::Candidates("1234".chars().collect()),
                states::CellState::Candidates("124".chars().collect()),
                states::CellState::Candidates("124".chars().collect()),
            ]])],
        };

        let region = shapes::Region {
            cells: (0..4).map(|x| shapes::Cell { x, y: 0 }).collect(),
        };
        let constraints = vec![settings::Constraint::SymbolRepartition {
            tokenset: 0,
            repartition: "1234".chars().map(|symbol| (symbol, 1)).collect(),
            region: region.clone(),
        }];

        let technique = HiddenTuples::with_fixed_size(1);
        let result = technique.search(&constraints, &state)?;
        let expected = vec![strategies::Deduction {
            category: strategies::Category::HiddenTuple {
                symbols: "3".chars().collect(),
                region,
                positions: vec![shapes::Cell { x: 1, y: 0 }],
            },
            actions: vec![strategies::Action::SetSymbol {
                tokenset: 0,
                cell: shapes::Cell { x: 1, y: 0 },
                symbol: '3',
            }],
        }];
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_no_results() -> Result<(), Box<dyn std::error::Error>> {
        let state = states::State {
            tokensets: vec![states::Tokenset::Symbols(vec![vec![
                states::CellState::Candidates("12".chars().collect()),
                states::CellState::Candidates("23".chars().collect()),
                states::CellState::Candidates("34".chars().collect()),
                states::CellState::Candidates("14".chars().collect()),
            ]])],
        };

        let region = shapes::Region {
            cells: (0..4).map(|x| shapes::Cell { x, y: 0 }).collect(),
        };
        let constraints = vec![settings::Constraint::SymbolRepartition {
            tokenset: 0,
            repartition: "1234".chars().map(|symbol| (symbol, 1)).collect(),
            region: region.clone(),
        }];

        let technique = HiddenTuples::with_min_size(1);
        let result = technique.search(&constraints, &state)?;
        let expected = vec![];
        assert_eq!(result, expected);
        Ok(())
    }
}
