use aoc_runner_derive::{aoc, aoc_generator};

use crate::utils::regex;

type Number = i32;
type ParseResult = String;

#[aoc_generator(day3)]
pub fn parse_input(input: &str) -> ParseResult {
    input.to_string()
}

#[aoc(day3, part1)]
pub fn part1(input: &ParseResult) -> isize {
    let mults = regex!(r"mul\((?<nb1>\d{1,3}),(?<nb2>\d{1,3})\)");
    mults.captures_iter(input).map(|cap| eval_mult(&cap)).sum::<Number>() as isize
}

fn eval_mult(cap: &regex::Captures<'_>) -> Number {
    cap.name("nb1").unwrap().as_str().parse::<Number>().unwrap()
        * cap.name("nb2").unwrap().as_str().parse::<Number>().unwrap()
}

#[aoc(day3, part2)]
pub fn part2(input: &ParseResult) -> isize {
    let mults = regex!(r"((?<do>do\(\))|(?<dont>don't\(\))|mul\((?<nb1>\d{1,3}),(?<nb2>\d{1,3})\))");

    let mut active = true;
    let mut pos = 0;
    let mut result = 0;

    loop {
        let Some(next) = mults.captures_at(input, pos) else {
            break;
        };

        if next.name("do").is_some() {
            active = true;
        } else if next.name("dont").is_some() {
            active = false;
        } else if active {
            result += eval_mult(&next);
        }

        pos = next.get(0).unwrap().end();
    }

    result as isize
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/2024/day3.txt");

    #[test]
    fn input_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), 164730528);
    }

    #[test]
    fn input_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), 70478672);
    }
}
