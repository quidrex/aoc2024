use anyhow::{Error, Result};
use aoc2024::init;
use itertools::Itertools;
use ndarray::Array2;
use std::iter::Iterator;
use std::str::FromStr;

init!("41", "6");

fn run(input: &str) -> Result<(String, String)> {
    let mut lab = input.parse::<Lab>()?;

    let a = lab.calc_a();
    let b = lab.calc_b();

    Ok((a.to_string(), b.to_string()))
}

#[derive(Clone)]
struct Lab {
    grid: Array2<Position>,
    guard: ((usize, usize), (isize, isize)),
    path: Vec<((usize, usize), (isize, isize))>
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Position {
    EMPTY,
    GUARD,
    OBSTRUCTED,
}

impl FromStr for Lab {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut rows = 0;
        let v: Vec<Position> = s
            .lines()
            .inspect(|_| rows += 1)
            .map(|line| {
                line.chars().map(|elem| match elem {
                    '.' => Position::EMPTY,
                    '^' => Position::GUARD,
                    '#' => Position::OBSTRUCTED,
                    _ => panic!(),
                })
            })
            .flatten()
            .collect();
        let cols = v.len() / rows;

        let grid = Array2::from_shape_vec((rows, cols), v)?;
        let guard_pos = grid
            .indexed_iter()
            .find(|(_, position)| position == &&Position::GUARD)
            .unwrap()
            .0;

        Ok(Lab {
            grid,
            guard: (guard_pos, (-1, 0)),
            path: vec![(guard_pos, (-1, 0))],
        })
    }
}

impl Lab {
    pub fn calc_a(&mut self) -> i32 {
        self.walk().unwrap()
    }

    pub fn calc_b(&mut self) -> i32 {
        self.path.iter().map(|(pos, _)| pos).unique().filter(|&&obstacle| {
            if self.grid[obstacle] == Position::EMPTY {
                let mut obstacled_lab = self.clone();
                obstacled_lab.grid[obstacle] = Position::OBSTRUCTED;
                obstacled_lab.guard = self.path[0];
                obstacled_lab.path = vec![obstacled_lab.guard];
                obstacled_lab.walk().is_none()
            } else {
                false
            }
        } ).count() as i32
    }

    fn walk(&mut self) -> Option<i32> {
        let dim = self.grid.dim();

        loop {
            let next_pos = (
                self.guard.0.0.wrapping_add_signed(self.guard.1.0),
                self.guard.0.1.wrapping_add_signed(self.guard.1.1),
            );

            if next_pos.0 >= dim.0 || next_pos.1 >= dim.1 {
                return Some(self.path.iter().map(|(pos, _)| pos).unique().count() as i32);
            }

            if &self.grid[next_pos] == &Position::OBSTRUCTED {
                self.guard.1 = (self.guard.1.1, self.guard.1.0 * -1);
            } else {
                self.guard.0 = next_pos;
                if self.path.contains(&self.guard) {
                    return None;
                }
                self.path.push(self.guard);
            }
        }
    }
}
