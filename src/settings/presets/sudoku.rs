use crate::settings;
use crate::settings::presets;
use crate::shapes;

pub fn setting(box_width: isize, givens: &[String]) -> settings::PuzzleSetting {
    // TODO: sizes checks
    let size = givens.len();
    let box_height = size as isize / box_width;
    let mut setting = settings::PuzzleSetting::default();
    let grid = setting.add_grid(size, size);
    let regions =
        shapes::Region::grid_boxes(box_width, box_height, box_width, box_height).collect();
    let givens = presets::givens(givens);
    add_symbolset(&mut setting, grid, size as u32, regions, givens);
    setting
}

pub fn add_symbolset(
    setting: &mut settings::PuzzleSetting,
    grid: settings::GridIndex,
    digits: u32,
    regions: Vec<shapes::Region>,
    givens: Vec<settings::GivenSymbol>,
) -> settings::TokenSetIndex {
    let candidates = (1u32..=digits)
        .map(|digit| std::char::from_digit(digit, 10).unwrap())
        .collect::<Vec<_>>();
    let category = settings::TokenSet::Symbols { candidates, grid };
    let tokenset = setting.add_symbolset(category);
    let rule = settings::Rule::Sudoku {
        tokenset,
        regions,
        givens,
    };
    setting.rules.push(rule);
    tokenset
}
