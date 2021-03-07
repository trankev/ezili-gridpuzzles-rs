use super::cells;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
pub struct Region {
    pub cells: Vec<cells::Cell>,
}

impl Region {
    fn row(position: usize, size: usize) -> Region {
        Region {
            cells: (0..size)
                .map(move |index| cells::Cell(index, position))
                .collect(),
        }
    }

    pub fn rows(count: usize, size: usize) -> impl Iterator<Item = Region> {
        (0..count).map(move |index| Region::row(index, size))
    }

    fn column(position: usize, size: usize) -> Region {
        Region {
            cells: (0..size)
                .map(|index| cells::Cell(position, index))
                .collect(),
        }
    }

    pub fn columns(count: usize, size: usize) -> impl Iterator<Item = Region> {
        (0..count).map(move |index| Region::column(index, size))
    }

    fn grid_box(v_offset: usize, h_offset: usize, width: usize, height: usize) -> Region {
        Region {
            cells: (0..width)
                .flat_map(|x| {
                    (0..height)
                        .map(move |y| cells::Cell(v_offset * width + x, h_offset * height + y))
                })
                .collect(),
        }
    }

    pub fn grid_boxes(
        v_count: usize,
        h_count: usize,
        width: usize,
        height: usize,
    ) -> impl Iterator<Item = Region> {
        (0..h_count).flat_map(move |h_offset| {
            (0..v_count).map(move |v_offset| Region::grid_box(v_offset, h_offset, width, height))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::cells;
    use super::Region;

    #[test]
    fn test_rows_nominal() {
        let mut result = Region::rows(3, 3).collect::<Vec<_>>();
        let mut expected = vec![
            Region {
                cells: vec![cells::Cell(0, 0), cells::Cell(1, 0), cells::Cell(2, 0)],
            },
            Region {
                cells: vec![cells::Cell(0, 1), cells::Cell(1, 1), cells::Cell(2, 1)],
            },
            Region {
                cells: vec![cells::Cell(0, 2), cells::Cell(1, 2), cells::Cell(2, 2)],
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
                cells: vec![cells::Cell(0, 0), cells::Cell(0, 1), cells::Cell(0, 2)],
            },
            Region {
                cells: vec![cells::Cell(1, 0), cells::Cell(1, 1), cells::Cell(1, 2)],
            },
            Region {
                cells: vec![cells::Cell(2, 0), cells::Cell(2, 1), cells::Cell(2, 2)],
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
                    cells::Cell(0, 0),
                    cells::Cell(0, 1),
                    cells::Cell(1, 0),
                    cells::Cell(1, 1),
                ],
            },
            Region {
                cells: vec![
                    cells::Cell(2, 0),
                    cells::Cell(2, 1),
                    cells::Cell(3, 0),
                    cells::Cell(3, 1),
                ],
            },
            Region {
                cells: vec![
                    cells::Cell(0, 2),
                    cells::Cell(0, 3),
                    cells::Cell(1, 2),
                    cells::Cell(1, 3),
                ],
            },
            Region {
                cells: vec![
                    cells::Cell(2, 2),
                    cells::Cell(2, 3),
                    cells::Cell(3, 2),
                    cells::Cell(3, 3),
                ],
            },
        ];
        result.sort();
        expected.sort();
        assert_eq!(result, expected);
    }
}
