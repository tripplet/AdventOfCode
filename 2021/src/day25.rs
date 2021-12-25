use ndarray::Array2;
use std::{error::Error, fmt, str::FromStr, string::ToString};
use strum_macros::{Display, EnumString};

const INPUT: &str = include_str!("../input/2021/day25.txt");

#[derive(Debug, Display, PartialEq, EnumString, Clone, Copy)]
enum SeaCucumber {
    #[strum(serialize = ">")]
    East,

    #[strum(serialize = "v")]
    South,

    #[strum(serialize = ".")]
    Empty,

    #[strum(serialize = "X")]
    DBG,
}

#[derive(Debug, Clone)]
struct SeaFloor {
    floor: Array2<SeaCucumber>,
}

impl fmt::Display for SeaFloor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.floor.shape()[0] {
            for x in 0..self.floor.shape()[1] {
                write!(f, "{}", self.floor[[y, x]])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl FromStr for SeaFloor {
    type Err = Box<dyn Error>;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let data = input
            .trim()
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_string().parse::<SeaCucumber>())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
            .into_iter()
            .map(|row| row.into_iter().collect::<Result<Vec<_>, _>>())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(SeaFloor {
            floor: Array2::from_shape_vec((data.len(), data[0].len()), data.iter().flatten().cloned().collect())?,
        })
    }
}

impl SeaFloor {
    fn step(&mut self) -> usize {
        let mut move_count = 0;
        let mut move_instr = vec![];

        for (idx, &value) in self.floor.indexed_iter() {
            if value == SeaCucumber::East && self.floor[self.wraped_idx((idx.0, idx.1 + 1))] == SeaCucumber::Empty {
                move_instr.push(idx);
            }
        }

        move_count += move_instr.len();

        for idx in &move_instr {
            self.floor[*idx] = SeaCucumber::Empty;
            let new_pos = self.wraped_idx((idx.0, idx.1 + 1));
            self.floor[new_pos] = SeaCucumber::East;
        }

        move_instr.clear();

        for (idx, &value) in self.floor.indexed_iter() {
            if value == SeaCucumber::South && self.floor[self.wraped_idx((idx.0 + 1, idx.1))] == SeaCucumber::Empty {
                move_instr.push(idx);
            }
        }

        move_count += move_instr.len();

        for idx in &move_instr {
            self.floor[*idx] = SeaCucumber::Empty;
            let new_pos = self.wraped_idx((idx.0 + 1, idx.1));
            self.floor[new_pos] = SeaCucumber::South;
        }

        move_count
    }

    fn wraped_idx(&self, idx: (usize, usize)) -> (usize, usize) {
        let shape = self.floor.shape();
        (idx.0 % shape[0], idx.1 % shape[1])
    }
}

pub fn main() {
    let mut now = std::time::Instant::now();
    let floor = INPUT.parse::<SeaFloor>().unwrap();
    println!("Parsing [{}]\n", humantime::format_duration(now.elapsed()));

    now = std::time::Instant::now();
    println!(
        "Part1: {} [{}]",
        part1(&floor),
        humantime::format_duration(now.elapsed())
    );
}

fn part1(input: &SeaFloor) -> usize {
    let mut floor = input.clone();

    for idx in 1.. {
        if floor.step() == 0 {
            return idx;
        }
    }
    unreachable!();
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/2021/day25_example.txt");

    #[test]
    fn example() {
        assert_eq!(58, part1(&EXAMPLE.parse::<SeaFloor>().unwrap()))
    }

    #[test]
    fn input() {
        assert_eq!(380, part1(&INPUT.parse::<SeaFloor>().unwrap()))
    }
}