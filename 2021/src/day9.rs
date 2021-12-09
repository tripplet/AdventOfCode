use std::error::Error;

const INPUT: &str = include_str!("../input/2021/day9.txt");
const EXAMPLE: &str = include_str!("../input/2021/day9_example.txt");

type Data = Vec<Vec<u8>>;

pub fn main() {
    let data = parse_input(INPUT);

    println!("Part1: {}", part1(&data));
    println!("Part2: {}", part2(&data));
}

fn parse_input(input: &str) -> Data {
    input
        .trim()
        .lines()
        .map(|line| line.chars().map(|d| d.to_digit(10).unwrap() as u8).collect())
        .collect()
}

fn part1(data: &Data) -> usize {
    let leny = data.len() as isize;
    let lenx = data[0].len() as isize;

    let mut sum = 0;

    for y in 0..leny as usize {
        for x in 0..lenx as usize {
            let adjescent = [
                if y as isize - 1 >= 0 {Some(data[y-1][x])} else { None },
                if y as isize + 1 < leny {Some(data[y+1][x])} else { None },
                if x as isize - 1 >= 0 {Some(data[y][x-1])} else { None },
                if x as isize + 1 < lenx {Some(data[y][x+1])} else { None },
            ];

            if adjescent.iter().filter_map(|x| *x).all(|adj| adj > data[y][x]) {
                sum += data[y][x] as usize + 1;
            }
        }
    }
    sum
}

// fn part2(data: ) -> usize {

// }

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "";

    // #[test]
    // fn part1_example() { assert_eq!(, part1(&parse_input(EXAMPLE).unwrap())); }

    // #[test]
    // fn part2_example() { assert_eq!(, part2(&parse_input(EXAMPLE).unwrap())); }

    // #[test]
    // fn part1_on_input() { assert_eq!(, part1(&parse_input(INPUT).unwrap())); }

    // #[test]
    // fn part2_on_input() { assert_eq!(, part2(&parse_input(INPUT).unwrap())); }
}
