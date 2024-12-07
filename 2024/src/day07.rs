use aoc_runner_derive::{aoc, aoc_generator};
type Number = u64;
type ParseResult = Vec<Operation>;

pub struct Operation {
    result: Number,
    values: Vec<Number>,
}

#[aoc_generator(day7)]
pub fn parse_input(input: &str) -> ParseResult {
    input
        .trim()
        .lines()
        .map(|line| {
            let parts = line.split_once(": ").unwrap();
            Operation {
                result: parts.0.parse::<Number>().unwrap(),
                values: parts.1.split(' ').map(|nb| nb.parse::<Number>().unwrap()).collect(),
            }
        })
        .collect()
}

#[aoc(day7, part1)]
pub fn part1(input: &ParseResult) -> Number {
    input
        .iter()
        .filter(|operation| try_check(operation, 0, 0, false))
        .map(|operation| operation.result)
        .sum()
}

#[aoc(day7, part2)]
pub fn part2(input: &ParseResult) -> Number {
    input
        .iter()
        .filter(|operation| try_check(operation, 0, 0, true))
        .map(|operation| operation.result)
        .sum()
}

fn try_check(operation: &Operation, total: Number, pos: usize, use_concat: bool) -> bool {
    if pos == 0 {
        return try_check(operation, operation.values[0], 1, use_concat);
    }

    if pos == operation.values.len() {
        return total == operation.result;
    }

    if total > operation.result {
        return false;
    }

    if try_check(operation, total + operation.values[pos], pos + 1, use_concat)
        || try_check(operation, total * operation.values[pos], pos + 1, use_concat)
    {
        return true;
    } else if use_concat {
        let log = (operation.values[pos] as f64).log10().floor() as u32 + 1;
        let concated = 10u64.pow(log) * total + operation.values[pos];

        return try_check(operation, concated, pos + 1, true);
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/2024/day7_example.txt");
    const INPUT: &str = include_str!("../input/2024/day7.txt");

    #[test]
    fn example_part1() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part1(&input), 3749);
    }

    #[test]
    fn input_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), 66343330034722);
    }

    #[test]
    fn example_part2() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part2(&input), 11387);
    }

    #[test]
    fn input_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), 637696070419031);
    }
}
