use anyhow::Result;
use aoc2024::init;
use itertools::Itertools;
use std::iter::{repeat, Iterator};

init!("1928", "2858");

fn run(input: &str) -> Result<(String, String)> {
    let disk_map = DiskMap::from_rle(input);
    let a = disk_map.compact().checksum();
    let b = disk_map.defrag().checksum();

    Ok((a.to_string(), b.to_string()))
}

struct DiskMap {
    data: Vec<i32>,
}

impl DiskMap {
    pub fn from_rle(input: &str) -> DiskMap {
        let data = input
            .trim_end()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .chunks(2)
            .into_iter()
            .enumerate()
            .flat_map(|(idx, mut lens)| {
                repeat(idx as i32)
                    .take(lens.next().unwrap() as usize)
                    .chain(repeat(-1).take(lens.next().unwrap_or(0) as usize))
            })
            .collect();

        DiskMap { data }
    }

    fn checksum(&self) -> usize {
        self.data
            .iter()
            .enumerate()
            .map(|(idx, &value)| idx * if value > 0 { value as usize } else { 0 })
            .sum()
    }

    fn compact(&self) -> Self {
        let mut data = self.data.clone();

        let mut dst_idx = 0;
        let mut src_idx = data.len() - 1;

        while dst_idx < src_idx {
            while src_idx > 0 && data[src_idx] == -1 {
                src_idx -= 1;
            }

            if data[dst_idx] == -1 {
                data.swap(dst_idx, src_idx);
                src_idx -= 1;
            }
            dst_idx += 1;
        }

        Self { data }
    }

    fn defrag(&self) -> Self {
        let mut data = self.data.clone();

        let mut src_idx = data.len();

        while src_idx > 0 {
            src_idx -= 1;
            while src_idx > 0 && data[src_idx] == -1 {
                src_idx -= 1;
            }
            let id = data[src_idx];

            let mut src_len = 1;
            while src_idx > 0 && data[src_idx - 1] == id {
                src_idx -= 1;
                src_len += 1;
            }

            let mut dst_idx = 0;
            let mut dst_len = 0;
            while dst_idx < src_idx {
                if data[dst_idx + dst_len] == -1 {
                    dst_len += 1;

                    if dst_len == src_len {
                        for off in 0..src_len {
                            data.swap(dst_idx + off, src_idx + off);
                        }
                        break;
                    }
                } else {
                    dst_idx += dst_len + 1;
                    dst_len = 0;
                }
            }
        }

        Self { data }
    }
}
