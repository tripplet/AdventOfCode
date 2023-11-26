type Number = i32;
type ParseResult = Vec<Number>;

pub fn parse_input(input: &str) -> ParseResult {
    vec![]
}

pub fn part1(input: &ParseResult) -> isize {
    42
}

pub fn part2(input: &ParseResult) -> isize {
    42
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/2023/day_example.txt");
    const INPUT: &str = include_str!("../input/2023/day.txt");

    #[test]
    fn example_part1() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part1(&input), 42);
    }

    #[test]
    fn example_part2() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part2(&input), 42);
    }

    #[test]
    fn input_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), 42);
    }

    #[test]
    fn input_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), 42);
    }
}
