use crate::settings;
use crate::shapes;
use crate::states;

pub fn apply_constraints(state: &mut states::State, constraints: &[settings::Constraint]) {
    for constraint in constraints {
        if let settings::Constraint::GivenSymbol {
            tokenset,
            cell,
            symbol,
        } = constraint
        {
            apply_given_symbol(state, *tokenset, cell, *symbol);
        }
    }
}

fn apply_given_symbol(
    state: &mut states::State,
    tokenset: usize,
    cell: &shapes::Cell,
    symbol: settings::SymbolType,
) {
    let tokenset = &mut state.tokensets[tokenset];
    match tokenset {
        states::Tokenset::Symbols(cells) => {
            cells[cell] = states::CellState::Set(symbol);
        }
    }
}
