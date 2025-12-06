use aoc_runner_derive::{aoc, aoc_generator};

type Number = usize;

#[derive(Debug, Clone, Copy)]
pub enum Op {
    Add,
    Mult,
}

#[derive(Debug)]
pub struct ParseResultPart1 {
    numbers: Vec<Vec<Number>>,
    operations: Vec<Op>,
}

#[derive(Debug)]
pub struct Equation {
    numbers: Vec<Number>,
    op: Op,
}

#[derive(Debug)]
pub struct ParseResultPart2 {
    equations: Vec<Equation>,
}

impl TryFrom<char> for Op {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '+' => Ok(Op::Add),
            '*' => Ok(Op::Mult),
            _ => Err("invalid char"),
        }
    }
}

#[aoc_generator(day06, part1)]
pub fn parse_input_part1(input: &str) -> ParseResultPart1 {
    let lines = input.trim().lines().collect::<Vec<_>>();
    let mut numbers = vec![];

    for idx in 0..lines.len() - 1 {
        numbers.push(
            lines[idx]
                .split_ascii_whitespace()
                .map(|nb| nb.parse::<Number>().unwrap())
                .collect::<Vec<_>>(),
        );
    }

    let operations = lines[lines.len() - 1]
        .split_ascii_whitespace()
        .map(|op| op.chars().next().unwrap().try_into().unwrap())
        .collect::<Vec<_>>();

    ParseResultPart1 {
        numbers,
        operations,
    }
}

#[aoc(day06, part1)]
pub fn part1(input: &ParseResultPart1) -> usize {
    let mut total = 0;

    for x_idx in 0..input.numbers[0].len() {
        let mut sub_result = input.numbers[0][x_idx];
        for y_idx in 1..input.numbers.len() {
            match input.operations[x_idx] {
                Op::Add => sub_result += input.numbers[y_idx][x_idx],
                Op::Mult => sub_result *= input.numbers[y_idx][x_idx],
            }
        }

        total += sub_result;
    }

    total
}

#[aoc_generator(day06, part2)]
pub fn parse_input_part2(input: &str) -> ParseResultPart2 {
    let mut lines = input
        .trim()
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let max_len = lines.iter().map(|line| line.len()).max().unwrap();

    for line in lines.iter_mut() {
        if line.len() < max_len {
            line.extend(vec![' '; max_len - line.len()]);
        }
    }

    // Find the start of each equation
    let op_positions = lines
        .last()
        .unwrap()
        .iter()
        .enumerate()
        .filter_map(|(pos, ch)| (*ch).try_into().map(|op| (pos, op)).ok())
        .collect::<Vec<_>>();

    // Extract column numbers
    let mut equations = vec![];
    for idx in 0..op_positions.len() {
        let mut equation = Equation {
            numbers: vec![],
            op: op_positions[idx].1,
        };

        let start = op_positions[idx].0;
        let end = match op_positions.get(idx + 1) {
            Some(found) => found.0 - 2,
            None => lines[0].len() - 1,
        };

        for x in start..=end {
            let mut nb = vec![];
            for y in 0..lines.len() - 1 {
                let ch = lines[y][x];
                if ch.is_digit(10) {
                    nb.push(ch);
                }
            }

            equation
                .numbers
                .push(nb.iter().collect::<String>().parse::<Number>().unwrap());
        }

        equations.push(equation);
    }

    ParseResultPart2 { equations }
}

#[aoc(day06, part2)]
pub fn part2(input: &ParseResultPart2) -> Number {
    let mut total = 0;
    for eq in &input.equations {
        total += match eq.op {
            Op::Add => eq.numbers.iter().sum::<Number>(),
            Op::Mult => eq.numbers.iter().product(),
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/2025/day6_example.txt");
    const INPUT: &str = include_str!("../input/2025/day6.txt");

    #[test]
    fn example_part1() {
        let input = parse_input_part1(EXAMPLE);
        assert_eq!(part1(&input), 4277556);
    }

    #[test]
    fn input_part1() {
        let input = parse_input_part1(INPUT);
        assert_eq!(part1(&input), 5552221122013);
    }

    #[test]
    fn example_part2() {
        let input = parse_input_part2(EXAMPLE);
        assert_eq!(part2(&input), 3263827);
    }

    #[test]
    fn input_part2() {
        let input = parse_input_part2(INPUT);
        assert_eq!(part2(&input), 11371597126232);
    }
}
