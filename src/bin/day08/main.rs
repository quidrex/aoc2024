use anyhow::{Error, Result};
use aoc2024::init;
use itertools::Itertools;
use num::integer::gcd;
use std::collections::{HashMap, HashSet};
use std::iter::Iterator;
use std::str::FromStr;

init!("14", "34");

fn run(input: &str) -> Result<(String, String)> {
    let state = input.parse::<State>()?;

    let a = state.find_antinodes();
    let b = state.find_antinodes_resonant();

    Ok((a.to_string(), b.to_string()))
}

struct State {
    size: (usize, usize),
    antennas: HashMap<char, Vec<(usize, usize)>>,
}

impl FromStr for State {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
        let mut size = (0, 0);

        for (row, line) in s.lines().enumerate() {
            for (col, c) in line.chars().enumerate() {
                if c != '.' {
                    antennas.entry(c).or_default().push((row, col));
                }
                size = (row + 1, col + 1);
            }
        }

        Ok(State { size, antennas })
    }
}

impl State {
    pub fn find_antinodes(&self) -> i32 {
        let mut antinodes = HashSet::new();

        for frequency in self.antennas.values() {
            for (antenna_a, antenna_b) in frequency.iter().tuple_combinations() {
                let diff = (
                    antenna_b.0 as isize - antenna_a.0 as isize,
                    antenna_b.1 as isize - antenna_a.1 as isize,
                );
                let antinode_a = (
                    antenna_a.0.wrapping_add_signed(-diff.0),
                    antenna_a.1.wrapping_add_signed(-diff.1),
                );
                let antinode_b = (
                    antenna_b.0.wrapping_add_signed(diff.0),
                    antenna_b.1.wrapping_add_signed(diff.1),
                );

                if antinode_a.0 < self.size.0 && antinode_a.1 < self.size.1 {
                    antinodes.insert(antinode_a);
                }
                if antinode_b.0 < self.size.0 && antinode_b.1 < self.size.1 {
                    antinodes.insert(antinode_b);
                }
            }
        }

        antinodes.len() as i32
    }

    pub fn find_antinodes_resonant(&self) -> i32 {
        let mut antinodes = HashSet::new();

        for frequency in self.antennas.values() {
            for (antenna_a, antenna_b) in frequency.iter().tuple_combinations() {
                let diff_unscaled = (
                    antenna_b.0 as isize - antenna_a.0 as isize,
                    antenna_b.1 as isize - antenna_a.1 as isize,
                );
                let diff_gcd = gcd(diff_unscaled.0, diff_unscaled.1);
                let diff = (diff_unscaled.0 / diff_gcd, diff_unscaled.1 / diff_gcd);

                let mut i = 0;
                let mut i_step = -1;
                loop {
                    let antinode = (
                        antenna_a.0.wrapping_add_signed(i * diff.0),
                        antenna_a.1.wrapping_add_signed(i * diff.1),
                    );

                    if antinode.0 < self.size.0 && antinode.1 < self.size.1 {
                        antinodes.insert(antinode);
                        i += i_step;
                    } else {
                        if i_step == -1 {
                            i = 1;
                            i_step = 1;
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        antinodes.len() as i32
    }
}
