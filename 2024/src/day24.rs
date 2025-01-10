use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

use crate::utils::regex;

#[derive(Debug)]
struct Operation {
    a: String,
    b: String,
    op: Op,
    out: String,
}

#[derive(Debug)]
enum Op {
    And,
    Or,
    Xor,
}

#[derive(Debug)]
pub struct ParseResult {
    init_states: HashMap<String, bool>,
    logic: HashMap<String, Operation>,
}

impl std::str::FromStr for Op {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AND" => Ok(Self::And),
            "OR" => Ok(Self::Or),
            "XOR" => Ok(Self::Xor),
            _ => Err(()),
        }
    }
}

impl Operation {
    fn apply(&self, a: bool, b: bool) -> bool {
        match self.op {
            Op::And => a & b,
            Op::Or => a | b,
            Op::Xor => a ^ b,
        }
    }
}

#[aoc_generator(day24)]
pub fn parse_input(input: &str) -> ParseResult {
    let input = input.trim().replace('\r', "");
    let parts = input.split_once("\n\n").unwrap();

    let init_states = parts
        .0
        .lines()
        .map(|line| {
            let value = line.split_once(": ").unwrap();
            (value.0.to_string(), value.1 == "1")
        })
        .collect::<HashMap<String, bool>>();

    let logic_regex = regex!(r"(\w+) (\w+) (\w+) -> (\w+)");
    let mut logic = HashMap::new();

    parts
        .1
        .lines()
        .map(|line| {
            let cap = logic_regex.captures(line).unwrap();
            let op = cap.get(2).unwrap().as_str();
            let a = cap.get(1).unwrap().as_str();
            let b = cap.get(3).unwrap().as_str();
            let out = cap.get(4).unwrap().as_str();

            Operation {
                a: a.to_string(),
                b: b.to_string(),
                op: op.parse().unwrap(),
                out: out.to_string(),
            }
        })
        .for_each(|op| {
            logic.insert(op.out.clone(), op);
        });

    ParseResult { init_states, logic }
}

fn calculate_state(input: &ParseResult, states: &mut HashMap<String, bool>, op: &Operation) -> bool {
    let a = match states.get(&op.a) {
        Some(value) => *value,
        None => calculate_state(input, states, &input.logic[&op.a]),
    };

    let b = match states.get(&op.b) {
        Some(value) => *value,
        None => calculate_state(input, states, &input.logic[&op.b]),
    };

    let new_state = op.apply(a, b);
    states.insert(op.out.clone(), new_state);

    new_state
}

#[aoc(day24, part1)]
pub fn part1(input: &ParseResult) -> u64 {
    let gates = input
        .logic
        .keys()
        .filter(|key| key.starts_with('z'))
        .sorted()
        .collect::<Vec<_>>();

    let mut states = input.init_states.clone();
    let mut result = 0_u64;

    for (pos, gate) in gates.iter().enumerate() {
        let op = &input.logic[*gate];
        calculate_state(input, &mut states, op);

        if states[*gate] {
            result |= 1 << (pos);
        }
    }

    result
}

#[aoc(day24, part2)]
pub fn part2(input: &ParseResult) -> isize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/2024/day24_example.txt");
    const INPUT: &str = include_str!("../input/2024/day24.txt");

    #[test]
    fn example_part1() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part1(&input), 2024);
    }

    #[test]
    fn input_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), 51107420031718);
    }

    //#[test]
    fn example_part2() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part2(&input), todo!());
    }

    //#[test]
    fn input_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), todo!());
    }
}
