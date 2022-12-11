use std::collections::VecDeque;

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, multispace0, multispace1},
    combinator::map_res,
    error::ParseError,
    multi::separated_list0,
    sequence::{delimited, tuple},
    IResult,
};

type NUMBER = u128;
type ParseResult = Vec<Monkey>;

#[derive(Debug, Copy, Clone)]
enum MonkeyOperation {
    Add(NUMBER),
    Multiply(NUMBER),
    Quadruple,
}

#[derive(Debug, Clone)]
pub struct Monkey {
    number: usize,
    items: VecDeque<NUMBER>,
    op: MonkeyOperation,
    divisor: NUMBER,
    condition_true_monkey: usize,
    condition_false_monkey: usize,
    inspect_count: NUMBER,
}

impl Monkey {
    fn parse(s: &str) -> IResult<&str, Self> {
        let (s, _) = multispace0(s)?;
        let (s, number) = delimited(tag("Monkey "), map_res(digit1, |s: &str| s.parse()), tag(":"))(s)?;

        let (s, _) = multispace1(s)?;
        let (s, _) = tag("Starting items: ")(s)?;
        let (s, items) = map_res(separated_list0(tuple((multispace0, tag(","), multispace0)), digit1), |v: Vec<&str>| {
            v.iter().map(|s| s.parse::<NUMBER>()).collect()
        })(s)?;

        let (s, _) = tuple((multispace1, tag("Operation: new = ")))(s)?;
        let (s, op) = MonkeyOperation::parse(s)?;

        let (s, _) = tuple((multispace1, tag("Test: divisible by ")))(s)?;
        let (s, divisor) = map_res(digit1, |s: &str| s.parse())(s)?;

        let (s, _) = tuple((multispace1, tag("If true: throw to monkey ")))(s)?;
        let (s, condition_true_monkey) = map_res(digit1, |s: &str| s.parse())(s)?;

        let (s, _) = tuple((multispace1, tag("If false: throw to monkey ")))(s)?;
        let (s, condition_false_monkey) = map_res(digit1, |s: &str| s.parse())(s)?;

        // Consume any trailing whitespace
        let (s, _) = multispace0(s)?;

        Ok((
            s,
            Monkey {
                number,
                items,
                op,
                divisor,
                condition_true_monkey,
                condition_false_monkey,
                inspect_count: 0,
            },
        ))
    }
}

impl MonkeyOperation {
    fn parse(s: &str) -> IResult<&str, Self> {
        let (s, op) = alt((
            map_res(tag("old * old"), |_| -> Result<_, ()> {
                Ok(MonkeyOperation::Quadruple)
            }),
            map_res(
                tuple((tag("old + "), digit1)),
                |(_, s): (&str, &str)| -> Result<_, ()> { Ok(MonkeyOperation::Add(s.parse::<NUMBER>().unwrap())) },
            ),
            map_res(
                tuple((tag("old * "), digit1)),
                |(_, s): (&str, &str)| -> Result<_, ()> { Ok(MonkeyOperation::Multiply(s.parse::<NUMBER>().unwrap())) },
            ),
        ))(s)?;

        Ok((s, op))
    }

    fn apply(&self, old: NUMBER) -> NUMBER {
        match self {
            MonkeyOperation::Add(n) => old + n,
            MonkeyOperation::Multiply(n) => old * n,
            MonkeyOperation::Quadruple => old * old,
        }
    }
}

pub fn parse_input(input: &str) -> ParseResult {
    let mut input = input;
    let mut monkeys = vec![];

    while input.len() > 0 {
        let (rest, monkey) = Monkey::parse(input).unwrap();
        input = rest;
        monkeys.push(monkey);
    }

    monkeys
}

impl std::fmt::Display for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Monkey: {nb}, items: {items}",
            nb = self.number,
            items = self.items.iter().join(", ")
        )
    }
}

fn simulate_turn(monkeys: &mut Vec<Monkey>, worry_fn: impl Fn(NUMBER) -> NUMBER) {
    for idx in 0..monkeys.len() {
        let monkey = &mut monkeys[idx];
        let div = monkey.divisor;
        let true_monkey = monkey.condition_true_monkey;
        let false_monkey = monkey.condition_false_monkey;
        let op = monkey.op;
        let items = std::mem::replace(&mut monkey.items, VecDeque::new());

        monkey.inspect_count += items.len() as NUMBER;

        for item in items {
            let item = worry_fn(op.apply(item));
            let rx_monkey = if item % div == 0 { true_monkey } else { false_monkey };
            monkeys[rx_monkey].items.push_back(item)
        }
    }
}

fn get_monkey_business(monkeys: &Vec<Monkey>) -> usize {
    monkeys
        .iter()
        .map(|m| m.inspect_count)
        .sorted()
        .rev()
        .take(2)
        .product::<NUMBER>() as usize
}

pub fn part1(monkeys: &ParseResult) -> usize {
    let mut monkeys = monkeys.clone();

    for _ in 0..20 {
        simulate_turn(&mut monkeys, |x| x / 3);
    }

    get_monkey_business(&monkeys)
}

pub fn part2(monkeys: &ParseResult) -> usize {
    let mut monkeys = monkeys.clone();

    let lcm = monkeys
        .iter()
        .map(|m| m.divisor)
        .fold(1, |x, y| num::integer::lcm(x, y));

    for _ in 0..10_000 {
        simulate_turn(&mut monkeys, |x| x % lcm);
    }

    get_monkey_business(&monkeys)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/2022/day11.txt");
    const INPUT_EXAMPLE: &str = include_str!("../input/2022/day11_example.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse_input(INPUT_EXAMPLE)), 10605);
        assert_eq!(part1(&parse_input(INPUT)), 55930);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse_input(INPUT_EXAMPLE)), 2713310158);
        assert_eq!(part2(&parse_input(INPUT)), 14636993466);
    }
}
