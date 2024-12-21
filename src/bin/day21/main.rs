use aoc2024::util::grid::Coord;
use aoc2024::{aoc_day, AocDay};
use arrayvec::ArrayVec;
use itertools::Itertools;
use std::collections::HashMap;
use std::str::FromStr;
use strum::EnumString;

aoc_day!(Input, "126384");

#[derive(Clone)]
struct Input {
    codes: Vec<ArrayVec<NumericButton, 4>>,
}

impl AocDay for Input {
    fn from(input: &str) -> Self {
        fn to_numeric_button(c: char) -> NumericButton {
            let mut buf = [0u8; 1];
            NumericButton::from_str(c.encode_utf8(&mut buf)).unwrap()
        }

        let codes = input
            .trim_end()
            .split("\n")
            .map(|code_str| {
                code_str
                    .chars()
                    .map(to_numeric_button)
                    .collect::<ArrayVec<NumericButton, 4>>()
            })
            .collect_vec();

        Input { codes }
    }

    fn a(&self) -> String {
        self.complexity_sum(2).to_string()
    }

    fn b(&self) -> String {
        /*for i in 2..=25 {
            print!("{} ", self.complexity_sum(i).to_string())
        }
        println!();*/

        self.complexity_sum(25).to_string()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Default, Hash, strum::Display, EnumString)]
enum NumericButton {
    #[strum(serialize = "7")]
    Digit7,
    #[strum(serialize = "8")]
    Digit8,
    #[strum(serialize = "9")]
    Digit9,
    #[strum(serialize = "4")]
    Digit4,
    #[strum(serialize = "5")]
    Digit5,
    #[strum(serialize = "6")]
    Digit6,
    #[strum(serialize = "1")]
    Digit1,
    #[strum(serialize = "2")]
    Digit2,
    #[strum(serialize = "3")]
    Digit3,
    #[strum(serialize = "0")]
    Digit0 = 10,
    #[strum(serialize = "A")]
    #[default]
    Accept,
}

#[derive(Copy, Clone, Eq, PartialEq, Default, Hash, strum::Display, EnumString, Debug)]
enum DirectionalButton {
    #[strum(serialize = "^")]
    Up = 1,
    #[strum(serialize = "A")]
    #[default]
    Accept,
    #[strum(serialize = "<")]
    Left,
    #[strum(serialize = "v")]
    Down,
    #[strum(serialize = ">")]
    Right,
}

impl NumericButton {
    fn number(buttons: &ArrayVec<NumericButton, 4>) -> u64 {
        let mut output = String::with_capacity(3);

        for &button in &buttons[0..3] {
            if button != NumericButton::Accept {
                output += &button.to_string();
            }
        }

        u64::from_str(&output).unwrap()
    }
}

trait Button: Default {
    fn pos(self) -> Coord;
    fn can_void(self, other: Self) -> bool;
    fn order_to(self, other: Self) -> [DirectionalButton; 4];
}

impl Button for NumericButton {
    fn pos(self) -> Coord {
        Coord((self as isize) / 3, (self as isize) % 3)
    }

    fn can_void(self, other: NumericButton) -> bool {
        match self {
            NumericButton::Accept | NumericButton::Digit0 => match other {
                NumericButton::Digit7 | NumericButton::Digit4 | NumericButton::Digit1 => true,
                _ => false,
            },
            NumericButton::Digit7 | NumericButton::Digit4 | NumericButton::Digit1 => match other {
                NumericButton::Accept | NumericButton::Digit0 => true,
                _ => false,
            },
            _ => false,
        }
    }

    fn order_to(self, other: Self) -> [DirectionalButton; 4] {
        if self.can_void(other) {
            [
                DirectionalButton::Up,
                DirectionalButton::Right,
                DirectionalButton::Left,
                DirectionalButton::Down,
            ]
        } else {
            [
                DirectionalButton::Left,
                DirectionalButton::Up,
                DirectionalButton::Down,
                DirectionalButton::Right,
            ]
        }
    }
}

impl Button for DirectionalButton {
    fn pos(self) -> Coord {
        Coord((self as isize) / 3, (self as isize) % 3)
    }

    fn can_void(self, other: DirectionalButton) -> bool {
        match self {
            DirectionalButton::Left => match other {
                DirectionalButton::Accept | DirectionalButton::Up => true,
                _ => false,
            },
            DirectionalButton::Accept | DirectionalButton::Up => match other {
                DirectionalButton::Left => true,
                _ => false,
            },
            _ => false,
        }
    }

