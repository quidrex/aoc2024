use anyhow::{Context, Error, Result};
use aoc2024::init;
use std::iter::Iterator;
use std::str::FromStr;

init!("143", "123");

fn run(input: &str) -> Result<(String, String)> {
    let rules = input.parse::<Rules>()?;

    let a = rules.a();
    let b = rules.b();

    Ok((a.to_string(), b.to_string()))
}

struct Rules {
    ordering: Vec<(i32, i32)>,
    updates: Vec<Vec<i32>>,
}

impl FromStr for Rules {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (input_ordering, input_updates) = s.split_once("\n\n").context("input split error")?;

        let ordering: Vec<(i32, i32)> = input_ordering
            .lines()
            .map(|line| {
                let (l, r) = line.split_once('|').context("input split error")?;
                Ok((l.parse::<i32>()?, r.parse::<i32>()?))
            })
            .collect::<Result<_>>()?;

        let updates: Vec<Vec<i32>> = input_updates
            .lines()
            .map(|line| {
                line.split(',')
                    .map(|elem| elem.parse::<i32>())
                    .collect::<Result<_, _>>()
            })
            .collect::<Result<_, _>>()?;

        Ok(Rules { ordering, updates })
    }
}

impl Rules {
    fn a(&self) -> i32 {
        self.updates
            .iter()
            .map(|update| self.check_update(update).is_none() as i32 * update[update.len() / 2])
            .sum()
    }

    fn b(&self) -> i32 {
        self.updates
            .iter()
            .filter_map(|update| {
                let Some(mut check) = self.check_update(update) else {
                    return None;
                };
                let mut update = update.clone();
                loop {
                    update.swap(check.0, check.1);
                    match self.check_update(&update) {
                        Some(v) => check = v,
                        None => return Some(update[update.len() / 2]),
                    }
                }
            })
            .sum()
    }

    fn check_update(&self, update: &Vec<i32>) -> Option<(usize, usize)> {
        for i in 0..update.len() - 1 {
            for j in i + 1..update.len() {
                if self.ordering.contains(&(update[j], update[i])) {
                    return Some((i, j));
                }
            }
        }
        None
    }
}
