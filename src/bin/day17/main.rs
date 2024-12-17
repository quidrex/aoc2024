use aoc2024::{aoc_day, AocDay};
use itertools::Itertools;
use regex::Regex;
use std::ops::BitXor;
use std::sync::LazyLock;
use strum::FromRepr;

aoc_day!(Day17, "4,6,3,5,6,3,5,2,1,0", "117440");

#[derive(Clone)]
struct Day17 {
    rom: Vec<u8>,
    r: [u64; 3],
    ip: usize,
}

#[derive(Copy, Clone, FromRepr)]
enum Instruction {
    Adv = 0,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

static INPUT_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"Register A: (\d+)\nRegister B: (\d+)\nRegister C: (\d+)\n\nProgram: ([\d,]+)")
        .unwrap()
});

impl AocDay for Day17 {
    fn from(input: &str) -> Self {
        let caps = INPUT_REGEX.captures(input).unwrap();

        let r = [
            caps.get(1).unwrap().as_str().parse::<u64>().unwrap(),
            caps.get(2).unwrap().as_str().parse::<u64>().unwrap(),
            caps.get(3).unwrap().as_str().parse::<u64>().unwrap(),
        ];

        let rom = caps
            .get(4)
            .unwrap()
            .as_str()
            .split(',')
            .map(|ins| ins.parse::<u8>().unwrap())
            .collect::<Vec<_>>();

        Day17 { rom, r, ip: 0 }
    }

    fn a(&self) -> String {
        let mut clone = self.clone();
        clone.run().iter().join(",")
    }

    fn b(&self) -> String {
        let mut clone = self.clone();
        let mut ra = 0u64;

        for &ins in self.rom.iter().rev() {
            ra <<= 3;
            loop {
                clone.reset([ra, self.r[1], self.r[2]]);
                let output = clone.run();
                if output[0] == ins {
                    break;
                }
                ra += 1;
            }
        }

        ra.to_string()
    }
}

impl Day17 {
    pub fn reset(&mut self, r: [u64; 3]) {
        self.r = r;
        self.ip = 0;
    }

    pub fn run(&mut self) -> Vec<u8> {
        let mut outputs = vec![];
        while self.ip < self.rom.len() {
            if let Some(output) = self.step() {
                outputs.push(output);
            }
        }
        outputs
    }

    fn step(&mut self) -> Option<u8> {
        let ins = Instruction::from_repr(self.rom[self.ip] as usize).unwrap();
        let op = self.rom[self.ip + 1];
        let mut output = None;

        match ins {
            Instruction::Adv => {
                self.r[0] = self.r[0] >> self.combo_op(op);
            }
            Instruction::Bxl => {
                self.r[1] = self.r[1].bitxor(op as u64);
            }
            Instruction::Bst => {
                self.r[1] = self.combo_op(op) % 8;
            }
            Instruction::Jnz => {
                if self.r[0] != 0 {
                    self.ip = (op / 2) as usize;
                    return None;
                }
            }
            Instruction::Bxc => {
                self.r[1] = self.r[1].bitxor(self.r[2]);
            }
            Instruction::Out => {
                output = Some((self.combo_op(op) % 8) as u8);
            }
            Instruction::Bdv => {
                self.r[1] = self.r[0] >> self.combo_op(op);
            }
            Instruction::Cdv => {
                self.r[2] = self.r[0] >> self.combo_op(op);
            }
        }

        self.ip += 2;
        output
    }

    fn combo_op(&self, op: u8) -> u64 {
        match op {
            0..=3 => op as u64,
            4..6 => self.r[(op - 4) as usize],
            _ => panic!(),
        }
    }
}
