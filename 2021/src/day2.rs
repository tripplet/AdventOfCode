use std::str::FromStr;

#[derive(Debug)]
enum MoveCommand {
    Forward(i32),
    Up(i32),
    Down(i32),
}

impl FromStr for MoveCommand {
    type Err = ();

    fn from_str(input: &str) -> Result<MoveCommand, Self::Err> {
        let parts = input.split(' ').collect::<Vec<_>>();
        if parts.len() != 2 {
            Err(())
        } else {
            match parts[0] {
                "forward" => Ok(MoveCommand::Forward(parts[1].parse::<i32>().unwrap())),
                "up" => Ok(MoveCommand::Up(parts[1].parse::<i32>().unwrap())),
                "down" => Ok(MoveCommand::Down(parts[1].parse::<i32>().unwrap())),
                _ => Err(()),
            }
        }
    }
}

pub fn main() {
    let input = include_str!("../input/2021/day2.txt");

    let commands: Vec<_> = input
        .trim()
        .lines()
        .map(|line| line.trim().parse::<MoveCommand>().unwrap())
        .collect();

    println!("Part1: {}", part1(&commands));
    println!("Part2: {}", part2(&commands));
}

fn part1(commands: &[MoveCommand]) -> i32 {
    let mut x: i32 = 0;
    let mut y: i32 = 0;

    for command in commands {
        match command {
            MoveCommand::Forward(delta) => x += delta,
            MoveCommand::Up(delta) => y -= delta,
            MoveCommand::Down(delta) => y += delta,
        }
    }
    x * y
}

fn part2(commands: &[MoveCommand]) -> i32 {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut aim: i32 = 0;

    for command in commands {
        match command {
            MoveCommand::Forward(delta) => {
                x += delta;
                y += aim * delta
            }
            MoveCommand::Up(delta) => aim -= delta,
            MoveCommand::Down(delta) => aim += delta,
        }
    }
    x * y
}
