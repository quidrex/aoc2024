use anyhow::{Error, Result};
use aoc2024::init;
use ndarray::{s, Array2, ArrayView1, ArrayView2};
use std::iter::Iterator;
use std::str::FromStr;

init!("18", "9");

fn run(input: &str) -> Result<(String, String)> {
    let word_matrix = input.parse::<WordMatrix>()?;

    let a = word_matrix.count_xmas_a();
    let b = word_matrix.count_xmas_b();

    Ok((a.to_string(), b.to_string()))
}

struct WordMatrix {
    matrix: Array2<char>,
}

impl FromStr for WordMatrix {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut rows = 0;
        let v: Vec<char> = s
            .lines()
            .inspect(|_| rows += 1)
            .map(&str::chars)
            .flatten()
            .collect();

        let cols = v.len() / rows;
        let matrix = Array2::from_shape_vec((rows, cols), v)?;
        Ok(WordMatrix { matrix })
    }
}

impl WordMatrix {
    pub fn count_xmas_a(&self) -> i32 {
        fn check(view: &ArrayView1<char>) -> bool {
            view[0] == 'X' && view[1] == 'M' && view[2] == 'A' && view[3] == 'S'
        }

        let rows_sum = self.window_sum((1, 4), |window| {
            check(&window.row(0)) || check(&window.row(0).slice(s![..;-1]))
        });

        let cols_sum = self.window_sum((4, 1), |window| {
            check(&window.column(0)) || check(&window.column(0).slice(s![..;-1]))
        });

        let diags_sum = self.window_sum((4, 4), |window| {
            check(&window.diag())
                || check(&window.slice(s![..,..;-1]).diag())
                || check(&window.slice(s![..;-1,..]).diag())
                || check(&window.slice(s![..;-1,..;-1]).diag())
        });

        rows_sum + cols_sum + diags_sum
    }

    pub fn count_xmas_b(&self) -> i32 {
        fn check(view: &ArrayView2<char>) -> bool {
            view[(1, 1)] == 'A'
                && view[(0, 0)] == 'M'
                && view[(2, 0)] == 'M'
                && view[(0, 2)] == 'S'
                && view[(2, 2)] == 'S'
        }

        self.window_sum((3, 3), |window| {
            check(window)
                || check(&window.slice(s![..,..;-1]))
                || check(&window.t())
                || check(&window.t().slice(s![..,..;-1]))
        })
    }

    fn window_sum<F>(&self, window_size: (usize, usize), f: F) -> i32
    where
        F: Fn(&ArrayView2<char>) -> bool,
    {
        self.matrix
            .windows(window_size)
            .into_iter()
            .filter(f)
            .count() as i32
    }
}
