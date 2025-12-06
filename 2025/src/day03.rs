use aoc_runner_derive::{aoc, aoc_generator};

type Number = u8;
type ParseResult = Vec<Vec<Number>>;

#[aoc_generator(day3)]
pub fn parse_input(input: &str) -> ParseResult {
    input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| unsafe { ch.to_digit(10).unwrap_unchecked() as u8 })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

#[aoc(day3, part1)]
fn part1(batteries: &ParseResult) -> usize {
    get_joltage(batteries, 2)
}

#[aoc(day3, part2)]
fn part2(batteries: &ParseResult) -> usize {
    get_joltage(batteries, 12)
}

#[aoc(day3, part1, old_version)]
fn part1_old(batteries: &ParseResult) -> usize {
    let mut total = 0;
    for row in batteries {
        // Find maximum exept last element
        let first = row[..row.len() - 1]
            .iter()
            .enumerate()
            .max_by(|x, y| (*x.1).cmp(y.1).then(y.0.cmp(&x.0)))
            .unwrap();

        let second = row[first.0 + 1..]
            .iter()
            .enumerate()
            .max_by(|x, y| (*x.1).cmp(y.1).then(y.0.cmp(&x.0)))
            .unwrap();

        total += (*first.1 * 10 + *second.1) as usize;
    }
    total
}

fn get_joltage(batteries: &[Vec<u8>], number_len: u32) -> usize {
    let mut total = 0usize;
    for row in batteries {
        let mut joltage = 0usize;
        let mut start = 0;

        for pos in 0u32..number_len {
            // Find maximum leave enought elements for the remaining digits
            let found_max = row[start..=(row.len() - number_len as usize + pos as usize)]
                .iter()
                .enumerate()
                .max_by(|x, y| (*x.1).cmp(y.1).then(y.0.cmp(&x.0)))
                .unwrap();

            joltage += *found_max.1 as usize * 10usize.pow(number_len - pos - 1);
            start += found_max.0 + 1;
        }

        total += joltage;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/2025/day3_example.txt");
    const INPUT: &str = include_str!("../input/2025/day3.txt");

    #[test]
    fn example_part1() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part1(&input), 357);
    }

    #[test]
    fn input_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), 17435);
    }

    #[test]
    fn example_part2() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part2(&input), 3121910778619);
    }

    #[test]
    fn input_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), 172886048065379);
    }
}
