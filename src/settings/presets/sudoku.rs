use crate::settings;
use crate::shapes;

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
