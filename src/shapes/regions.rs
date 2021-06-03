use super::cells;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
pub struct Region {
    pub cells: Vec<cells::Cell>,
}

impl Region {
    fn row(position: usize, size: usize) -> Region {
        Region {
            cells: (0..size)
                .map(move |index| cells::Cell(index as isize, position as isize))
                .collect(),
        }
    }

    pub fn rows(count: usize, size: usize) -> impl Iterator<Item = Region> {
        (0..count).map(move |index| Region::row(index, size))
    }

    fn column(position: usize, size: usize) -> Region {
        Region {
            cells: (0..size)
                .map(|index| cells::Cell(position as isize, index as isize))
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
                        .map(move |y| cells::Cell(
                            (v_offset * width + x) as isize,
                            (h_offset * height + y) as isize
                        ))
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

    fn offset_box(
        v_offset: usize,
        h_offset: usize,
        width: usize,
        height: usize,
    ) -> Region {
        let cells = (0..width)
            .flat_map(|x| {
                (0..height)
                    .map(move |y| cells::Cell(
                        (v_offset + x * width) as isize,
                        (h_offset + y * height) as isize
                    ))
            })
            .collect();
        Region { cells }
    }

    pub fn offset_boxes(
        v_count: usize,
        h_count: usize,
        width: usize,
        height: usize,
    ) -> impl Iterator<Item = Region> {
        (0..h_count).flat_map(move |h_offset| {
            (0..v_count).map(move |v_offset| Region::offset_box(v_offset, h_offset, width, height))
        })
    }

    pub fn diagonal(size: usize) -> Region {
        let cells = (0..size as isize)
            .map(move |offset| cells::Cell(offset, offset))
            .collect();
        Region {cells}
    }

    pub fn reverse_diagonal(size: usize) -> Region {
        let cells = (0..size)
            .map(move |offset| cells::Cell((size - offset - 1) as isize, offset as isize))
            .collect();
        Region {cells}
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