    fn order_to(self, other: Self) -> [DirectionalButton; 4] {
        if self.can_void(other) {
            [
                DirectionalButton::Right,
                DirectionalButton::Down,
                DirectionalButton::Left,
                DirectionalButton::Up,
            ]
        } else {
            [
                DirectionalButton::Left,
                DirectionalButton::Up,
                DirectionalButton::Down,
                DirectionalButton::Right,
            ]
        }
    }
}

impl Input {
    fn complexity_sum(&self, directional_pads: u32) -> u64 {
        let mut complexity = 0;

        for code in &self.codes {
            let mut sequences = Self::expand(Self::init(code.clone()));

            for _i in 0..directional_pads {
                sequences = Self::expand(sequences);
            }
            let len = sequences
                .iter()
                .map(|(sequence, &sequence_count)| sequence.len() as u64 * sequence_count)
                .sum::<u64>();
            complexity += len * NumericButton::number(code);
        }

        complexity
    }

    fn init(code: ArrayVec<NumericButton, 4>) -> HashMap<ArrayVec<NumericButton, 4>, u64> {
        let mut sequences = HashMap::new();
        sequences.insert(code, 1);
        sequences
    }

    fn expand<T: Button + Copy, const N: usize>(
        sequences: HashMap<ArrayVec<T, N>, u64>,
    ) -> HashMap<ArrayVec<DirectionalButton, 5>, u64> {
        let mut expanded_sequences = HashMap::new();
        let mut last_button = T::default();

        for (&ref sequence, &sequence_count) in sequences.iter() {
            for &button in sequence {
                let mut expanded_sequence = ArrayVec::<_, 5>::new();

                let last_button_pos = last_button.pos();
                let button_pos = button.pos();

                for dir in last_button.order_to(button) {
                    match dir {
                        DirectionalButton::Up => Self::push_directions(
                            &mut expanded_sequence,
                            DirectionalButton::Up,
                            last_button_pos.0,
                            button_pos.0,
                        ),
                        DirectionalButton::Left => Self::push_directions(
                            &mut expanded_sequence,
                            DirectionalButton::Left,
                            last_button_pos.1,
                            button_pos.1,
                        ),
                        DirectionalButton::Down => Self::push_directions(
                            &mut expanded_sequence,
                            DirectionalButton::Down,
                            button_pos.0,
                            last_button_pos.0,
                        ),
                        DirectionalButton::Right => Self::push_directions(
                            &mut expanded_sequence,
                            DirectionalButton::Right,
                            button_pos.1,
                            last_button_pos.1,
                        ),
                        _ => panic!(),
                    }
                }
                expanded_sequence.push(DirectionalButton::Accept);
                //println!("{:?}", expanded_sequence);
                *expanded_sequences.entry(expanded_sequence).or_insert(0) += sequence_count;
                last_button = button;
            }
        }

        expanded_sequences
    }

    /* fn type_sequence<T: Button + Copy>(code: &Vec<T>) -> Vec<DirectionalButton> {
        let mut sequence = vec![];
        let mut last_button = T::default();

        for &button in code {
            let last_button_pos = last_button.pos();
            let button_pos = button.pos();

            for dir in last_button.order_to(button) {
                match dir {
                    DirectionalButton::Up => Self::push_directions(
                        &mut sequence,
                        DirectionalButton::Up,
                        last_button_pos.0,
                        button_pos.0,
                    ),
                    DirectionalButton::Left => Self::push_directions(
                        &mut sequence,
                        DirectionalButton::Left,
                        last_button_pos.1,
                        button_pos.1,
                    ),
                    DirectionalButton::Down => Self::push_directions(
                        &mut sequence,
                        DirectionalButton::Down,
                        button_pos.0,
                        last_button_pos.0,
                    ),
                    DirectionalButton::Right => Self::push_directions(
                        &mut sequence,
                        DirectionalButton::Right,
                        button_pos.1,
                        last_button_pos.1,
                    ),
                    _ => panic!(),
                }
            }

            sequence.push(DirectionalButton::Accept);
            last_button = button;
        }
        sequence
    }*/

    fn push_directions(
        sequence: &mut ArrayVec<DirectionalButton, 5>,
        button: DirectionalButton,
        greater: isize,
        less: isize,
    ) {
        if greater > less {
            for _ in 0..(greater - less) {
                sequence.push(button);
            }
        }
    }
}
