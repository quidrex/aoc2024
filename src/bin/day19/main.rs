use aoc2024::{aoc_day, AocDay};
use enum_map::EnumMap;
use itertools::Itertools;
use std::collections::HashMap;
use std::str::FromStr;
use strum::EnumString;

aoc_day!(Input, "6", "16");

#[derive(Clone)]
struct Input {
    towels: Vec<Vec<Stripe>>,
    patterns: Vec<Vec<Stripe>>,
}

#[derive(Copy, Clone, Eq, PartialEq, EnumString, enum_map::Enum)]
enum Stripe {
    #[strum(serialize = "w")]
    White,
    #[strum(serialize = "u")]
    Blue,
    #[strum(serialize = "b")]
    Black,
    #[strum(serialize = "r")]
    Red,
    #[strum(serialize = "g")]
    Green,
}

impl AocDay for Input {
    fn from(input: &str) -> Self {
        fn to_stripe(c: char) -> Stripe {
            let mut buf = [0u8; 1];
            Stripe::from_str(c.encode_utf8(&mut buf)).unwrap()
        }

        let (towels_str, patterns_str) = input.trim_end().split_once("\n\n").unwrap();
        let towels = towels_str
            .split(", ")
            .map(|towel_str| towel_str.chars().map(to_stripe).collect_vec())
            .collect_vec();
        let patterns = patterns_str
            .split("\n")
            .map(|pattern_str| pattern_str.chars().map(to_stripe).collect_vec())
            .collect_vec();

        Input { towels, patterns }
    }

    fn a(&self) -> String {
        let mut nfa = Nfa::new(&self);
        self.patterns
            .iter()
            .filter(|pattern| nfa.accepts(pattern) > 0)
            .count()
            .to_string()
    }

    fn b(&self) -> String {
        let mut nfa = Nfa::new(&self);
        self.patterns
            .iter()
            .map(|pattern| nfa.accepts(pattern))
            .sum::<usize>()
            .to_string()
    }
}

struct Nfa {
    transitions: Vec<EnumMap<Stripe, Vec<usize>>>,
    current_states: HashMap<usize, usize>,
}

impl Nfa {
    fn new(input: &Input) -> Nfa {
        let mut transitions: Vec<EnumMap<Stripe, Vec<usize>>> = vec![EnumMap::default()];
        let mut state_idx = 1;

        for towel in &input.towels {
            if towel.len() == 1 {
                transitions[0][towel[0]].push(0);
                continue;
            }

            transitions[0][towel[0]].push(state_idx);

            for &stripe in &towel[1..towel.len() - 1] {
                transitions.push(EnumMap::default());
                transitions[state_idx][stripe].push(state_idx + 1);
                state_idx += 1;
            }

            transitions.push(EnumMap::default());
            transitions[state_idx][towel[towel.len() - 1]].push(0);
            state_idx += 1;
        }

        let mut current_states = HashMap::new();
        current_states.insert(0, 1);

        Nfa {
            transitions,
            current_states,
        }
    }

    fn accepts(&mut self, stripes: &Vec<Stripe>) -> usize {
        for &stripe in stripes {
            self.accept(stripe);
        }

        let accepts = *self.current_states.get(&0).unwrap_or(&0);
        self.current_states.clear();
        self.current_states.insert(0, 1);
        accepts
    }

    fn accept(&mut self, stripe: Stripe) {
        let mut next_states = HashMap::new();

        for (&current_state, &current_state_count) in &self.current_states {
            for &next_state in &self.transitions[current_state][stripe] {
                *next_states.entry(next_state).or_insert(0) += current_state_count;
            }
        }

        self.current_states = next_states;
    }
}
