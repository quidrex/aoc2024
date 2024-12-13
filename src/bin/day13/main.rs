use aoc2024::{aoc_day, AocDay};
use regex::Regex;
use std::iter::Iterator;
use std::sync::LazyLock;

aoc_day!(Day13, 480);

struct Day13 {
    machines: Vec<Machine>,
}

#[derive(Copy, Clone)]
struct Machine {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize: (i64, i64),
}

static INPUT_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)",
    )
    .unwrap()
});

impl AocDay for Day13 {
    fn from(input: &str) -> Self {
        let machines = INPUT_REGEX
            .captures_iter(input)
            .map(|caps| {
                let ax = caps.get(1).unwrap().as_str().parse::<i64>().unwrap();
                let ay = caps.get(2).unwrap().as_str().parse::<i64>().unwrap();
                let bx = caps.get(3).unwrap().as_str().parse::<i64>().unwrap();
                let by = caps.get(4).unwrap().as_str().parse::<i64>().unwrap();
                let px = caps.get(5).unwrap().as_str().parse::<i64>().unwrap();
                let py = caps.get(6).unwrap().as_str().parse::<i64>().unwrap();

                Machine {
                    button_a: (ax, ay),
                    button_b: (bx, by),
                    prize: (px, py),
                }
            })
            .collect::<Vec<_>>();

        Day13 { machines }
    }

    fn a(&self) -> i64 {
        self.optimize(0)
    }

    fn b(&self) -> i64 {
        self.optimize(10_000_000_000_000)
    }
}

impl Day13 {
    pub fn optimize(&self, prize_offset: i64) -> i64 {
        self.machines
            .iter()
            .map(|machine| {
                let offset_machine = Machine {
                    prize: (
                        machine.prize.0 + prize_offset,
                        machine.prize.1 + prize_offset,
                    ),
                    ..*machine
                };
                Self::optimize_machine(&offset_machine)
            })
            .sum()
    }

    pub fn optimize_machine(machine: &Machine) -> i64 {
        let t = machine.button_a.1 as f64 / machine.button_a.0 as f64;
        let b = (machine.prize.1 as f64 - t * machine.prize.0 as f64)
            / (machine.button_b.1 as f64 - t * machine.button_b.0 as f64);
        let a =
            (machine.prize.0 as f64 - machine.button_b.0 as f64 * b) / machine.button_a.0 as f64;

        let a_rounded = a.round() as i64;
        let b_rounded = b.round() as i64;

        if a_rounded * machine.button_a.0 + b_rounded * machine.button_b.0 == machine.prize.0
            && a_rounded * machine.button_a.1 + b_rounded * machine.button_b.1 == machine.prize.1
        {
            3 * a_rounded + b_rounded
        } else {
            0
        }
    }
}
