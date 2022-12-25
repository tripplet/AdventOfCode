use std::ops::Add;

use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, i32};
use nom::combinator::map;
use nom::IResult;
use nom::{multi::separated_list1, sequence::separated_pair};

use crate::utils::ws;

type Number = i32;
type ParseResult = Vec<Vec<Coordinate>>;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Coordinate {
    x: Number,
    y: Number,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Content {
    Rock,
    Sand,
}

impl Add<Coordinate> for Coordinate {
    type Output = Coordinate;

    fn add(self, other: Coordinate) -> Coordinate {
        Coordinate {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

pub fn parse_input(input: &str) -> ParseResult {
    (separated_list1(
        line_ending,
        separated_list1(
            ws(tag("->")),
            map(separated_pair(i32, ws(tag(",")), i32), |(x, y)| Coordinate { x, y }),
        ),
    )(input) as IResult<_,_>).unwrap().1
}

pub fn part1(walls: &ParseResult) -> isize {
    // Build pic of walls from coordinates
    let mut pic = std::collections::HashMap::new();
    let spawn = Coordinate { x: 500, y: 0 };

    let mut max_y = std::i32::MIN;

    walls.iter().for_each(|wall| {
        wall.iter().tuple_windows().for_each(|(a, b)| {
            for x in a.x.min(b.x)..=a.x.max(b.x) {
                for y in a.y.min(b.y)..=a.y.max(b.y) {
                    max_y = max_y.max(y);
                    pic.insert(Coordinate {x, y}, Content::Rock);
                }
            }
        });
    });

    // Fill pic with sand
    'spawn_new_sand: for unit in 1.. {
        let mut sand_pos = spawn;

        loop {
            if sand_pos.y > max_y {
                return unit - 1;
            }

            if !pic.contains_key(&(sand_pos + Coordinate {x: 0, y: 1})) {
                sand_pos = sand_pos + Coordinate {x: 0, y: 1};
            }
            else if !pic.contains_key(&(sand_pos + Coordinate {x: -1, y: 1})) {
                sand_pos = sand_pos + Coordinate {x: -1, y: 1};
            }
            else if !pic.contains_key(&(sand_pos + Coordinate {x: 1, y: 1})) {
                sand_pos = sand_pos + Coordinate {x: 1, y: 1};
            }
            else {
                // Sand has come to rest
                pic.insert(sand_pos, Content::Sand);
                // println!("Unit: {}", unit);
                // print_pic(&pic);
                // println!();
                continue 'spawn_new_sand;
            }
        }
    }

    unreachable!()
}

#[allow(dead_code)]
fn print_pic(pic: &std::collections::HashMap<Coordinate, Content>) {
    let (min_x, max_x) = pic.keys().minmax_by_key(|k| k.x).into_option().unwrap();
    let (min_y, max_y) = pic.keys().minmax_by_key(|k| k.y).into_option().unwrap();

    for y in min_y.y..=max_y.y {
        for x in min_x.x..=max_x.x {
            if let Some(content) = pic.get(&Coordinate {x, y}) {
                match content {
                    Content::Rock => print!("#"),
                    Content::Sand => print!("o"),
                }
            } else {
                print!(".");
            }
        }
        println!();
    }
}

pub fn part2(walls: &ParseResult) -> isize {
    // Build pic of walls from coordinates
    let mut pic = std::collections::HashMap::new();
    let spawn = Coordinate { x: 500, y: 0 };

    let mut max_y = std::i32::MIN;

    walls.iter().for_each(|wall| {
        wall.iter().tuple_windows().for_each(|(a, b)| {
            for x in a.x.min(b.x)..=a.x.max(b.x) {
                for y in a.y.min(b.y)..=a.y.max(b.y) {
                    max_y = max_y.max(y);
                    pic.insert(Coordinate {x, y}, Content::Rock);
                }
            }
        });
    });

    // Fill pic with sand
    'spawn_new_sand: for unit in 1.. {
        let mut sand_pos = spawn;

        loop {
            if sand_pos.y == max_y + 1 {
                // Reached infinite bottom wall
                pic.insert(sand_pos, Content::Sand);
                continue 'spawn_new_sand;
            }

            if !pic.contains_key(&(sand_pos + Coordinate {x: 0, y: 1})) {
                sand_pos = sand_pos + Coordinate {x: 0, y: 1};
            }
            else if !pic.contains_key(&(sand_pos + Coordinate {x: -1, y: 1})) {
                sand_pos = sand_pos + Coordinate {x: -1, y: 1};
            }
            else if !pic.contains_key(&(sand_pos + Coordinate {x: 1, y: 1})) {
                sand_pos = sand_pos + Coordinate {x: 1, y: 1};
            }
            else {
                if sand_pos == spawn {
                    return unit;
                }

                // Sand has come to rest
                pic.insert(sand_pos, Content::Sand);
                // println!("Unit: {}", unit);
                // print_pic(&pic);
                // println!();
                continue 'spawn_new_sand;
            }
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/2022/day14_example.txt");
    const INPUT: &str = include_str!("../input/2022/day14.txt");

    #[test]
    fn example_part1() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part1(&input), 24);
    }

    #[test]
    fn example_part2() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part2(&input), 93);
    }

    #[test]
    fn input_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), 655);
    }

    #[test]
    fn input_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), 26484);
    }
}
