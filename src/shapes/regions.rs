use super::cells;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
pub struct Region {
    pub cells: Vec<cells::Cell>,
}

impl Region {
    fn row(position: isize, size: isize) -> Region {
        Region{
            cells: (0..size).map(move |index| cells::Cell{x: index, y: position}).collect(),
        }
    }

    pub fn rows(count: isize, size: isize) -> impl Iterator<Item=Region> {
        (0..count).map(move |index| Region::row(index, size))
    }

    fn column(position: isize, size: isize) -> Region {
        Region{
            cells: (0..size).map(|index| cells::Cell{x: position, y: index}).collect(),
        }
    }

    pub fn columns(count: isize, size: isize) -> impl Iterator<Item=Region> {
        (0..count).map(move |index| Region::column(index, size))
    }

    fn grid_box(v_offset: isize, h_offset: isize, width: isize, height: isize) -> Region {
        Region {
            cells: (0..width).flat_map(|x| (0..height).map(
                move |y| cells::Cell{
                    x: v_offset * width + x,
                    y: h_offset * height + y,
                }
            )).collect(),
        }
    }

    pub fn grid_boxes(v_count: isize, h_count: isize, width: isize, height: isize) -> impl Iterator<Item=Region> {
        (0..h_count).flat_map(move |h_offset| (0..v_count).map(move |v_offset| {
            Region::grid_box(v_offset, h_offset, width, height)
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::Region;
    use super::cells;

    #[test]
    fn test_rows_nominal() {
        let mut result = Region::rows(3, 3).collect::<Vec<_>>();
        let mut expected = vec![
            Region {
                cells: vec![
                    cells::Cell{x: 0, y: 0},
                    cells::Cell{x: 1, y: 0},
                    cells::Cell{x: 2, y: 0},
                ],
            },
            Region {
                cells: vec![
                    cells::Cell{x: 0, y: 1},
                    cells::Cell{x: 1, y: 1},
                    cells::Cell{x: 2, y: 1},
                ],
            },
            Region {
                cells: vec![
                    cells::Cell{x: 0, y: 2},
                    cells::Cell{x: 1, y: 2},
                    cells::Cell{x: 2, y: 2},
                ],
            },
        ];
        result.sort();
        expected.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_columns_nominal() {
        let mut result = Region::columns(3, 3).collect::<Vec<_>>();
        let mut expected = vec![
            Region {
                cells: vec![
                    cells::Cell{x: 0, y: 0},
                    cells::Cell{x: 0, y: 1},
                    cells::Cell{x: 0, y: 2},
                ],
            },
            Region {
                cells: vec![
                    cells::Cell{x: 1, y: 0},
                    cells::Cell{x: 1, y: 1},
                    cells::Cell{x: 1, y: 2},
                ],
            },
            Region {
                cells: vec![
                    cells::Cell{x: 2, y: 0},
                    cells::Cell{x: 2, y: 1},
                    cells::Cell{x: 2, y: 2},
                ],
            },
        ];
        result.sort();
        expected.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_boxes_nominal() {
        let mut result = Region::grid_boxes(2, 2, 2, 2).collect::<Vec<_>>();
        let mut expected = vec![
            Region {
                cells: vec![
                    cells::Cell{x: 0, y: 0},
                    cells::Cell{x: 0, y: 1},
                    cells::Cell{x: 1, y: 0},
                    cells::Cell{x: 1, y: 1},
                ],
            },
            Region {
                cells: vec![
                    cells::Cell{x: 2, y: 0},
                    cells::Cell{x: 2, y: 1},
                    cells::Cell{x: 3, y: 0},
                    cells::Cell{x: 3, y: 1},
                ],
            },
            Region {
                cells: vec![
                    cells::Cell{x: 0, y: 2},
                    cells::Cell{x: 0, y: 3},
                    cells::Cell{x: 1, y: 2},
                    cells::Cell{x: 1, y: 3},
                ],
            },
            Region {
                cells: vec![
                    cells::Cell{x: 2, y: 2},
                    cells::Cell{x: 2, y: 3},
                    cells::Cell{x: 3, y: 2},
                    cells::Cell{x: 3, y: 3},
                ],
            },
        ];
        result.sort();
        expected.sort();
        assert_eq!(result, expected);
    }
}
