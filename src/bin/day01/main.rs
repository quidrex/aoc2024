use anyhow::{Context, Error, Result};
use aoc2024::init;
use itertools::process_results;
use regex::Regex;
use std::str::FromStr;
use std::sync::LazyLock;

init!("11", "31");

fn run(input: &str) -> Result<(String, String)> {
    let lists = input.parse::<Lists>()?;

    let a = lists.total_distance();
    let b = lists.similarity_score();

    Ok((a.to_string(), b.to_string()))
}

struct Lists {
    pub left: Vec<i32>,
    pub right: Vec<i32>,
}

static RE_INPUT: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(\d+)\s+(\d+)$").unwrap());

impl FromStr for Lists {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let results_iter = s.lines().map(|line| {
            let caps = RE_INPUT.captures(line).context("input regex match error")?;
            let a = caps.get(1).unwrap().as_str().parse::<i32>()?;
            let b = caps.get(2).unwrap().as_str().parse::<i32>()?;

            Ok::<(i32, i32), Error>((a, b))
        });
        let (mut left, mut right): (Vec<i32>, Vec<i32>) =
            process_results(results_iter, |iter| iter.unzip())?;

        left.sort();
        right.sort();

        Ok(Lists { left, right })
    }
}

impl Lists {
    fn total_distance(&self) -> i32 {
        self.left
            .iter()
            .zip(&self.right)
            .map(|(l, r)| (l - r).abs())
            .sum()
    }

    fn similarity_score(&self) -> i32 {
        self.left
            .iter()
            .map(|l| self.right.iter().filter(|r| *r == l).count() as i32 * l)
            .sum()
    }
}
