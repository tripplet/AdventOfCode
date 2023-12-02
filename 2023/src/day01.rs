use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

type Number = u32;
type ParseResult = Vec<String>;

#[aoc_generator(day1)]
pub fn parse_input(input: &str) -> ParseResult {
    input.trim().lines().map(|line| line.to_owned()).collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &ParseResult) -> Number {
    input
        .iter()
        .map(|line| {
            let mut digits = line
                .chars()
                .filter_map(|char| char.to_digit(10));

            let first = digits.next().unwrap();
            let last = digits.last();

            first * 10 + last.unwrap_or(first)
        })
        .sum()
}

const NUMBERS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

#[aoc(day1, part2)]
pub fn part2(input: &ParseResult) -> Number {
    input
        .iter()
        .map(|line| {
            let mut first = None;
            let mut last = None;

            for idx in 0..line.len() {
                let part_of_line = &line[idx..];
                let char = part_of_line.chars().next().unwrap().to_digit(10);

                let mut digit = None;
                if char.is_some() {
                    digit = char;
                } else {
                    for nb in NUMBERS.iter().enumerate() {
                        if part_of_line.starts_with(nb.1) {
                            digit = Some(nb.0 as u32);
                            break;
                        }
                    }
                };

                if digit.is_some() {
                    if first.is_none() {
                        first = digit;
                    }
                    else {
                        last = digit;
                    }
                }
            }

            let first = first.unwrap();
            first * 10 + last.unwrap_or(first)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/2023/day1.txt");

    #[test]
    fn input_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), 55172);
    }

    #[test]
    fn input_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), 54925);
    }
}
