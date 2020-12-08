use std::error::Error;
use std::str::FromStr;

use strum_macros::EnumString;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Instruction {
    opcode: OpCode,
    value: i32,
}

#[derive(Debug, PartialEq, Copy, Clone, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum OpCode {
    Nop,
    Acc,
    Jmp,
}

impl FromStr for Instruction {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(" ").collect();

        let opcode = OpCode::from_str(parts[0])?;
        let value: i32 = parts[1].parse()?;

        Ok(Instruction {
            opcode: opcode,
            value: value,
        })
    }
}

pub struct CpuState {
    instruction_pointer: i32,
    accumulator: i32,
}

impl Default for CpuState {
    fn default() -> CpuState {
        CpuState {
            instruction_pointer: 0,
            accumulator: 0,
        }
    }
}

pub fn main() {
    let input = include_str!("../input/2020/day8.txt").trim();
    let code = parse(input).unwrap();
    drop(input);

    let mut state = CpuState { ..Default::default() };

    let mut now = std::time::Instant::now();
    println!("Part1: {}  [{}]", part1(&code, &mut state), humantime::format_duration(now.elapsed()));

    now = std::time::Instant::now();
    println!("Part2: {:?}  [{}]", part2(&code), humantime::format_duration(now.elapsed()));
}

pub fn parse(input: &str) -> Result<Vec<Instruction>, Box<dyn Error>> {
    input
        .lines()
        .map(|l| Instruction::from_str(l.trim()))
        .collect()
}

fn part1(code: &[Instruction], state: &mut CpuState) -> i32 {
    let result = execute(code, state);

    assert_eq!(result, false);
    state.accumulator
}

fn part2(code: &[Instruction]) -> Option<i32> {
    let mut state: CpuState;

    for idx in 0..code.len() {
        if code[idx].opcode == OpCode::Acc {
            continue;
        }

        let mut patched_code = code.to_vec();

        match patched_code[idx].opcode {
            OpCode::Nop => patched_code[idx].opcode = OpCode::Jmp,
            OpCode::Jmp => patched_code[idx].opcode = OpCode::Nop,
            _ => (),
        }

        state = CpuState { ..Default::default() };
        if execute(&patched_code, &mut state) {
            return Some(state.accumulator);
        }
    }

    None
}

fn execute(code: &[Instruction], state: &mut CpuState) -> bool {
    let mut loop_check = vec![false; code.len()];

    while (state.instruction_pointer as usize) < code.len()
        && loop_check[state.instruction_pointer as usize] == false
    {
        let instr = &code[state.instruction_pointer as usize];
        loop_check[state.instruction_pointer as usize] = true;

        match &instr.opcode {
            OpCode::Nop => state.instruction_pointer += 1,
            OpCode::Acc => {
                state.accumulator += &instr.value;
                state.instruction_pointer += 1
            }
            OpCode::Jmp => state.instruction_pointer += &instr.value,
        }
    }

    (state.instruction_pointer as usize) == code.len()
}
