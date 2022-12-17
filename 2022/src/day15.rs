use crate::utils::ws;

use nom::{
    bytes::complete::tag,
    character::complete::{i64, multispace0},
    sequence::delimited,
    sequence::separated_pair,
    IResult,
};

type ParseResult = Vec<Sensor>;

#[derive(Debug, PartialEq)]
struct Position {
    x: i64,
    y: i64,
}

#[derive(Debug, PartialEq)]
pub struct Sensor {
    position: Position,
    beacon: Position,
    radius: i64,
}

impl Position {
    fn parse(s: &str) -> IResult<&str, Self> {
        let (s, (x, y)) = separated_pair(
            delimited(ws(tag("x=")), i64, multispace0),
            tag(","),
            delimited(ws(tag("y=")), i64, multispace0),
        )(s)?;

        Ok((s, Position { x, y }))
    }
}

impl Sensor {
    fn parse(s: &str) -> IResult<&str, Self> {
        let (s, (position, beacon)) = separated_pair(
            delimited(ws(tag("Sensor at")), Position::parse, multispace0),
            tag(":"),
            delimited(ws(tag("closest beacon is at")), Position::parse, multispace0),
        )(s)?;

        let radius = (position.x - beacon.x).abs() + (position.y - beacon.y).abs();
        Ok((
            s,
            Sensor {
                position,
                beacon,
                radius,
            },
        ))
    }

    fn get_occupied_range_in_line(&self, line: i64) -> Option<(i64, i64)> {
        let delta = (self.position.y - line).abs();

        if delta > self.radius {
            return None;
        }

        let range_left_right = self.radius - delta;
        Some((self.position.x - range_left_right, self.position.x + range_left_right))
    }
}

pub fn parse_input(input: &str) -> ParseResult {
    input
        .trim()
        .lines()
        .map(|line| Sensor::parse(line).unwrap().1)
        .collect()
}

fn get_occupied_ranges_in_line(input: &ParseResult, line_index: i64) -> Vec<(i64, i64)> {
    let mut ranges = input
        .iter()
        .filter_map(|sensor| sensor.get_occupied_range_in_line(line_index))
        .collect::<Vec<_>>();

    ranges.sort_unstable_by_key(|&(l, _)| l);

    // combine ranges
    'outer: loop {
        for i in 0..ranges.len() {
            for j in 0..ranges.len() {
                if i == j {
                    continue;
                }

                if let Some((left, right)) = ranges.get(i) {
                    if let Some((left2, right2)) = ranges.get(j) {
                        if left2 <= left && right2 >= right {
                            ranges.remove(i);
                            continue 'outer;
                        } else if (left2 <= right && left2 >= left) || (right2 >= left && right2 <= right) {
                            ranges[i] = (*left2.min(left), *right2.max(right));
                            ranges.remove(j);
                            continue 'outer;
                        }
                    }
                }
            }
        }

        break;
    }

    ranges
}

fn get_occupied_in_line(input: &ParseResult, line_index: i64) -> u64 {
    get_occupied_ranges_in_line(input, line_index)
        .iter()
        .map(|(left, right)| (right - left).abs() as u64)
        .sum()
}

fn find_distress_beacon_parallel(input: &ParseResult, search_range_max: u64) -> Option<(i64, i64)> {
    use rayon::prelude::*;

    (0..=search_range_max).into_par_iter().find_map_any(|line_index| {
        let occupied = get_occupied_ranges_in_line(input, line_index as i64);

        let occupied_count = occupied
            .iter()
            .map(|&(left, right)| ((right.min(search_range_max as i64) - left.max(0)).abs()) as u64)
            .sum::<u64>();

        if occupied_count < search_range_max {
            if occupied.len() == 1 {
                todo!("does not occur in sample data");
            } else {
                return Some((occupied[0].0.max(occupied[1].0) - 1, line_index as i64));
            }
        }

        None
    })
}

pub fn part1(input: &ParseResult) -> u64 {
    get_occupied_in_line(input, 2_000_000)
}

pub fn part2(input: &ParseResult) -> i64 {
    let beacon = find_distress_beacon_parallel(input, 4_000_000).unwrap();
    (beacon.0) * 4_000_000 + beacon.1
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/2022/day15_example.txt");

    #[test]
    fn parse_position() {
        assert_eq!(Position::parse("x=17, y=5"), Ok(("", Position { x: 17, y: 5 })));
        assert_eq!(
            Position::parse(" x=12345 , y=6789 "),
            Ok(("", Position { x: 12345, y: 6789 }))
        );
    }

    #[test]
    fn parse_sensor() {
        assert_eq!(
            Sensor::parse("Sensor at x=17, y=5: closest beacon is at x=12345, y=6789"),
            Ok((
                "",
                Sensor {
                    position: Position { x: 17, y: 5 },
                    beacon: Position { x: 12345, y: 6789 },
                    radius: 19112
                }
            ))
        );
    }

    #[test]
    fn check_occupied_range() {
        let sensor = Sensor::parse("Sensor at x=8, y=7: closest beacon is at x=2, y=10")
            .unwrap()
            .1;

        assert_eq!(sensor.radius, 9);
        assert_eq!(sensor.get_occupied_range_in_line(0), Some((6, 10)));
        assert_eq!(sensor.get_occupied_range_in_line(-2), Some((8, 8)));
        assert_eq!(sensor.get_occupied_range_in_line(-3), None);
    }

    #[test]
    fn part1_example() {
        let sensors = parse_input(EXAMPLE);
        assert_eq!(get_occupied_in_line(&sensors, 10), 26);
    }

    #[test]
    fn part2_example() {
        let sensors = parse_input(EXAMPLE);
        assert_eq!(find_distress_beacon_parallel(&sensors, 20), Some((14, 11)));
    }
}
