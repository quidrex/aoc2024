mod util;

use aoc2024::{aoc_day, parse, AocDay};
use regex::Regex;
use std::iter::Iterator;
use std::sync::LazyLock;
use crate::util::{Matrix, MatrixVec};

aoc_day!(Input, 12);

struct Input {
    dim: MatrixVec,
    guard_rules: Vec<GuardRule>,
}

#[derive(Copy, Clone)]
struct GuardRule {
    pos: MatrixVec,
    vel: MatrixVec,
}

static INPUT_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap());

impl AocDay for Input {
    fn from(input: &str) -> Self {
        let (dim_str, guard_rules_str) = input.split_once("\n\n").unwrap();

        let (dim_col, dim_row) = parse!((i32 "," i32))(dim_str);
        let dim = (dim_row, dim_col).into();

        let guard_rules = INPUT_REGEX
            .captures_iter(guard_rules_str)
            .map(|caps| {
                let pos_col = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
                let pos_row = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
                let vel_col = caps.get(3).unwrap().as_str().parse::<i32>().unwrap();
                let vel_row = caps.get(4).unwrap().as_str().parse::<i32>().unwrap();

                GuardRule {
                    pos: (pos_row, pos_col).into(),
                    vel: (vel_row, vel_col).into(),
                }
            })
            .collect::<Vec<_>>();

        Input { dim, guard_rules }
    }

    fn a(&self) -> i64 {
        self.calculate_safety_factor(100) as i64
    }

    fn b(&self) -> i64 {
        self.find_tree() as i64
        /*let mut seconds = 8;

        loop {
            seconds += 101;
            let matrix = self.calculate_matrix(seconds);
            println!("{} seconds:", seconds);
            for row_idx in 0..self.dim.row {
                for col_idx in 0..self.dim.col {
                    let c = if matrix[(row_idx, col_idx).into()] > 0 {
                        '#'
                    } else {
                        '.'
                    };
                    print!("{}", c);
                }
                println!();
            }
            println!();

            io::stdin().lines().next();
        }*/
    }
}

impl Input {
    fn calculate_safety_factor(&self, seconds: i32) -> i32 {
        let matrix = self.calculate_matrix(seconds);

        let quadrant_iters = [
            MatrixVec::iter_row_major((0, 0).into(), (self.dim.row / 2, self.dim.col / 2).into()),
            MatrixVec::iter_row_major(
                (0, self.dim.col / 2 + 1).into(),
                (self.dim.row / 2, self.dim.col).into(),
            ),
            MatrixVec::iter_row_major(
                (self.dim.row / 2 + 1, 0).into(),
                (self.dim.row, self.dim.col / 2).into(),
            ),
            MatrixVec::iter_row_major(
                (self.dim.row / 2 + 1, self.dim.col / 2 + 1).into(),
                (self.dim.row, self.dim.col).into(),
            ),
        ];

        quadrant_iters
            .into_iter()
            .map(|iter| iter.map(|idx| matrix[idx]).sum::<i32>())
            .product()
    }

    fn find_tree(&self) -> i32 {
        let mut seconds = 0;

        loop {
            seconds += 1;
            let matrix = self.calculate_matrix(seconds);

            for row_idx in 0..self.dim.row - 2 {
                for col_idx in 0..self.dim.col - 2 {
                    if matrix[(row_idx, col_idx).into()] > 0 {
                        let count = MatrixVec::iter_row_major(
                            (row_idx, col_idx).into(),
                            (row_idx + 3, col_idx + 3).into(),
                        )
                        .map(|idx| (matrix[idx] > 0) as i32)
                        .sum::<i32>();

                        if count == 9 {
                            return seconds;
                        }
                    }
                }
            }
        }
    }

    fn calculate_matrix(&self, seconds: i32) -> Matrix<i32> {
        let mut matrix = Matrix::from_elem(self.dim, 0);

        for guard_rule in &self.guard_rules {
            let final_guard_pos = (guard_rule.pos + guard_rule.vel * seconds).mod_euc(self.dim);
            matrix[final_guard_pos] += 1;
        }

        matrix
    }
}
