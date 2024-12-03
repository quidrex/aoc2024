use anyhow::{Error, Result};
use aoc2024::init;
use regex::Regex;
use std::str::FromStr;
use std::sync::LazyLock;
use strum::EnumString;

init!("161", "48");

fn run(input: &str) -> Result<(String, String)> {
    let program = input.parse::<Program>()?;

    let a = program.run_only_mul();
    let b = program.run();

    Ok((a.to_string(), b.to_string()))
}

struct Program {
    instructions: Vec<Instruction>,
}

#[derive(EnumString)]
#[strum(serialize_all = "lowercase")]
enum Instruction {
    Mul(i32, i32),
    Do,
    #[strum(serialize = "don't")]
    Dont,
}

static RE_INPUT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(mul|do|don't)\((?:(\d+),(\d+))?\)").unwrap());

impl FromStr for Program {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let instructions = RE_INPUT
            .captures_iter(s)
            .filter_map(|caps| {
                let instruction: Instruction = caps.get(1).unwrap().as_str().parse().unwrap();

                match instruction {
                    Instruction::Mul(_, _) => {
                        let a = caps.get(2)?.as_str().parse::<i32>().ok()?;
                        let b = caps.get(3)?.as_str().parse::<i32>().ok()?;
                        Some(Instruction::Mul(a, b))
                    }
                    instruction => Some(instruction),
                }
            })
            .collect();

        Ok(Program { instructions })
    }
}

impl Program {
    pub fn run_only_mul(&self) -> i32 {
        self.do_run(true)
    }

    pub fn run(&self) -> i32 {
        self.do_run(false)
    }

    fn do_run(&self, ignore_do: bool) -> i32 {
        let mut acc = 0;
        let mut mul_active = true;

        for instruction in &self.instructions {
            match instruction {
                Instruction::Mul(a, b) => {
                    if mul_active {
                        acc += a * b
                    }
                }
                Instruction::Do => {
                    if !ignore_do {
                        mul_active = true
                    }
                }
                Instruction::Dont => {
                    if !ignore_do {
                        mul_active = false
                    }
                }
            }
        }

        acc
    }
}
