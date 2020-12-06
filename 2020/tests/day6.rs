#[cfg(test)]
mod day6_test {
    use adventofcode::day6;

    #[test]
    fn works_for_sample_input() {
        let input = include_str!("../input/2020/day6_example.txt").trim();

        assert_eq!(day6::part1(&day6::parse(input)), 11);
        assert_eq!(day6::part2(&day6::parse(input)), 6);
    }
}