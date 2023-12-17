use aoc_runner_derive::{aoc, aoc_generator};

type Number = i32;
type ParseResult = Vec<Number>;

#[aoc_generator(day1)]
pub fn parse_input(input: &str) -> ParseResult {
    vec![]
}

#[aoc(day1, part1)]
pub fn part1(input: &ParseResult) -> isize {
    42
}

#[aoc(day1, part2)]
pub fn part2(input: &ParseResult) -> isize {
    42
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/2023/day1_example.txt");
    const INPUT: &str = include_str!("../input/2023/day1.txt");

    #[test]
    fn example_part1() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part1(&input), todo!());
    }

    //#[test]
    fn input_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), todo!());
    }

    //#[test]
    fn example_part2() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part2(&input), todo!());
    }

    //#[test]
    fn input_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), 42);
    }
}
