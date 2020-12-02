#[cfg(test)]
mod day1_test {
    use adventofcode::day1;

    const SAMPLE_INPUT: [usize; 6] = [1721, 979, 366, 299, 675, 1456];

    #[test]
    fn part1_works_for_sample_input() {
        assert_eq!(day1::part1(&SAMPLE_INPUT, 2020).unwrap(), 5145729);
    }
}