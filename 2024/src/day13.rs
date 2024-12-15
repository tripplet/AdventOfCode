use aoc_runner_derive::{aoc, aoc_generator};
use glam::I64Vec2;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

type ParseResult = Vec<ClawMachine>;

#[derive(Debug)]
pub struct ClawMachine {
    button_a: I64Vec2,
    button_b: I64Vec2,
    prize: I64Vec2,
}

#[aoc_generator(day13)]
pub fn parse_input(input: &str) -> ParseResult {
    let input = input.trim().replace("\r", "");

    input
        .split("\n\n")
        .map(|machine| {
            let lines = machine.trim().lines().collect::<Vec<_>>();

            let button_a = lines[0]
                .strip_prefix("Button A: X+")
                .unwrap()
                .split_once(", Y+")
                .unwrap();
            let button_b = lines[1]
                .strip_prefix("Button B: X+")
                .unwrap()
                .split_once(", Y+")
                .unwrap();
            let prize = lines[2].strip_prefix("Prize: X=").unwrap().split_once(", Y=").unwrap();

            let button_a = I64Vec2::new(button_a.0.parse().unwrap(), button_a.1.parse().unwrap());
            let button_b = I64Vec2::new(button_b.0.parse().unwrap(), button_b.1.parse().unwrap());
            let prize = I64Vec2::new(prize.0.parse().unwrap(), prize.1.parse().unwrap());

            ClawMachine {
                button_a,
                button_b,
                prize,
            }
        })
        .collect()
}

fn analytic_solution(machine: &ClawMachine) -> Option<(i64, i64)> {
    let det1 = machine.prize.x as f64 * machine.button_b.y as f64 - machine.button_b.x as f64 * machine.prize.y as f64;
    let det2 = machine.button_a.x as f64 * machine.prize.y as f64 - machine.prize.x as f64 * machine.button_a.y as f64;

    let det =
        machine.button_a.x as f64 * machine.button_b.y as f64 - machine.button_b.x as f64 * machine.button_a.y as f64;

    let a = (det1 / det) as i64;
    let b = (det2 / det) as i64;

    if a * machine.button_a + b * machine.button_b != machine.prize {
        return None;
    }

    Some((a, b))
}

fn brute_force_solution(machine: &ClawMachine) -> Option<(i64, i64)> {
    let mut min_tokens = usize::MAX;
    let mut result = None;

    for a in 1..=100 {
        for b in 1..=100 {
            if machine.button_a * a + machine.button_b * b == machine.prize {
                let tokens = (a * 3 + b) as usize;
                if tokens < min_tokens {
                    min_tokens = tokens;
                    result = Some((a, b));
                }
            }
        }
    }

    result
}

#[aoc(day13, part1, brute_force)]
pub fn part1_brute_force(input: &ParseResult) -> u64 {
    input
        .par_iter()
        .map(|machine| match brute_force_solution(machine) {
            Some((a, b)) => a as u64 * 3 + b as u64,
            None => 0,
        })
        .sum()
}

#[aoc(day13, part1, analytic)]
pub fn part1_analytic(input: &ParseResult) -> u64 {
    input
        .iter()
        .map(|machine| match analytic_solution(machine) {
            Some((a, b)) if a <= 100 && b <= 100 => a as u64 * 3 + b as u64,
            None => 0,
            _ => 0,
        })
        .sum()
}

#[aoc(day13, part2)]
pub fn part2(input: &ParseResult) -> u64 {
    input
        .par_iter()
        .map(|machine| {
            let machine_10x = ClawMachine {
                button_a: machine.button_a,
                button_b: machine.button_b,
                prize: machine.prize + 10000000000000,
            };

            match analytic_solution(&machine_10x) {
                Some((a, b)) => (a * 3 + b) as u64,
                None => 0,
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/2024/day13_example.txt");
    const INPUT: &str = include_str!("../input/2024/day13.txt");

    #[test]
    fn example_part1() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part1_brute_force(&input), 480);
        assert_eq!(part1_analytic(&input), 480);
    }

    #[test]
    fn input_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1_brute_force(&input), 25751);
        assert_eq!(part1_analytic(&input), 25751);
    }

    #[test]
    fn input_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), 108528956728655);
    }
}
