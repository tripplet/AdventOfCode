use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;

use aoc_runner_derive::{aoc, aoc_generator};
use dyn_clone::DynClone;
use itertools::Itertools;
use num::Integer;

type Graph = HashMap<String, Node>;
type ParseResult = Graph;

#[derive(Debug, Clone)]
pub struct Node {
    module: Box<dyn Module>,
    outputs: Vec<String>,
}

dyn_clone::clone_trait_object!(Module);

#[derive(Debug, Default, Clone)]
pub struct FlipFlo {
    pub state: Pulse,
}

#[derive(Debug, Default, Clone)]
pub struct Broadcast {}

#[derive(Debug, Default, Clone)]
pub struct Conjunction {
    pub input_states: HashMap<String, Pulse>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Pulse {
    #[default]
    Low,
    High,
}

impl std::ops::Not for Pulse {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Pulse::Low => Pulse::High,
            Pulse::High => Pulse::Low,
        }
    }
}

pub trait Module: Debug + DynClone {
    fn evaluate(&mut self, pulse: Pulse, from: &str) -> Option<Pulse>;
    fn update_inputs(&mut self, _inputs: HashMap<String, Pulse>) {}
}

impl Module for FlipFlo {
    fn evaluate(&mut self, pulse: Pulse, _from: &str) -> Option<Pulse> {
        match (pulse, self.state) {
            (Pulse::Low, _) => {
                self.state = !self.state;
                Some(self.state)
            }
            (Pulse::High, _) => None,
        }
    }
}

impl Module for Broadcast {
    fn evaluate(&mut self, pulse: Pulse, _from: &str) -> Option<Pulse> {
        Some(pulse)
    }
}

impl Module for Conjunction {
    fn evaluate(&mut self, pulse: Pulse, from: &str) -> Option<Pulse> {
        if let Some(state) = self.input_states.get_mut(from) {
            *state = pulse;
        } else {
            panic!("Unknown input {}", from);
        };

        if self.input_states.values().all(|&state| state == Pulse::High) {
            Some(Pulse::Low)
        } else {
            Some(Pulse::High)
        }
    }

    fn update_inputs(&mut self, inputs: HashMap<String, Pulse>) {
        self.input_states = inputs;
    }
}

#[aoc_generator(day20)]
pub fn parse_input(input: &str) -> ParseResult {
    let mut graph: Graph = HashMap::new();

    let mut conjunctions = vec![];

    input.trim().lines().for_each(|line| {
        let line = line.trim();
        match line.split_once("->") {
            Some((input, outputs)) => {
                let input = input.trim();
                let outputs = outputs
                    .trim()
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect::<Vec<_>>();

                let module: (&str, Box<dyn Module>) = if input == "broadcaster" {
                    (input, Box::<Broadcast>::default())
                } else if let Some(node) = input.strip_prefix('%') {
                    (node, Box::<FlipFlo>::default())
                } else if let Some(node) = input.strip_prefix('&') {
                    conjunctions.push(node);
                    (node, Box::<Conjunction>::default())
                } else {
                    panic!("Unknown input {}", input);
                };

                graph.insert(
                    module.0.to_string(),
                    Node {
                        module: module.1,
                        outputs,
                    },
                );
            }
            _ => panic!("Invalid line {}", line),
        }
    });

    for conjunction in conjunctions {
        let incoming = graph
            .iter()
            .filter(|(_, node)| node.outputs.iter().contains(&String::from(conjunction)))
            .map(|(module, _)| module)
            .cloned()
            .collect::<Vec<_>>();

        let module = &mut graph.get_mut(conjunction).unwrap().module;

        module.update_inputs(
            incoming
                .iter()
                .map(|node| (node.to_string(), Pulse::Low))
                .collect::<HashMap<_, _>>(),
        );
    }

    graph
}

fn run_to_end(
    from: &str,
    start: &str,
    pulse: Pulse,
    graph: &mut Graph,
    mut callback: impl FnMut(&str, &str, Pulse),
) -> (usize, usize) {
    let mut queue = VecDeque::from([(String::from(from), String::from(start), pulse)]);

    let mut low_pulses = 0;
    let mut hight_pulses = 0;

    while let Some((from, to, pulse)) = queue.pop_front() {
        callback(&from, &to, pulse);

        if pulse == Pulse::Low {
            low_pulses += 1;
        } else {
            hight_pulses += 1;
        }

        if let Some(node) = graph.get_mut(&to) {
            if let Some(pulse) = node.module.evaluate(pulse, &from) {
                for output in &node.outputs {
                    queue.push_back((to.clone(), output.clone(), pulse));
                }
            }
        }
    }

    (low_pulses, hight_pulses)
}

#[aoc(day20, part1)]
pub fn part1(input: &ParseResult) -> usize {
    let mut graph = input.clone();

    let mut low_pulses = 0;
    let mut hight_pulses = 0;

    for _ in 0..1_000 {
        let (low, high) = run_to_end("button", "broadcaster", Pulse::Low, &mut graph, |_, _, _| {});
        low_pulses += low;
        hight_pulses += high;
    }

    low_pulses * hight_pulses
}

#[aoc(day20, part2)]
pub fn part2(input: &ParseResult) -> isize {
    let mut graph = input.clone();

    // General solutions is not possible with brute force
    // The graph has four areas which input to the final state
    // Determine the number of button presses required for each area

    let areas = graph.get("broadcaster").unwrap().outputs.clone();
    let src = graph
        .iter()
        .find_map(|module| {
            if module.1.outputs.contains(&String::from("rx")) {
                Some(module.0)
            } else {
                None
            }
        })
        .unwrap()
        .clone();

    areas
        .iter()
        .map(|area| {
            let mut found = false;
            let mut button_presses = 0;

            while !found {
                button_presses += 1;

                run_to_end(
                    "broadcaster",
                    area,
                    Pulse::Low,
                    &mut graph,
                    |_from: &str, to: &str, pulse: Pulse| {
                        if to == src && pulse == Pulse::High {
                            found = true;
                            //println!("{} -> {} {:?}", from, to, pulse);
                        }
                    },
                );
            }

            button_presses
        })
        .reduce(|acc, b| acc.lcm(&b))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = include_str!("../input/2023/day20_example1.txt");
    const EXAMPLE2: &str = include_str!("../input/2023/day20_example2.txt");
    const INPUT: &str = include_str!("../input/2023/day20.txt");

    #[test]
    fn example1_part1() {
        let input = parse_input(EXAMPLE1);
        assert_eq!(part1(&input), 32000000);
    }

    #[test]
    fn example2_part1() {
        let input = parse_input(EXAMPLE2);
        assert_eq!(part1(&input), 11687500);
    }

    #[test]
    fn input_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), 898557000);
    }

    #[test]
    fn input_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), 238420328103151);
    }
}
