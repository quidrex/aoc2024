use anyhow::{ensure, Error, Result};
use aoc2024::init;
use std::str::FromStr;

init!("2", "4");

fn run(input: &str) -> Result<(String, String)> {
    let reports = input.parse::<Reports>()?;

    let a = reports.count_safe();
    let b = reports.count_safe_dampened();

    Ok((a.to_string(), b.to_string()))
}

struct Reports {
    reports: Vec<Vec<i32>>,
}

impl FromStr for Reports {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let reports: Vec<Vec<i32>> = s
            .lines()
            .map(|line| {
                let report: Vec<i32> = line
                    .split(' ')
                    .map(|item| item.parse::<i32>())
                    .collect::<Result<_, _>>()?;
                ensure!(report.len() >= 3);
                Ok(report)
            })
            .collect::<Result<_, _>>()?;

        Ok(Reports { reports })
    }
}

impl Reports {
    pub fn count_safe(&self) -> i32 {
        self.reports.iter().filter(|report| Self::is_safe(&report)).count() as i32
    }

    pub fn count_safe_dampened(&self) -> i32 {
        self.reports
            .iter()
            .filter(|report| {
                if Self::is_safe(&report) {
                    return true;
                }

                (0..report.len()).any(|skip_idx| {
                    let dampened_report: Vec<i32> = report
                        .iter()
                        .enumerate()
                        .filter(|(idx, _)| *idx != skip_idx)
                        .map(|(_, e)| *e)
                        .collect();
                    Self::is_safe(&dampened_report)
                })
            })
            .count() as i32
    }

    fn is_safe(report: &[i32]) -> bool {
        let sign = if report[1] > report[0] { 1 } else { -1 };
        report.windows(2).all(|window| {
            let delta = (window[1] - window[0]) * sign;
            delta >= 1 && delta <= 3
        })
    }
}
