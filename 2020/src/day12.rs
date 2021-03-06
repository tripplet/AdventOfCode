use std::error::Error;
use std::str::FromStr;

type Instructions = Vec<Instruction>;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Instruction {
    direction: Direction,
    value: i64,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Direction {
    North, South, West, East, Right, Left, Forward, Invalid,
}

impl FromStr for Instruction {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Instruction {
            direction: match s.chars().nth(0).unwrap() {
                'N' => Direction::North,
                'S' => Direction::South,
                'W' => Direction::West,
                'E' => Direction::East,
                'R' => Direction::Right,
                'L' => Direction::Left,
                'F' => Direction::Forward,
                x => panic!("Invliad direction {}", x),
            },
            value: s.chars().skip(1).collect::<String>().parse::<i64>()?,
        })
    }
}

pub fn main() {
    let instructions = parse(include_str!("../input/2020/day12.txt"));

    let mut now = std::time::Instant::now();
    println!("Part1: {}  [{}]", part1(&instructions), humantime::format_duration(now.elapsed()));

    now = std::time::Instant::now();
    println!("Part2: {}  [{}]", part2(&instructions), humantime::format_duration(now.elapsed()));
}

pub fn part1(instructions: &Instructions) -> i64 {
    let mut pos: (i64, i64) = (0, 0);
    let mut dir = Direction::East;

    let rotate = [Direction::North, Direction::East, Direction::South, Direction::West];

    for instr in instructions {
        if instr.direction == Direction::Forward {
            runstr(dir, instr.value, &mut pos)
        }
        else if instr.direction == Direction::Right ||instr.direction == Direction::Left {
            let mut cur = rotate.iter().position(|d| *d == dir).unwrap() as i64;

            if instr.direction == Direction::Right {
                cur += instr.value / 90;
            }
            else {
                cur -= instr.value / 90;
            }

            if cur < 0 {
                cur += 4;
            }

            dir = rotate[(cur % 4) as usize];
        }
        else {
            runstr(instr.direction, instr.value, &mut pos)
        }
    }
    pos.0.abs() + pos.1.abs()
}


pub fn part2(instructions: &Instructions) -> i64 {
    let mut waypos: (i64, i64) = (10, 1);
    let mut pos: (i64, i64) = (0, 0);

    for instr in instructions {
        if instr.direction == Direction::Forward {
            // Move the ship x times the waypoint values
            pos.0 += waypos.0 * instr.value;
            pos.1 += waypos.1 * instr.value;
        }
        else if instr.direction == Direction::Right ||instr.direction == Direction::Left {
            // Rotate the waypoint
            let angle = match instr.direction {
                Direction::Left => (-instr.value as f64).to_radians(),
                Direction::Right => (instr.value as f64).to_radians(),
                _ => panic!()
            };

            let x = (waypos.0 as f64 * angle.cos()) + (waypos.1 as f64 * angle.sin());
            let y = (waypos.1 as f64 * angle.cos()) - (waypos.0 as f64 * angle.sin());

            waypos = (x.round() as i64, y.round() as i64);
        }
        else {
            // Move the waypoint
            runstr(instr.direction, instr.value, &mut waypos)
        }
    }
    pos.0.abs() + pos.1.abs()
}

fn runstr(dir: Direction, value: i64, pos: &mut (i64, i64)) {
    match dir {
        Direction::North => { pos.1 += value; },
        Direction::South => { pos.1 -= value; },
        Direction::East => { pos.0 += value; },
        Direction::West => { pos.0 -= value; },
        _ => panic!()
    }
}

pub fn parse(input: &str) -> Instructions {
    input.trim().lines().map(|line| Instruction::from_str(line.trim()).unwrap()).collect()
}
