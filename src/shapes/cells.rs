#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
pub struct Cell(pub isize, pub isize);

pub struct DirectionGenerator {
    count: isize,
    factor: isize,
    remaining_directions: usize,
    x: isize,
    y: isize,
}

impl DirectionGenerator {
    pub fn new(x: isize, y: isize, count: isize) -> DirectionGenerator {
        DirectionGenerator{
            count,
            factor: count,
            remaining_directions: 4,
            x,
            y,
        }
    }
}

impl Iterator for DirectionGenerator {
    type Item = Cell;

    fn next(&mut self) -> Option<Cell> {
        self.factor += 1;
        if self.factor > self.count {
            self.factor = 1;
            self.remaining_directions -= 1;
            if self.remaining_directions <= 0 {
                return None;
            }
            let tmp = self.y;
            self.y = -self.x;
            self.x = tmp;
        }
        let cell = Cell(self.factor * self.x, self.factor * self.y);
        return Some(cell);
    }
}
