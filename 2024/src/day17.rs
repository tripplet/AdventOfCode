use aoc_runner_derive::{aoc, aoc_generator};

type Number = i64;
type ParseResult = BitComputer;

const REG_A: usize = 0;
const REG_B: usize = 1;
const REG_C: usize = 2;

#[derive(Debug, Clone)]
pub struct BitComputer {
    program: Vec<Number>,
    reg: [Number; 3],
    pc: usize,
}

#[aoc_generator(day17)]
pub fn parse_input(input: &str) -> ParseResult {
    let mut lines = input.trim().lines();

    let reg_a = lines
        .next()
        .unwrap()
        .strip_prefix("Register A: ")
        .unwrap()
        .parse()
        .unwrap();
    let reg_b = lines
        .next()
        .unwrap()
        .strip_prefix("Register B: ")
        .unwrap()
        .parse()
        .unwrap();
    let reg_c = lines
        .next()
        .unwrap()
        .strip_prefix("Register C: ")
        .unwrap()
        .parse()
        .unwrap();

    // Consume empty line
    _ = lines.next();

    let program = lines
        .next()
        .unwrap()
        .strip_prefix("Program: ")
        .unwrap()
        .split(',')
        .map(|s| s.trim().parse().unwrap())
        .collect();

    ParseResult {
        program,
        reg: [reg_a, reg_b, reg_c],
        pc: 0,
    }
}

#[derive(Debug, Clone)]
enum OpcodeResult {
    Output(Number),
    NoOutput,
    Halt,
    Error(String),
}

impl BitComputer {
    fn run_opcode(&mut self) -> OpcodeResult {
        if self.pc >= self.program.len() {
            return OpcodeResult::Halt;
        }

        let opcode = self.program[self.pc];
        let operand = self.get_operand(self.pc + 1);

        match opcode {
            0 => {
                // ADV instruction#
                self.reg[REG_A] /= 2i64.pow(operand as u32);
                self.pc += 2;
            }
            1 => {
                // BXL instruction
                self.reg[REG_B] ^= self.program[self.pc + 1];
                self.pc += 2;
            }
            2 => {
                // BST instruction
                self.reg[REG_B] = operand % 8;
                self.pc += 2;
            }
            3 => {
                // JNZ instruction
                if self.reg[REG_A] != 0 {
                    self.pc = operand as usize;
                } else {
                    self.pc += 2;
                }
            }
            4 => {
                // BXC instruction
                self.reg[REG_B] ^= self.reg[REG_C];
                self.pc += 2;
            }
            5 => {
                // OUT instruction
                self.pc += 2;
                return OpcodeResult::Output(operand % 8);
            }
            6 => {
                // BDV instruction
                self.reg[REG_B] = self.reg[REG_A] / 2i64.pow(operand as u32);
                self.pc += 2;
            }
            7 => {
                // CDV instruction
                self.reg[REG_C] = self.reg[REG_A] / 2i64.pow(operand as u32);
                self.pc += 2;
            }

            _ => return OpcodeResult::Error(format!("Invalid opcode: {opcode}")),
        }

        OpcodeResult::NoOutput
    }

    fn get_operand(&self, offset: usize) -> Number {
        let raw_operand = self.program[offset];

        match raw_operand {
            0..=3 => raw_operand,
            4 => self.reg[REG_A],
            5 => self.reg[REG_B],
            6 => self.reg[REG_C],
            _ => panic!("Invalid operand: {raw_operand}"),
        }
    }
}

#[aoc(day17, part1)]
pub fn part1(input: &ParseResult) -> String {
    let mut computer = input.clone();
    let mut result = vec![];

    loop {
        match computer.run_opcode() {
            OpcodeResult::Output(output) => result.push(output),
            OpcodeResult::Halt => break,
            OpcodeResult::Error(error) => panic!("Error: {error}"),
            OpcodeResult::NoOutput => {}
        }
    }

    result.iter().map(std::string::ToString::to_string).collect::<Vec<String>>().join(",")
}

#[aoc(day17, part2)]
pub fn part2(input: &ParseResult) -> usize {
    let expeted_output = input.program.clone();

    for idx in 0.. {
        //println!("idx: {}", idx);
        let mut computer = input.clone();
        computer.reg[REG_A] = idx as i64;

        let mut result = vec![];

        loop {
            match computer.run_opcode() {
                OpcodeResult::Output(output) => result.push(output),
                OpcodeResult::Halt => break,
                OpcodeResult::Error(error) => panic!("Error: {error}"),
                OpcodeResult::NoOutput => {}
            }

            if result == expeted_output {
                return idx;
            }
            else if expeted_output.starts_with(&result) {
                continue
            }

            break;
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = include_str!("../input/2024/day17_example1.txt");
    const EXAMPLE_2: &str = include_str!("../input/2024/day17_example2.txt");
    const INPUT: &str = include_str!("../input/2024/day17.txt");

    #[test]
    fn example_part1() {
        let input = parse_input(EXAMPLE_1);
        assert_eq!(part1(&input), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn input_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), "3,5,0,1,5,1,5,1,0");
    }

    #[test]
    fn example_part2() {
        let input = parse_input(EXAMPLE_2);
        assert_eq!(part2(&input), 117440);
    }

    #[test]
    fn input_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), 10);
    }
}
