use crate::settings;
use crate::shapes;
use crate::states;

pub enum Status {
    Valid,
    Ongoing,
    Invalid,
}

type ResultStatus = Result<Status, Box<dyn std::error::Error>>;

pub fn compute_status(state: &states::State, constraints: &[settings::Constraint]) -> ResultStatus {
    for constraint in constraints {
        let status = match constraint {
            settings::Constraint::GivenSymbol {
                tokenset,
                cell,
                symbol,
            } => check_given_symbol(state, *tokenset, cell, *symbol)?,
            settings::Constraint::SymbolRepartition {
                tokenset,
                region,
                repartition,
            } => check_symbol_repartition(state, *tokenset, region, repartition)?,
        };
        match status {
            Status::Ongoing => (),
            _ => {
                return Ok(status);
            }
        }
    }
    Ok(Status::Ongoing)
}

fn check_given_symbol(
    state: &states::State,
    tokenset: usize,
    cell: &shapes::Cell,
    symbol: char,
) -> ResultStatus {
    let tokenset = &state.tokensets[tokenset];
    match tokenset {
        states::Tokenset::Symbols(cells) => {
            let cell_state = &cells[cell.x as usize][cell.y as usize];
            match cell_state {
                states::CellState::Set(value) => {
                    if *value == symbol {
                        Ok(Status::Valid)
                    } else {
                        Ok(Status::Invalid)
                    }
                }
                _ => Ok(Status::Ongoing),
            }
        }
    }
}

fn check_symbol_repartition(
    state: &states::State,
    tokenset: usize,
    region: &shapes::Region,
    repartition: &settings::SymbolRepartition,
) -> ResultStatus {
    let tokenset = &state.tokensets[tokenset];
    match tokenset {
        states::Tokenset::Symbols(cells) => {
            let mut found = settings::SymbolRepartition::new();
            let mut completed = true;
            for cell in &region.cells {
                let cell_state = &cells[cell.x as usize][cell.y as usize];
                if let states::CellState::Set(value) = cell_state {
                    found
                        .entry(*value)
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                }
            }
            for (symbol, expected_count) in repartition.iter() {
                let count = found.get(symbol).unwrap_or(&0);
                if count > expected_count {
                    return Ok(Status::Invalid);
                }
                if count < expected_count {
                    completed = false;
                }
            }
            Ok(if completed {
                Status::Valid
            } else {
                Status::Ongoing
            })
        }
    }
}
