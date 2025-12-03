use std::{ops::RangeInclusive, usize};

use aoc_runner_derive::{aoc, aoc_generator};

type Number = usize;
type ParseResult = Vec<RangeInclusive<usize>>;

#[aoc_generator(day2)]
pub fn parse_input(input: &str) -> ParseResult {
    input
        .split(',')
        .map(|range| unsafe { range.rsplit_once('-').unwrap_unchecked() })
        .map(|(left, right)| unsafe {
            left.trim().parse::<usize>().unwrap_unchecked()
                ..=right.trim().parse::<usize>().unwrap_unchecked()
        })
        .collect::<Vec<_>>()
}

#[aoc(day2, part1)]
pub fn part1(ranges: &ParseResult) -> usize {
    let mut total_count_part1 = 0;

    for range_pair in ranges {
        total_count_part1 += range_pair
            .clone()
            .filter(|&value| is_invalid_id_part1(value))
            .sum::<usize>();
    }
    total_count_part1
}

#[aoc(day2, part2)]
pub fn part2(ranges: &ParseResult) -> usize {
    let mut total_count_part2 = 0;

    for range_pair in ranges {
        total_count_part2 += range_pair
            .clone()
            .filter(|&value| is_invalid_id_part2(value))
            .sum::<usize>();
    }
    total_count_part2
}

fn is_invalid_id_part1(number: Number) -> bool {
    let string_rep = format!("{number}");

    if !string_rep.len().is_multiple_of(2) {
        return false;
    }

    let chars = string_rep.as_bytes();
    let mid_point = string_rep.len() / 2;

    for pos in 0..mid_point {
        if chars[pos] != chars[mid_point + pos] {
            return false;
        }
    }

    true
}

fn is_invalid_id_part2(number: Number) -> bool {
    let string_rep = format!("{number}");
    is_invalid_id_part2_pattern(string_rep.as_bytes(), 1)
}

fn is_invalid_id_part2_pattern(string_rep: &[u8], pattern_len: usize) -> bool {
    if pattern_len > string_rep.len() / 2 {
        return false;
    }

    if !string_rep.len().is_multiple_of(pattern_len) {
        return is_invalid_id_part2_pattern(string_rep, pattern_len + 1);
    }

    for pattern_len_idx in 0..pattern_len {
        let cc = string_rep[pattern_len_idx];

        // Compare all repetitions
        for repetition in 1..string_rep.len() / pattern_len {
            if cc != string_rep[pattern_len * repetition + pattern_len_idx] {
                // No repeating pattern found => continue with next longer pattern
                return is_invalid_id_part2_pattern(string_rep, pattern_len + 1);
            }
        }
    }

    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/2025/day2_example.txt");
    const INPUT: &str = include_str!("../input/2025/day2.txt");

    #[test]
    fn example_part1() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part1(&input), 1227775554);
    }

    #[test]
    fn input_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), 8576933996);
    }

    #[test]
    fn example_part2() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part2(&input), 4174379265);
    }

    #[test]
    fn input_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), 25663320831);
    }
}
