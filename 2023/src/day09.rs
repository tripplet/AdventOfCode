use aoc_runner_derive::{aoc, aoc_generator};
use nom::character::complete::{i64, line_ending, space1};
use nom::multi::separated_list1;
use nom::IResult;

type Number = i64;
type ParseResult = Vec<Vec<Number>>;

#[aoc_generator(day9)]
pub fn parse_input(input: &str) -> ParseResult {
    let parser: IResult<_, _> = separated_list1(line_ending, separated_list1(space1, i64))(input.trim());
    parser.unwrap().1
}

fn calc_part1(input_sequence: &[Number]) -> Number {
    let next_sequence = input_sequence.windows(2).map(|p| p[1] - p[0]).collect::<Vec<_>>();

    if next_sequence.iter().all(|nb| *nb == 0) {
        *input_sequence.last().unwrap()
    } else {
        *input_sequence.last().unwrap() + calc_part1(&next_sequence)
    }
}

fn calc_part2(input_sequence: &[Number]) -> Number {
    let next_sequence = input_sequence.windows(2).map(|p| p[1] - p[0]).collect::<Vec<_>>();

    if next_sequence.iter().all(|nb| *nb == 0) {
        *input_sequence.first().unwrap()
    } else {
        *input_sequence.first().unwrap() - calc_part2(&next_sequence)
    }
}

#[aoc(day9, part1)]
pub fn part1(input: &ParseResult) -> Number {
    input.iter().map(|seq| calc_part1(seq)).sum()
}

#[aoc(day9, part2)]
pub fn part2(input: &ParseResult) -> Number {
    input.iter().map(|seq| calc_part2(seq)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/2023/day9_example.txt");
    const INPUT: &str = include_str!("../input/2023/day9.txt");

    #[test]
    fn example_part1() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part1(&input), 114);
    }

    #[test]
    fn input_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), 1980437560);
    }

    #[test]
    fn example_part2() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part2(&input), 2);
    }

    #[test]
    fn input_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), 977);
    }
}
