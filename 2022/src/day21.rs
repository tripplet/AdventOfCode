use std::collections::{HashMap, VecDeque};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, one_of},
    combinator::map,
    multi::separated_list1,
    sequence::{separated_pair, tuple},
    IResult,
};

use crate::utils::ws;

type Number = u64;
type ParseResult<'a> = HashMap<&'a str, Monkey<'a>>;

#[derive(Debug, Clone)]
pub enum Op {
    Add,
    Sub,
    Mult,
    Div,
}

#[derive(Debug, Clone)]
pub enum Monkey<'a> {
    Number(Number),
    Operation { op: Op, left: &'a str, right: &'a str },
}

impl<'a> Monkey<'a> {
    fn parse(input: &'a str) -> IResult<&str, Self> {
        alt((
            map(nom::character::complete::u64, |nb| Monkey::Number(nb)),
            map(
                tuple((
                    alpha1,
                    map(ws(one_of("+-*/")), |symbol| match symbol {
                        '+' => Op::Add,
                        '-' => Op::Sub,
                        '*' => Op::Mult,
                        '/' => Op::Div,
                        _ => unreachable!(),
                    }),
                    alpha1,
                )),
                |res| Monkey::Operation { left: res.0, op: res.1, right: res.2 },
            ),
        ))(input)
    }
}

pub fn parse_input(input: &str) -> ParseResult {
    separated_list1(line_ending, separated_pair(alpha1, ws(tag(":")), Monkey::parse))(input)
        .unwrap().1
        .into_iter()
        .collect()
}

pub fn part1(input: &ParseResult) -> Number {
    let mut equations = input.clone();

    let mut work_queue: VecDeque<&str> = VecDeque::new();
    work_queue.push_back("root");

    while let Some(monkey_to_solve) = work_queue.pop_back() {
        let Some((op, &nb1, &nb2)) = (match &equations[monkey_to_solve] {
            Monkey::Number(_) => { None },
            Monkey::Operation{op, left,right } => {
                let Monkey::Number(nb1) = &equations[left] else {
                    work_queue.push_back(monkey_to_solve);
                    work_queue.push_back(left);
                    continue;
                };

                let Monkey::Number(nb2) = &equations[right] else {
                    work_queue.push_back(monkey_to_solve);
                    work_queue.push_back(right);
                    continue;
                };

                Some((op, nb1, nb2))
            },
        })
        else {
            continue;
        };

        equations.insert(
            monkey_to_solve,
            Monkey::Number(match op {
                Op::Add => nb1 + nb2,
                Op::Sub => nb1 - nb2,
                Op::Mult => nb1 * nb2,
                Op::Div => nb1 / nb2,
            }),
        );
    }

    let Monkey::Number(result) = equations["root"] else {
        panic!("Algorithm not working");
    };
    result
}

pub fn part2(input: &ParseResult) -> isize {
    42
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/2022/day21_example.txt");
    const INPUT: &str = include_str!("../input/2022/day21.txt");

    #[test]
    fn example_part1() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part1(&input), 152);
    }

    #[test]
    fn example_part2() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part2(&input), 301);
    }

    #[test]
    fn input_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), 155708040358220);
    }

    #[test]
    fn input_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), 42);
    }
}
