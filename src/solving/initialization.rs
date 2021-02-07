use crate::settings::Rule;
use crate::settings::PuzzleSetting;
use crate::settings::TokenSet;
use crate::solving::states::CellState;
use crate::solving::states::State;
use crate::solving::states::Tokenset;

pub fn initialize(setting: &PuzzleSetting) -> State {
    let mut result = State {
        tokensets: setting
            .tokensets
            .iter()
            .map(|tokenset| match tokenset {
                TokenSet::Symbols { grid, candidates } => {
                    let grid = &setting.grids[*grid];
                    let candidates = (0..grid.columns)
                        .map(|_| {
                            (0..grid.rows)
                                .map(|_| CellState::Candidates(candidates.clone()))
                                .collect()
                        })
                        .collect();
                    Tokenset::Symbols(candidates)
                }
            })
            .collect(),
    };
    for rule in &setting.rules {
        match rule {
            Rule::Sudoku {tokenset, regions: _, givens} => {
                for given in givens {
                    match &mut result.tokensets[*tokenset] {
                        Tokenset::Symbols(candidates) => {
                            candidates[given.cell.x as usize][given.cell.y as usize] =
                                CellState::Set(given.symbol);
                        }
                    }
                }
            }
        }
    }
    result
}
