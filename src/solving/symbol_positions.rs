use crate::settings;
use crate::shapes;
use crate::states;

type SymbolPositions =
    std::collections::HashMap<settings::SymbolType, std::collections::HashSet<shapes::Cell>>;

pub fn symbol_positions(
    cells: &states::CellGrid,
    region: &shapes::Region,
) -> (SymbolPositions, SymbolPositions) {
    let mut set_positions = SymbolPositions::new();
    let mut candidate_positions = SymbolPositions::new();
    for cell in &region.cells {
        match &cells[cell] {
            states::CellState::Set(value) => {
                set_positions
                    .entry(*value)
                    .or_insert(std::collections::HashSet::new())
                    .insert(cell.clone());
            }
            states::CellState::Candidates(candidates) => candidates.iter().for_each(|candidate| {
                candidate_positions
                    .entry(*candidate)
                    .or_insert(std::collections::HashSet::new())
                    .insert(cell.clone());
            }),
            states::CellState::Empty => (),
        }
    }
    (set_positions, candidate_positions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nominal() {
        let cells = vec![
            vec![
                states::CellState::Set('a'),
                states::CellState::Candidates(['b', 'c'].iter().cloned().collect()),
                states::CellState::Set('a'),
                states::CellState::Candidates(['e', 'f'].iter().cloned().collect()),
            ],
            vec![
                states::CellState::Candidates(['g', 'h'].iter().cloned().collect()),
                states::CellState::Candidates(['e', 'j'].iter().cloned().collect()),
                states::CellState::Set('k'),
                states::CellState::Set('l'),
            ],
        ];
        let region = shapes::Region {
            cells: vec![
                shapes::Cell { x: 0, y: 0 },
                shapes::Cell { x: 2, y: 0 },
                shapes::Cell { x: 3, y: 0 },
                shapes::Cell { x: 1, y: 1 },
                shapes::Cell { x: 2, y: 1 },
            ],
        };

        let (set, candidates) = symbol_positions(&cells, &region);
        let expected_set = [
            (
                'a',
                [shapes::Cell { x: 0, y: 0 }, shapes::Cell { x: 2, y: 0 }]
                    .iter()
                    .cloned()
                    .collect(),
            ),
            ('k', [shapes::Cell { x: 2, y: 1 }].iter().cloned().collect()),
        ]
        .iter()
        .cloned()
        .collect();
        let expected_candidates = [
            (
                'e',
                [shapes::Cell { x: 3, y: 0 }, shapes::Cell { x: 1, y: 1 }]
                    .iter()
                    .cloned()
                    .collect(),
            ),
            ('f', [shapes::Cell { x: 3, y: 0 }].iter().cloned().collect()),
            ('j', [shapes::Cell { x: 1, y: 1 }].iter().cloned().collect()),
        ]
        .iter()
        .cloned()
        .collect();
        assert_eq!(set, expected_set);
        assert_eq!(candidates, expected_candidates);
    }
}
