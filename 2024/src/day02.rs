use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type Number = i32;
type ParseResult = Vec<Vec<Number>>;

#[aoc_generator(day2)]
pub fn parse_input(input: &str) -> ParseResult {
    input
        .trim()
        .lines()
        .map(|line| line.split(' ').map(|nb| nb.trim().parse().unwrap()).collect())
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &ParseResult) -> isize {
    input.iter().filter(|report| check_report(report)).count() as isize
}

fn check_report(report: &[Number]) -> bool {
    fn is_safe(nb: Number) -> bool {
        nb.abs() <= 3 && nb != 0
    }

    report
        .iter()
        .tuple_windows()
        .map(|(&a, &b)| b - a)
        .tuple_windows()
        .all(|(a, b)| is_safe(a) && is_safe(b) && a.signum() == b.signum())
}

#[aoc(day2, part2)]
pub fn part2(input: &ParseResult) -> isize {
    let mut safe = 0;

    for report in input {
        if check_report(report) {
            safe += 1;
            continue;
        }

        for remove_idx in 0..report.len() {
            let mut report_1_removed = report.clone();
            report_1_removed.remove(remove_idx);

            if check_report(&report_1_removed) {
                safe += 1;
                break;
            }
        }
    }

    safe
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/2024/day2_example.txt");
    const INPUT: &str = include_str!("../input/2024/day2.txt");

    #[test]
    fn example_part1() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part1(&input), 2);
    }

    #[test]
    fn input_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), 663);
    }

    #[test]
    fn example_part2() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part2(&input), 4);
    }

    #[test]
    fn input_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), 692);
    }
}
