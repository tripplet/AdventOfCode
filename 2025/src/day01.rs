use aoc_runner_derive::{aoc, aoc_generator};

type Number = i16;
type ParseResult = Vec<i16>;

#[aoc_generator(day1)]
pub fn parse_input(input: &str) -> ParseResult {
    input
        .trim()
        .lines()
        .map(|line| {
            let value = line[1..].parse::<Number>().unwrap();
            if line.starts_with('R') { value } else { -value }
        })
        .collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &ParseResult) -> isize {
    let mut pos = 50;
    let mut times_at_zero = 0;
    for &rot in input {
        pos += rot;
        if rot > 0 {
            pos %= 100;
        } else {
            while pos < 0 {
                pos += 100;
            }
        }

        pos %= 100;

        if pos == 0 {
            times_at_zero += 1;
        }
    }

    times_at_zero
}

#[aoc(day1, part2)]
pub fn part2(input: &ParseResult) -> isize {
    let mut pos = 50;
    let mut pos_prev = 50;
    let mut times_zero = 0;
    for &rot in input {
        let mut time_over_zero = 0;

        pos += rot;

        if rot > 0 {
            while pos > 100 {
                pos -= 100;
                time_over_zero += 1;
            }
        } else {
            while pos < 0 {
                pos += 100;
                if pos_prev != 0 {
                    time_over_zero += 1;
                }

                pos_prev = pos;
            }
        }

        pos %= 100;

        if pos == 0 {
            times_zero += 1;
        }

        pos_prev = pos;
        times_zero += time_over_zero;
    }

    times_zero
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/2025/day1_example.txt");
    const INPUT: &str = include_str!("../input/2025/day1.txt");

    #[test]
    fn example_part1() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part1(&input), 3);
    }

    #[test]
    fn input_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), 1118);
    }

    #[test]
    fn example_part2() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part2(&input), 6);
    }

    #[test]
    fn input_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), 6289);
    }
}
