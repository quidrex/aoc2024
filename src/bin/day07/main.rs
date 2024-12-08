use anyhow::{Error, Result};
use aoc2024::init;
use std::iter::Iterator;
use std::str::FromStr;
use strum::{EnumIter, IntoEnumIterator};

init!("3749", "11387");

fn run(input: &str) -> Result<(String, String)> {
    let data = input.parse::<Data>()?;

    let a = data.calc(false);
    let b = data.calc(true);

    Ok((a.to_string(), b.to_string()))
}

struct Data {
    equations: Vec<(i64, Vec<i64>)>,
}

#[derive(Eq, EnumIter, PartialEq)]
enum Op {
    ADD,
    MUL,
    CONCAT,
}

impl FromStr for Data {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let equations = s
            .lines()
            .map(|line| {
                let (l, r) = line.split_once(": ").unwrap();
                (
                    l.parse::<i64>().unwrap(),
                    r.split(" ")
                        .map(|elem| elem.parse::<i64>().unwrap())
                        .collect(),
                )
            })
            .collect();
        Ok(Data { equations })
    }
}

impl Data {
    pub fn calc(&self, allow_concat: bool) -> i64 {
        self.equations
            .iter()
            .map(|equation| {
                let results = Self::do_calc(
                    &equation.1[0..equation.1.len() - 1],
                    equation.1[equation.1.len() - 1],
                    allow_concat,
                );
                if results.contains(&equation.0) {
                    equation.0
                } else {
                    0
                }
            })
            .sum()
    }

    pub fn do_calc(l: &[i64], r: i64, allow_concat: bool) -> Vec<i64> {
        let ls = if l.len() > 1 {
            Self::do_calc(&l[0..l.len() - 1], l[l.len() - 1], allow_concat)
        } else {
            vec![l[0]]
        };
        ls.iter()
            .map(|l| {
                Op::iter()
                    .filter(|op| allow_concat || op != &Op::CONCAT)
                    .map(move |op| match op {
                        Op::ADD => l + r,
                        Op::MUL => l * r,
                        Op::CONCAT => (l.to_string() + &r.to_string()).parse::<i64>().unwrap(),
                    })
            })
            .flatten()
            .collect()
    }
}
