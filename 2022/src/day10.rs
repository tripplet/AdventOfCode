use std::{str::FromStr};

type ParseResult = Vec<Instruction>;

#[derive(Debug)]
pub enum Instruction {
    Addx(i16),
    Noop,
}

impl FromStr for Instruction {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        if s == "noop" {
            Ok(Instruction::Noop)
        } else if s.starts_with("addx") {
            Ok(Instruction::Addx(s[5..].parse::<i16>()?))
        } else {
            Err("Unknown instruction".into())
        }
    }
}

pub fn parse_input(input: &str) -> ParseResult {
    input
        .lines()
        .map(|line| line.parse::<Instruction>().unwrap())
        .collect()
}

pub fn part1(instructions: &ParseResult) -> isize {
    let mut reg_x = 1;
    let mut result: isize = 0;

    let mut instr_idx = 0;
    let mut last_instr_idx = 0;
    let mut instr_cycle_counter = 0;

    let measure_points = vec![20, 60, 100, 140, 180, 220];

    let mut instruction = instructions.get(0).unwrap();

    for cycle in 1.. {
        if instr_idx >= instructions.len() {
            break;
        }

        if last_instr_idx != instr_idx {
            last_instr_idx = instr_idx;
            instr_cycle_counter = 0;
            instruction = instructions.get(instr_idx).unwrap();
        }

        if measure_points.contains(&cycle) {
            result += cycle * reg_x;
        }

        match instruction {
            Instruction::Noop => {
                instr_idx += 1;
            },
            Instruction::Addx(x) => {
                if instr_cycle_counter == 1 {
                    reg_x += *x as isize;
                    instr_idx += 1;
                }
                else {
                    instr_cycle_counter += 1;
                }
            }
        }
    }

    result as isize
}

pub fn part2(instructions: &ParseResult) -> String {
    let mut reg_x = 1;
    let mut result: isize = 0;

    let mut instr_idx = 0;
    let mut last_instr_idx = 0;
    let mut instr_cycle_counter = 0;

    let mut instruction = instructions.get(0).unwrap();

    let mut display = vec![vec![' '; 40]; 6];

    for cycle in 1usize.. {
        if instr_idx >= instructions.len() {
            break;
        }

        if last_instr_idx != instr_idx {
            last_instr_idx = instr_idx;
            instr_cycle_counter = 0;
            instruction = instructions.get(instr_idx).unwrap();
        }

        let crt_pos = (cycle - 1) as isize % 40;

        if ((crt_pos % 40) - reg_x).abs() <= 1 {
            let y = (cycle / 40) as usize;
            let x = (cycle - y*40) as usize;

            display[y][x] = 'X';
        }

        match instruction {
            Instruction::Noop => {
                instr_idx += 1;
            },
            Instruction::Addx(x) => {
                if instr_cycle_counter == 1 {
                    reg_x += *x as isize;
                    instr_idx += 1;
                }
                else {
                    instr_cycle_counter += 1;
                }
            }
        }
    }

    let display = display.iter().map(|line| line.iter().collect::<String>()).collect::<Vec<String>>().join("\n");
    let display = String::from("\n") + display.as_str() + "\n";

    display
}