use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use glam::IVec2;

use crate::utils::regex;

type ParseResult = Vec<Robot>;

#[derive(Debug)]
pub struct Robot {
    pos: IVec2,
    vel: IVec2,
}

#[aoc_generator(day14)]
pub fn parse_input(input: &str) -> ParseResult {
    let mults = regex!(r"p=(?<x>-?\d+),(?<y>-?\d+)\s+v=(?<vx>-?\d+),(?<vy>-?\d+)");

    input
        .trim()
        .lines()
        .map(|line| {
            let cap = mults.captures(line).unwrap();
            Robot {
                pos: IVec2::new(
                    cap.name("x").unwrap().as_str().parse().unwrap(),
                    cap.name("y").unwrap().as_str().parse().unwrap(),
                ),
                vel: IVec2::new(
                    cap.name("vx").unwrap().as_str().parse().unwrap(),
                    cap.name("vy").unwrap().as_str().parse().unwrap(),
                ),
            }
        })
        .collect()
}

impl Robot {
    fn after_seconds(&self, time: i32, room_size: IVec2) -> IVec2 {
        let mut pos = (self.pos + self.vel * time).rem_euclid(room_size);
        if pos.x < 0 {
            pos.x += room_size.x;
        }
        if pos.y < 0 {
            pos.y += room_size.y;
        }
        pos
    }
}

fn solve(input: &ParseResult, time: i32, room_size: IVec2) -> HashMap<IVec2, i32> {
    let mut positions = HashMap::new();

    for pos in input.iter().map(|robot| robot.after_seconds(time, room_size)) {
        positions.entry(pos).and_modify(|count| *count += 1).or_insert(1);
    }

    positions
}

fn count_in_quadrants(positions: &HashMap<IVec2, i32>, room_size: IVec2) -> i32 {
    let mut quadrants = [0; 4];

    for pos in positions.keys() {
        let quadrant = match (pos.x < room_size.x / 2, pos.y < room_size.y / 2) {
            (true, true) => 0,
            (true, false) => 1,
            (false, true) => 2,
            (false, false) => 3,
        };

        // Skip elements which are on the border
        if pos.x == room_size.x / 2 || pos.y == room_size.y / 2 {
            continue;
        }

        quadrants[quadrant as usize] += positions[pos];
    }

    quadrants.iter().product()
}

#[allow(unused)]
fn print_positions(positions: &HashMap<IVec2, i32>, room_size: IVec2) {
    for y in 0..room_size.y {
        for x in 0..room_size.x {
            if positions.get(&IVec2::new(x, y)).is_some() {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

#[aoc(day14, part1)]
pub fn part1(input: &ParseResult) -> i32 {
    let room_size = IVec2::new(101, 103);
    let positions = solve(input, 100, room_size);
    count_in_quadrants(&positions, room_size)
}

#[aoc(day14, part2)]
pub fn part2(input: &ParseResult) -> i32 {
    let room_size = IVec2::new(101, 103);

    for time in 0.. {
        let positions = solve(input, time, room_size);

        let mut lines = 0;
        for y in 0..room_size.y {
            let mut cnt = 0;
            for x in 0..room_size.x {
                if positions.contains_key(&IVec2::new(x, y)) {
                    cnt += 1;
                }
            }

            if cnt > 30 {
                lines += 1;
            }
        }

        // Found the image of the christmas tree by the bounding box with two continuous lines
        if lines == 2 {
            // println!("time: {}", time);
            // print_positions(&positions, room_size);
            return time;
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/2024/day14_example.txt");
    const INPUT: &str = include_str!("../input/2024/day14.txt");

    #[test]
    fn test_wrap_around() {
        let room_size = IVec2::new(11, 7);

        let robot = Robot {
            pos: IVec2::new(2, 4),
            vel: IVec2::new(2, -3),
        };

        assert_eq!(robot.after_seconds(1, room_size), IVec2::new(4, 1));
        assert_eq!(robot.after_seconds(2, room_size), IVec2::new(6, 5));
    }

    #[test]
    fn example_part1() {
        let input = parse_input(EXAMPLE);
        let room_size = IVec2::new(11, 7);

        let positions = solve(&input, 100, room_size);
        assert_eq!(count_in_quadrants(&positions, room_size), 12);
    }

    #[test]
    fn input_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), 229069152);
    }

    #[test]
    fn input_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), 7383);
    }
}
