use aoc2024::{aoc_day, AocDay};
use itertools::Itertools;
use std::ops::BitXor;

aoc_day!(Input, "37327623", "23");

const MODULUS: i64 = 16777216;

#[derive(Clone)]
struct Input {
    seeds: Vec<i64>,
}

impl AocDay for Input {
    fn from(input: &str) -> Self {
        let seeds = input
            .trim_end()
            .split("\n")
            .map(|seed_str| seed_str.parse::<i64>().unwrap())
            .collect_vec();
        Input { seeds }
    }

    fn a(&self) -> String {
        self.get_number_sum().to_string()
    }

    fn b(&self) -> String {
        self.get_max_price_sum().to_string()
    }
}

impl Input {
    fn get_number_sum(&self) -> i64 {
        self.seeds
            .iter()
            .map(|&seed| Self::generate_number(seed, 2000))
            .sum::<i64>()
    }

    fn generate_number(seed: i64, n: i32) -> i64 {
        let mut number = seed;
        for _ in 0..n {
            number = number.bitxor(number * 64) % MODULUS;
            number = number.bitxor(number / 32) % MODULUS;
            number = number.bitxor(number * 2048) % MODULUS;
        }
        number
    }

    fn get_max_price_sum(&self) -> i64 {
        let seeds_prices = self
            .seeds
            .iter()
            .map(|&seed| Self::generate_prices(seed, 2000))
            .collect_vec();

        let seeds_changes = seeds_prices
            .iter()
            .map(|prices| prices.windows(2).map(|w| w[1] - w[0] + 9).collect_vec())
            .collect_vec();

        let mut prices = vec![0i64; 19 * 19 * 19 * 19];
        let mut visited = vec![false; 19 * 19 * 19 * 19];

        for seed_idx in 0..self.seeds.len() {
            visited.clear();
            visited.resize(19 * 19 * 19 * 19, false);

            for changes_idx in 3..seeds_changes[0].len() {
                let changes = &seeds_changes[seed_idx][changes_idx - 3..=changes_idx];
                let idx = (changes[0] as usize) * 19 * 19 * 19
                    + (changes[1] as usize) * 19 * 19
                    + (changes[2] as usize) * 19
                    + (changes[3] as usize);
                if !visited[idx] {
                    prices[idx] += seeds_prices[seed_idx][changes_idx+1] as i64;
                    visited[idx] = true;
                }
            }
        }

        *prices.iter().max().unwrap()
    }

    fn generate_prices(seed: i64, n: i32) -> Vec<i8> {
        let mut prices = vec![(seed % 10) as i8];

        let mut number = seed;
        for _ in 0..n {
            number = number.bitxor(number * 64) % MODULUS;
            number = number.bitxor(number / 32) % MODULUS;
            number = number.bitxor(number * 2048) % MODULUS;
            prices.push((number % 10) as i8);
        }

        prices
    }
}
