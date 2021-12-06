pub fn main() {
    let fish = parse_input(include_str!("../input/2021/day6.txt")).unwrap();

    println!("Part1: {}", part1(&fish));
    println!("Part2: {}", part2(&fish));
}

fn parse_input(input: &str) -> Result<Vec<i8>, std::num::ParseIntError> {
    input.trim().split(",").map(|nb| nb.parse()).collect()
}

fn part1(fish: &[i8]) -> usize { run_simulation(fish, 80) }
fn part2(fish: &[i8]) -> usize { run_simulation(fish, 256) }

fn run_simulation(fish_timers: &[i8], days: u16) -> usize {
    let mut fish_timers = fish_timers.clone().to_vec();

    for _ in 0..days {
        let mut new_fish = 0;
        for idx in 0..fish_timers.len() {
            let fish = &mut fish_timers[idx];
            *fish -= 1;

            if *fish == -1 {
                *fish = 6;
                new_fish += 1;
            }
        }

        fish_timers.append(&mut vec![8; new_fish]);
    }

    fish_timers.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(5934, part1(&parse_input("3,4,3,1,2").unwrap()));
    }

    #[test]
    fn part2_example() {
        assert_eq!(26984457539, part2(&parse_input("3,4,3,1,2").unwrap()));
    }
}
