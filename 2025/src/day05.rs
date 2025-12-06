use aoc_runner_derive::{aoc, aoc_generator};

type Number = usize;
type FreshRange = std::ops::RangeInclusive<Number>;

pub struct ParseResult {
    ranges: Vec<FreshRange>,
    values: Vec<Number>,
}

#[aoc_generator(day5)]
pub fn parse_input(input: &str) -> ParseResult {
    let input = input.trim().replace('\r', "");
    let (ranges, values) = input.split_once("\n\n").unwrap();

    ParseResult {
        ranges: ranges
            .trim()
            .lines()
            .map(|line| {
                let parts = line.trim().split_once('-').unwrap();
                parts.0.parse::<Number>().unwrap()..=parts.1.parse::<Number>().unwrap()
            })
            .collect(),
        values: values
            .trim()
            .lines()
            .map(|line| line.parse::<Number>().unwrap())
            .collect(),
    }
}

#[aoc(day5, part1)]
pub fn part1(input: &ParseResult) -> usize {
    input
        .values
        .iter()
        .filter(|&value| {
            for range in &input.ranges {
                if range.contains(value) {
                    return true;
                }
            }
            false
        })
        .count()
}

#[aoc(day5, part2)]
pub fn part2(input: &ParseResult) -> usize {
    let mut ranges = input.ranges.clone();

    'from_the_start: loop {
        for idx1 in 0..ranges.len() {
            for idx2 in 0..ranges.len() {
                if idx2 == idx1 {
                    continue;
                }

                // Try merging the ranges
                if let Some(new_range) = merge_ranges(&ranges[idx1], &ranges[idx2]) {
                    // remove the ranges
                    ranges.remove(idx1.max(idx2));
                    ranges.remove(idx1.min(idx2));
                    ranges.push(new_range);

                    continue 'from_the_start;
                }
            }
        }

        // No more range to merge
        break;
    }

    ranges
        .iter()
        .map(|range| range.end() - range.start() + 1)
        .sum()
}

fn merge_ranges(r1: &FreshRange, r2: &FreshRange) -> Option<FreshRange> {
    if r1.contains(r2.start())
        || r1.contains(r2.end())
        || r2.contains(r1.start())
        || r2.contains(r1.end())
    {
        Some(*r1.start().min(r2.start())..=*r1.end().max(r2.end()))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/2025/day5_example.txt");
    const INPUT: &str = include_str!("../input/2025/day5.txt");

    #[test]
    fn example_part1() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part1(&input), 3);
    }

    #[test]
    fn input_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), 862);
    }

    #[test]
    fn example_part2() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part2(&input), 14);
    }

    #[test]
    fn input_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), 357907198933892);
    }
}
