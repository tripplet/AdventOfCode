use aoc_runner_derive::{aoc, aoc_generator};

use regex::Regex;

type Number = u64;
type ParseResult = Vec<Game>;

#[derive(Debug, Clone, Copy)]
pub struct Game {
    time: Number,
    distance: Number,
}

#[aoc_generator(day6)]
pub fn parse_input(input: &str) -> ParseResult {
    let mut lines = input.trim().lines();
    let re = Regex::new(r"\s+(?P<value>[[:digit:]]+)").unwrap();

    let number = |s: &str| -> Vec<Number> {
        re.captures_iter(s)
            .map(|cap| cap.name("value").unwrap().as_str().parse::<Number>().unwrap())
            .collect()
    };

    let times = number(lines.next().unwrap());
    let distances = number(lines.next().unwrap());

    times
        .into_iter()
        .zip(distances)
        .map(|(time, distance)| Game { time, distance })
        .collect()
}

#[aoc(day6, part1)]
pub fn part1(input: &ParseResult) -> Number {
    input
        .iter()
        .map(|game| {
            let (min, max) = solve_equation(game.time, game.distance);
            max - min + 1
        })
        .product()
}

#[inline(always)]
fn solve_equation(time: Number, distance: Number) -> (Number, Number) {
    let (min, max) = solve_quadratic(-1, time as i64, -(distance as i64));
    (min.floor() as Number + 1, max.ceil() as Number - 1)
}

#[inline(always)]
fn solve_quadratic(a: i64, b: i64, c: i64) -> (f32, f32) {
    let inter = ((b * b - 4 * a * c) as f32).sqrt();
    (
        (-b as f32 + inter) / (2.0 * (a as f32)),
        (-b as f32 - inter) / (2.0 * (a as f32)),
    )
}

#[aoc(day6, part2)]
pub fn part2(input: &ParseResult) -> Number {
    let a = input.iter().cloned().reduce(|acc, game| Game {
        time: format!("{}{}", acc.time, game.time).parse::<Number>().unwrap(),
        distance: format!("{}{}", acc.distance, game.distance).parse::<Number>().unwrap(),
    });

    part1(&vec![a.unwrap()])
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/2023/day6_example.txt");
    const INPUT: &str = include_str!("../input/2023/day6.txt");

    #[test]
    fn equation() {
        assert_eq!(solve_equation(7, 9), (2, 5));
        assert_eq!(solve_equation(15, 40), (4, 11));
    }

    #[test]
    fn example_part1() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part1(&input), 288);
    }

    #[test]
    fn example_part2() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part2(&input), 71503);
    }

    #[test]
    fn input_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), 293046);
    }

    #[test]
    fn input_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), 35150181);
    }
}
