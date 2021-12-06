pub fn main() {
    let fish = parse_input(include_str!("../input/2021/day6.txt")).unwrap();

    println!("Part1: {}", part1(&fish));
    println!("Part2: {}", part2(&fish));
}

fn parse_input(input: &str) -> Result<Vec<i8>, std::num::ParseIntError> {
    input.trim().split(",").map(|nb| nb.parse()).collect()
}

fn part1(fish: &[i8]) -> usize { calculate_generations(fish, 80) }
fn part2(fish: &[i8]) -> usize { calculate_generations(fish, 256) }

fn calculate_generations(fish_timers: &[i8], days: u16) -> usize {
    let fish_timers = fish_timers.clone().to_vec();

    let mut fish_counter = [0; 9];
    fish_timers
        .iter()
        .for_each(|&f| fish_counter[f as usize] += 1);

    for _ in 0..days {
        fish_counter.rotate_left(1);
        fish_counter[6] += fish_counter[8];
    }

    fish_counter.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "3,4,3,1,2";

    #[test]
    fn part1_example() {
        assert_eq!(5934, part1(&parse_input(EXAMPLE_INPUT).unwrap()));
    }

    #[test]
    fn part2_example() {
        assert_eq!(26984457539, part2(&parse_input(EXAMPLE_INPUT).unwrap()));
    }
}
