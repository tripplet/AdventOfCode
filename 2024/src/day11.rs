use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

type Number = u64;
type ParseResult = Vec<Number>;

#[aoc_generator(day11)]
pub fn parse_input(input: &str) -> ParseResult {
    input.split_whitespace().map(|nb| nb.parse().unwrap()).collect()
}

enum Blink {
    Single(u64),
    Double([u64; 2]),
}

fn single_blink(stone: u64) -> Blink {
    if stone == 0 {
        Blink::Single(1)
    } else {
        let digits = ((stone as f64).log10().floor() + 1f64).ceil() as u8;

        if digits % 2 == 0 {
            let power = 10f64.powf(f64::from(digits) / 2.0) as u64;
            let x = (stone) / power;
            let y = (stone) - (x * power);

            Blink::Double([x, y])
        } else {
            Blink::Single(stone * 2024)
        }
    }
}

fn blink(stone: u64, times: usize, cache: &mut HashMap<(u64, usize), u64>) -> u64 {
    if times == 0 {
        return 1;
    }

    if let Some(result) = cache.get(&(stone, times)) {
        return *result;
    }

    let result = match single_blink(stone) {
        Blink::Single(new_stone) => blink(new_stone, times - 1, cache),
        Blink::Double([a, b]) => blink(a, times - 1, cache) + blink(b, times - 1, cache),
    };

    cache.insert((stone, times), result);

    result
}

#[aoc(day11, part1)]
pub fn part1(input: &ParseResult) -> u64 {
    let mut cache = HashMap::new();

    input.iter().map(|stone| blink(*stone, 25, &mut cache)).sum()
}

#[aoc(day11, part2)]
pub fn part2(input: &ParseResult) -> u64 {
    let mut cache = HashMap::new();
    input.iter().map(|stone| blink(*stone, 75, &mut cache)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/2024/day11_example.txt");
    const INPUT: &str = include_str!("../input/2024/day11.txt");

    #[test]
    fn example_part1() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part1(&input), 55312);
    }

    #[test]
    fn input_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), 203609);
    }

    #[test]
    fn input_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), 240954878211138);
    }
}
