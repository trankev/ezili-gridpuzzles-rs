use crate::settings;
use crate::shapes;
use crate::states;

type SymbolPositions =
    std::collections::HashMap<settings::SymbolType, std::collections::HashSet<shapes::Cell>>;

pub fn symbol_positions(
    #[allow(clippy::ptr_arg)] cells: &states::CellGrid,
    region: &shapes::Region,
) -> (SymbolPositions, SymbolPositions) {
    let mut set_positions = SymbolPositions::new();
    let mut candidate_positions = SymbolPositions::new();
    for cell in &region.cells {
        match &cells[cell] {
            states::CellState::Set(value) => {
                set_positions
                    .entry(*value)
                    .or_insert_with(std::collections::HashSet::new)
                    .insert(cell.clone());
            }
            states::CellState::Candidates(candidates) => candidates.chars().for_each(|candidate| {
                candidate_positions
                    .entry(candidate)
                    .or_insert_with(std::collections::HashSet::new)
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
                states::CellState::Candidates("bc".to_string()),
                states::CellState::Set('a'),
                states::CellState::Candidates("ef".to_string()),
            ],
            vec![
                states::CellState::Candidates("gh".to_string()),
                states::CellState::Candidates("ej".to_string()),
                states::CellState::Set('k'),
                states::CellState::Set('l'),
            ],
        ];
        let region = shapes::Region {
            cells: vec![
                shapes::Cell(0, 0),
                shapes::Cell(2, 0),
                shapes::Cell(3, 0),
                shapes::Cell(1, 1),
                shapes::Cell(2, 1),
            ],
        };

        let (set, candidates) = symbol_positions(&cells, &region);
        let expected_set = [
            (
                'a',
                [shapes::Cell(0, 0), shapes::Cell(2, 0)]
                    .iter()
                    .cloned()
                    .collect(),
            ),
            ('k', [shapes::Cell(2, 1)].iter().cloned().collect()),
        ]
        .iter()
        .cloned()
        .collect();
        let expected_candidates = [
            (
                'e',
                [shapes::Cell(3, 0), shapes::Cell(1, 1)]
                    .iter()
                    .cloned()
                    .collect(),
            ),
            ('f', [shapes::Cell(3, 0)].iter().cloned().collect()),
            ('j', [shapes::Cell(1, 1)].iter().cloned().collect()),
        ]
        .iter()
        .cloned()
        .collect();
        assert_eq!(set, expected_set);
        assert_eq!(candidates, expected_candidates);
    }
}
