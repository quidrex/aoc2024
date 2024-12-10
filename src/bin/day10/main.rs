use aoc2024::util::parse_matrix::ToMatrix;
use aoc2024::{aoc_day, AocDay};
use ndarray::Array2;
use std::collections::HashMap;
use std::iter::Iterator;

aoc_day!(Day10, 36, 81);

struct Day10 {
    trail_map: Array2<u32>,
}

impl AocDay for Day10 {
    fn from(input: &str) -> Self {
        let data = input.parse_matrix(|c| c.to_digit(10).unwrap());
        Day10 { trail_map: data }
    }

    fn a(&self) -> i64 {
        self.find_trailheads().0 as i64
    }

    fn b(&self) -> i64 {
        self.find_trailheads().1 as i64
    }
}

impl Day10 {
    pub fn find_trailheads(&self) -> (usize, usize) {
        self.trail_map
            .indexed_iter()
            .filter(|(_, &v)| v == 0)
            .map(|(idx, _)| {
                let mut acc: HashMap<[usize; 2], usize> = HashMap::new();
                self.bfs(&mut acc, idx.into(), 1);
                let score = acc.keys().len();
                let rating: usize = acc.values().sum();
                (score, rating)
            })
            .fold((0, 0), |(a0, a1), (b0, b1)| (a0 + b0, a1 + b1))
    }

    fn bfs(&self, acc: &mut HashMap<[usize; 2], usize>, idx: [usize; 2], value: u32) {
        let neighbor_idxs = [
            [idx[0].wrapping_sub(1), idx[1]],
            [idx[0] + 1, idx[1]],
            [idx[0], idx[1].wrapping_sub(1)],
            [idx[0], idx[1] + 1],
        ];

        neighbor_idxs
            .iter()
            .filter(|&&idx| self.trail_map.get(idx) == Some(&value))
            .for_each(|&idx| {
                if value == 9 {
                    acc.entry(idx).and_modify(|e| *e += 1).or_insert(1);
                } else {
                    self.bfs(acc, idx, value + 1)
                }
            });
    }
}
