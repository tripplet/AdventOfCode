pub fn main() {
    let input = include_str!("../input/2020/day9.txt").trim();

    let numbers = parse(input).unwrap();
    drop(input);

    let mut now = std::time::Instant::now();
    let part1_solution = part1(&numbers, 25).unwrap();
    println!("Part1: {}  [{}]", part1_solution, humantime::format_duration(now.elapsed()));

    assert_eq!(part1_solution, 57195069);

    now = std::time::Instant::now();
    println!("Part2: {}  [{}]", part2(&numbers, part1_solution).unwrap(), humantime::format_duration(now.elapsed()));
}

pub fn parse(input: &str) -> Result<Vec<i64>, std::num::ParseIntError> {
    input.trim().lines().map(|l| l.trim().parse::<i64>()).collect()
}

fn find_sum(numbers: &[i64], sum: i64) -> Option<(usize, usize)> {
    for idx0 in 0..numbers.len() {
        for idx1 in 0..numbers.len() {
            if idx0 == idx1 {
                continue;
            }

            if idx0 != idx1 && numbers[idx0] + numbers[idx1] == sum {
                return Some((idx0, idx1));
            }
        }
    }

    None
}

pub fn part1(numbers: &[i64], prefix_range: usize) -> Option<i64> {
    for check_idx in prefix_range..numbers.len() {
        let result = find_sum(&numbers[check_idx - prefix_range..check_idx], numbers[check_idx]);
        if result.is_none() {
            return Some(numbers[check_idx]);
        }
    }

    None
}

pub fn part2(numbers: &[i64], target_sum: i64) -> Option<i64> {
    for start_idx in 0..numbers.len() {
        let mut sum = 0;
        let mut pos = 0;

        while sum < target_sum && start_idx + pos < numbers.len() {
            sum += numbers[start_idx + pos];
            pos += 1;
        }

        if sum != target_sum {
            continue;
        }

        let target_range = &numbers[start_idx..start_idx + pos - 1];
        return Some(target_range.iter().min()? + target_range.iter().max()?);
    }

    None
}
