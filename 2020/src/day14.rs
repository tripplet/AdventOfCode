use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Instruction {
    Mask(Mask),
    Memory(Memory),
}

#[derive(Debug)]
pub struct Memory {
    dest: u64,
    value: usize,
}

pub type Mask = Vec<(usize, char)>;

fn main() {
    let data = parse(include_str!("../input/2020/day14.txt"));

    let now = std::time::Instant::now();
    let part1 = part1(&data);
    println!("Part1: {}  [{}]", part1, humantime::format_duration(now.elapsed()));
    assert_eq!(part1, 16003257187056);

    let now = std::time::Instant::now();
    let part2 = part2(&data);
    println!("Part1: {}  [{}]", part2, humantime::format_duration(now.elapsed()));
    assert_eq!(part2, 3219837697833);
}

pub fn parse(input: &str) -> Vec<Instruction> {
    let re_mask = Regex::new(r"(?m)mask = ([01X]+)").unwrap();
    let re_mem = Regex::new(r"(?m)mem\[(?P<dest>\d+)\] = (?P<value>\d+)").unwrap();

    let mut instr = Vec::new();

    for line in input.trim().lines() {
        let mask = re_mask.captures(line.trim());
        let mem = re_mem.captures(line.trim());

        if let Some(mem) = mem {
            instr.push(Instruction::Memory(Memory {
                dest: mem.name("dest").unwrap().as_str().parse().unwrap(),
                value: mem.name("value").unwrap().as_str().parse().unwrap(),
            }));
        } else if let Some(mask) = mask {
            instr.push(Instruction::Mask(
                mask.get(1)
                    .unwrap()
                    .as_str()
                    .chars()
                    .rev()
                    .enumerate()
                    .map(|c| (1 << c.0, c.1))
                    .collect(),
            ));
        }
    }
    instr
}

fn part1(code: &Vec<Instruction>) -> usize {
    let mut mem = HashMap::new();
    let mut current_mask = None;

    for instr in code {
        match instr {
            Instruction::Mask(m) => current_mask = Some(m),
            Instruction::Memory(m) => {
                mem.insert(m.dest, apply_mask(current_mask.unwrap(), m.value));
            }
        }
    }
    mem.values().sum()
}

fn apply_mask(mask: &Mask, value: usize) -> usize {
    let mut new_value = value;

    for bit in mask {
        match bit.1 {
            '1' => new_value |= bit.0,
            '0' => new_value &= !bit.0,
            'X' => (),
            _ => panic!(),
        }
    }
    new_value
}

fn part2(code: &Vec<Instruction>) -> usize {
    let mut mem = HashMap::new();
    let mut current_mask = None;

    for instr in code {
        match instr {
            Instruction::Mask(m) => current_mask = Some(m),
            Instruction::Memory(m) => {
                set_memory(&mut mem, m, current_mask.unwrap(), 0);
            }
        }
    }
    mem.values().sum()
}

fn set_memory(mem: &mut HashMap<u64, usize>, instr: &Memory, mask: &Mask, pos: usize) {
    let floating_pos = mask.iter().position(|p| p.0 >= pos && p.1 == 'X');

    if let Some(floating_pos) = floating_pos {
        let mut mask_0 = mask.clone();
        mask_0[floating_pos] = (mask_0[floating_pos].0, 'z'); // forced zero bit

        let mut mask_1 = mask.clone();
        mask_1[floating_pos] = (mask_1[floating_pos].0, '1');

        set_memory(mem, instr, &mask_0, floating_pos + 1);
        set_memory(mem, instr, &mask_1, floating_pos + 1);
    } else {
        mem.insert(apply_mask2(mask, instr.dest as usize) as u64, instr.value);
    }
}

fn apply_mask2(mask: &Mask, value: usize) -> usize {
    let mut new_value = value;

    for bit in mask {
        match bit.1 {
            '0' => (),                  // ignore
            '1' => new_value |= bit.0,  // set to 1
            'z' => new_value &= !bit.0, // forced 0 bit
            'X' => panic!(),            // should no longer be part of the mask
            _ => panic!(),
        }
    }
    new_value
}
