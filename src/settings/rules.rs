use crate::shapes;
use crate::types;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct GivenSymbol {
    pub symbol: types::SymbolType,
    pub cell: shapes::Cell,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RegionConfig {
    Regular{box_width: usize, box_height: usize},
    Irregular{regions: Vec<shapes::Region>},
    None,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SudokuVariant {
  Diagonal,
  ReverseDiagonal,
  AntiKnight,
  AntiKing,
  AntiQueen,
  DisjointGroup,
  NonConsecutive,
  OddDigits,
  EvenDigits,
  Thermometers,
  KillerCages,
  LittleKiller,
  Palindrome,
  Sandwich,
  Difference,
  Ratio,
  Clone,
  ArrowSum,
  Minimum,
  Maximum,
  BetweenLine,
  Quadruple,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Rule {
    Sudoku {
        region_config: RegionConfig,
        givens: Vec<GivenSymbol>,
        variants: Vec<SudokuVariant>,
    },
}
