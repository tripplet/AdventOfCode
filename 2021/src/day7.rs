use rstats::Stats;

const INPUT: &str = include_str!("../input/2021/day7.txt");

pub fn main() {
    let submariens = parse_input(INPUT).unwrap();

    println!("Part1: {}", part1(&submariens));
    println!("Part2: {}", part2(&submariens));
}

fn parse_input(input: &str) -> Result<Vec<u16>, std::num::ParseIntError> {
    input.trim().split(",").map(|nb| nb.parse()).collect()
}

fn part1(submariens: &[u16]) -> usize {
    let median = submariens.median().unwrap().median.round() as isize;
    submariens.iter().fold(0, |sum, &pos| sum + (median - pos as isize).abs() as usize)
}

fn part2(submariens: &[u16]) -> usize {
    let average = submariens.amean().unwrap().round() as isize;
    dbg!(submariens.amean().unwrap());
    submariens.iter().fold(0, |sum, &pos| {
        let distance = (average - (pos as isize)).abs() as f64;
        // Sum of consecutive numbers
        let fuel = ((distance / 2.0) * (1.0 + distance)) as usize;
        sum + fuel
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn part1_example() { assert_eq!(37, part1(&parse_input(EXAMPLE).unwrap())); }

    #[test]
    fn part2_example() { assert_eq!(168, part2(&parse_input(EXAMPLE).unwrap())); }

    #[test]
    fn part1_on_input() { assert_eq!(325528, part1(&parse_input(INPUT).unwrap())); }

    #[test]
    fn part2_on_input() { assert_eq!(85015836, part2(&parse_input(INPUT).unwrap())); }
}
