use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};

type Number = u64;
type ParseResult = Vec<Number>;

#[aoc_generator(day22)]
pub fn parse_input(input: &str) -> ParseResult {
    input.trim().lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day22, part1)]
pub fn part1(input: &ParseResult) -> Number {
    input.iter().map(|&n| process(n, 2_000)).sum::<Number>()
}

#[aoc(day22, part2)]
pub fn part2(input: &ParseResult) -> Number {
    let digit_sequences = input.iter().map(|&n| ones_digits(n, 2_000)).collect::<Vec<_>>();
    let deltas = digit_sequences
        .iter()
        .map(|seq| generate_deltas(seq))
        .collect::<Vec<_>>();

    let mut bananas = HashMap::new();

    for (delta, digits) in deltas.iter().zip(digit_sequences.iter()) {
        let mut already_seen = HashSet::new();

        for (idx, buy_seq) in delta.windows(4).enumerate() {
            if already_seen.insert(buy_seq) {
                let bought_bananas = Number::from(digits[idx + 4]);
                *bananas.entry(buy_seq).or_insert(0) += bought_bananas;
            }
        }
    }

    *bananas.values().max().unwrap()
}

fn ones_digits(mut secret: Number, times: Number) -> Vec<u8> {
    let mut digits = Vec::with_capacity(times as usize);
    digits.push((secret % 10) as u8);

    for _ in 0..times {
        secret = process_step(secret);
        digits.push((secret % 10) as u8);
    }

    digits
}

fn generate_deltas(seq: &[u8]) -> Vec<i8> {
    seq.windows(2).map(|pair| pair[0] as i8 - pair[1] as i8).collect()
}

fn process(mut secret: Number, times: usize) -> Number {
    for _ in 0..times {
        secret = process_step(secret);
    }

    secret
}

#[inline]
fn process_step(mut secret: Number) -> Number {
    secret = ((secret * 64) ^ secret) % 16_777_216;
    secret = ((secret / 32) ^ secret) % 16_777_216;
    secret = ((secret * 2048) ^ secret) % 16_777_216;
    secret
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = include_str!("../input/2024/day22_example1.txt");
    const EXAMPLE2: &str = include_str!("../input/2024/day22_example2.txt");
    const INPUT: &str = include_str!("../input/2024/day22.txt");

    #[test]
    fn example_part1() {
        let input = parse_input(EXAMPLE1);
        assert_eq!(part1(&input), 37327623);
    }

    #[test]
    fn input_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), 15006633487);
    }

    #[test]
    fn example_part2() {
        let input = parse_input(EXAMPLE2);
        assert_eq!(part2(&input), 23);
    }

    #[test]
    fn input_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), 1710);
    }
}
