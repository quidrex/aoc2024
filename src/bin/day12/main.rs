use aoc2024::util::parse_matrix::ToMatrix;
use aoc2024::{aoc_day, AocDay};
use ndarray::Array2;
use std::ops::Add;
use strum::{EnumIter, FromRepr, IntoEnumIterator};

aoc_day!(Day12, 1930, 1206);

#[derive(Clone)]
struct Day12 {
    matrix: Array2<char>,
}

impl AocDay for Day12 {
    fn from(input: &str) -> Self {
        let grid = input.parse_matrix(|c| c);
        Day12 { matrix: grid }
    }

    fn a(&self) -> i64 {
        let metrics_list = self.to_region_matrix().calculate();
        metrics_list
            .iter()
            .map(|metrics| metrics.area * metrics.perimeter)
            .sum::<usize>() as i64
    }

    fn b(&self) -> i64 {
        let metrics_list = self.to_region_matrix().calculate();
        metrics_list
            .iter()
            .map(|metrics| metrics.area * metrics.sides)
            .sum::<usize>() as i64
    }
}

impl Day12 {
    pub fn to_region_matrix(&self) -> RegionMatrix {
        let mut state = ToRegionMatrixState {
            visited_matrix: Array2::from_elem(self.matrix.raw_dim(), false),
            region_matrix: Array2::from_elem(self.matrix.raw_dim(), 0),
            region_idx: 0,
            region_val: '\x00',
        };

        for (idx, &val) in self.matrix.indexed_iter() {
            if !state.visited_matrix[idx] {
                state.region_val = val;
                self.dfs_to_region_matrix(&mut state, idx.into());
                state.region_idx += 1;
            }
        }

        RegionMatrix {
            region_matrix: state.region_matrix,
            region_count: state.region_idx,
        }
    }

    fn dfs_to_region_matrix(&self, state: &mut ToRegionMatrixState, idx: [usize; 2]) {
        state.visited_matrix[idx] = true;
        state.region_matrix[idx] = state.region_idx;

        for dir in Dir4::iter() {
            let next_idx = idx + dir;
            if state.visited_matrix.get(next_idx) == Some(&false)
                && self.matrix[next_idx] == state.region_val
            {
                self.dfs_to_region_matrix(state, next_idx)
            }
        }
    }
}

struct ToRegionMatrixState {
    visited_matrix: Array2<bool>,
    region_matrix: Array2<usize>,
    region_idx: usize,
    region_val: char,
}

struct RegionMatrix {
    region_matrix: Array2<usize>,
    region_count: usize,
}

impl RegionMatrix {
    pub fn calculate(&self) -> Vec<RegionMetrics> {
        let mut region_metrics = vec![RegionMetrics::default(); self.region_count];

        for row_idx in -1..self.region_matrix.dim().0 as i32 {
            let mut last_val = [None, None];
            for col_idx in 0..self.region_matrix.dim().1 {
                let val = [
                    self.region_matrix.get((row_idx as usize, col_idx)),
                    self.region_matrix.get(((row_idx + 1) as usize, col_idx)),
                ];

                if let Some(&v) = val[0] {
                    region_metrics[v].area += 1;
                }

                Self::detect_perimeter(&mut region_metrics, &mut last_val, val);

                last_val = val;
            }
        }

        for col_idx in -1..self.region_matrix.dim().1 as i32 {
            let mut last_val = [None, None];
            for row_idx in 0..self.region_matrix.dim().0 {
                let val = [
                    self.region_matrix.get((row_idx, col_idx as usize)),
                    self.region_matrix.get((row_idx, (col_idx + 1) as usize)),
                ];

                Self::detect_perimeter(&mut region_metrics, &mut last_val, val);

                last_val = val;
            }
        }

        region_metrics
    }

    fn detect_perimeter(
        region_metrics: &mut Vec<RegionMetrics>,
        last_val: &[Option<&usize>; 2],
        val: [Option<&usize>; 2],
    ) {
        if val[0] != val[1] {
            if let Some(&v) = val[0] {
                region_metrics[v].perimeter += 1;

                if last_val[0] != val[0] || last_val[0] == val[0] && last_val[1] == val[0] {
                    region_metrics[v].sides += 1;
                }
            }
            if let Some(&v) = val[1] {
                region_metrics[v].perimeter += 1;

                if last_val[1] != val[1] || last_val[0] == val[1] && last_val[1] == val[1] {
                    region_metrics[v].sides += 1;
                }
            }
        }
    }
}

#[derive(Copy, Clone, Default)]
struct RegionMetrics {
    area: usize,
    perimeter: usize,
    sides: usize,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, FromRepr, EnumIter)]
enum Dir4 {
    Up = 0,
    Right,
    Down,
    Left,
}

impl Dir4 {
    fn dir(&self) -> [isize; 2] {
        match self {
            Dir4::Up => [-1, 0],
            Dir4::Right => [0, 1],
            Dir4::Down => [1, 0],
            Dir4::Left => [0, -1],
        }
    }
}

impl Add<Dir4> for [usize; 2] {
    type Output = [usize; 2];

    fn add(self, rhs: Dir4) -> Self::Output {
        let dir = rhs.dir();
        [
            self[0].wrapping_add_signed(dir[0]),
            self[1].wrapping_add_signed(dir[1]),
        ]
    }
}
