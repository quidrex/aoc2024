use aoc2024::{aoc_day, parse, AocDay};
use std::collections::HashMap;
use std::iter::Iterator;

aoc_day!(Day11, 55312);

#[derive(Clone)]
struct Day11 {
    stones: Vec<i64>,
}

impl AocDay for Day11 {
    fn from(input: &str) -> Self {
        let stones = parse!([i64 " "])(input.trim_end());
        Day11 { stones }
    }

    fn a(&self) -> i64 {
        let mut state = self.init();

        for _ in 0..25 {
            state.blink();
        }

        state.count() as i64
    }

    fn b(&self) -> i64 {
        let mut state = self.init();

        for _ in 0..75 {
            state.blink();
        }

        state.count() as i64
    }
}

impl Day11 {
    pub fn init(&self) -> Day11State {
        let mut new_stones: HashMap<i64, usize> = HashMap::new();

        for stone in &self.stones {
            *new_stones.entry(*stone).or_insert(0) += 1
        }

        Day11State { stones: new_stones }
    }
}

struct Day11State {
    stones: HashMap<i64, usize>,
}

impl Day11State {
    pub fn blink(&mut self) {
        let mut new_stones: HashMap<i64, usize> = HashMap::new();

        for (stone, count) in &self.stones {
            if *stone == 0 {
                *new_stones.entry(1).or_insert(0) += count
            } else {
                let stone_len = stone.ilog10() + 1;

                if stone_len % 2 == 0 {
                    let divisor = 10_i64.pow(stone_len / 2);
                    *new_stones.entry(stone / divisor).or_insert(0) += count;
                    *new_stones.entry(stone % divisor).or_insert(0) += count;
                } else {
                    *new_stones.entry(stone * 2024).or_insert(0) += count;
                }
            }
        }

        self.stones = new_stones;
    }

    pub fn count(&self) -> usize {
        self.stones.values().sum()
    }
}
