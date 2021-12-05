use lazy_static::lazy_static;
use regex::Regex;
use std::cmp;
use std::error::Error;
use std::str::FromStr;

lazy_static! {
    static ref REGEX_LINES: Regex = Regex::new(r"(?P<x1>\d+),(?P<y1>\d+) -> (?P<x2>\d+),(?P<y2>\d+)").unwrap();
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Line {
    x1: u16,
    y1: u16,
    x2: u16,
    y2: u16,
}

pub fn main() {
    let lines = parse_input(include_str!("../input/2021/day5.txt")).unwrap();

    println!("Part1: {}", part1(&lines));
    println!("Part2: {}", part2(&lines));
}

fn part1(lines: &[Line]) -> usize { solve(lines, false) }
fn part2(lines: &[Line]) -> usize { solve(lines, true) }

impl FromStr for Line {
    type Err = Box<dyn Error>;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        if let Some(values) = REGEX_LINES.captures(line) {
            Ok(Line {
                x1: values.name("x1").ok_or("invalid x1")?.as_str().parse()?,
                y1: values.name("y1").ok_or("invalid y1")?.as_str().parse()?,
                x2: values.name("x2").ok_or("invalid x2")?.as_str().parse()?,
                y2: values.name("y2").ok_or("invalid y2")?.as_str().parse()?,
            })
        } else {
            Err("line does not match correct syntax".into())
        }
    }
}

impl Line {
    fn is_vertical(&self) -> bool { self.x1 == self.x2 }
    fn is_horizontal(&self) -> bool { self.y1 == self.y2 }
}

fn solve(lines: &[Line], use_diagonals: bool) -> usize {
    // get map boundaries
    let mut max_x = 0;
    let mut max_y = 0;

    for line in lines {
        max_x = cmp::max(max_x, cmp::max(line.x1, line.x2));
        max_y = cmp::max(max_y, cmp::max(line.y1, line.y2));
    }

    let mut map: Vec<Vec<u16>> =
        vec![vec![0; (max_x + 1) as usize]; (max_y + 1) as usize];

    for line in lines {
        if line.is_horizontal() {
            for x in cmp::min(line.x1, line.x2)..=cmp::max(line.x1, line.x2) {
                map[line.y1 as usize][x as usize] += 1;
            }
        } else if line.is_vertical() {
            for y in cmp::min(line.y1, line.y2)..=cmp::max(line.y1, line.y2) {
                map[y as usize][line.x1 as usize] += 1;
            }
        } else if use_diagonals {
            let dx: i16 = if line.x1 < line.x2 { 1 } else { -1 };
            let dy: i16 = if line.y1 < line.y2 { 1 } else { -1 };
            let mut x = line.x1 as i16;
            let mut y = line.y1 as i16;

            loop {
                map[y as usize][x as usize] += 1;
                x += dx;
                y += dy;
                if x as u16 == line.x2 {
                    map[y as usize][x as usize] += 1;
                    break;
                }
            }
        }
    }

    map.iter()
        .map(|row| {
            row.iter()
                .fold(0, |acc, v| if *v > 1 { acc + 1 } else { acc })
        })
        .sum()
}

fn parse_input(input: &str) -> Result<Vec<Line>, Box<dyn Error>> {
    input
        .trim()
        .lines()
        .map(|line| line.trim().parse::<Line>())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            5,
            part1(&parse_input(include_str!("../input/2021/day5_example.txt")).unwrap())
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            12,
            part2(&parse_input(include_str!("../input/2021/day5_example.txt")).unwrap())
        );
    }

    #[test]
    fn parse() {
        let lines = parse_input("1,2 -> 3,4\n5,6 -> 7,8").unwrap();
        assert_eq!(
            vec![
                Line {x1: 1, y1: 2, x2: 3, y2: 4},
                Line {x1: 5, y1: 6, x2: 7, y2: 8}
            ],
            lines
        );
    }
}
