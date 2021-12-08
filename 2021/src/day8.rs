use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::error::Error;

const INPUT: &str = include_str!("../input/2021/day8.txt");

// Ugly mega Vector
type Data = Vec<Vec<Vec<HashSet<char>>>>;

pub fn main() {
    let data = parse_input(INPUT);

    println!("Part1: {}", part1(&data));
    println!("Part2: {}", part2(&data).unwrap());
}

fn parse_input(input: &str) -> Data {
    input
        .trim()
        .lines()
        .map(|line| {
            line.split("|")
                .map(|sym| {
                    sym.trim()
                        .split_whitespace()
                        .map(|sym| sym.chars().collect::<HashSet<_>>())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Data>()
}

fn part1(data: &Data) -> usize {
    data.iter().fold(0, |sum, line| {
        sum + line[1]
            .iter()
            .map(|sym| {
                let len = sym.len();
                if len == 2 || len == 4 || len == 3 || len == 7 {
                    1
                } else {
                    0
                }
            })
            .sum::<usize>()
    })
}

fn part2(data: &Data) -> Result<usize, Box<dyn Error>> {
    //  aaaa
    // b    c
    // b    c
    //  dddd
    // e    f
    // e    f
    //  gggg
    let mut sum = 0;
    for line in data {
        let scrambled_digits = &line[0];
        let mut dig: Vec<Option<&HashSet<char>>> = vec![None; 10];
        let mut decryption: HashMap<String, u8> = HashMap::new();

        dig[1] = Some(scrambled_digits.iter().find(|sym| sym.len() == 2).ok_or("1 not found")?,);
        dig[4] = Some(scrambled_digits.iter().find(|sym| sym.len() == 4).ok_or("4 not found")?,);
        dig[7] = Some(scrambled_digits.iter().find(|sym| sym.len() == 3).ok_or("7 not found")?,);
        dig[8] = Some(scrambled_digits.iter().find(|sym| sym.len() == 7).ok_or("8 not found")?,);

        let digits_235: Vec<_> = scrambled_digits.iter().filter(|sym| sym.len() == 5).collect();
        let digits_069: Vec<_> = scrambled_digits.iter().filter(|sym| sym.len() == 6).collect();

        dig[6] = Some(digits_069.iter()
                .find(|&d| dig[1].unwrap().intersection(d).count() != 2)
                .copied().ok_or("6 not found")?);

        dig[9] = Some(digits_069.iter()
                .find(|&d| dig[4].unwrap().intersection(d).count() == 4)
                .copied().ok_or("9 not found")?,);

        dig[0] = Some(digits_069.iter()
                .find(|&&d| d != dig[6].unwrap() && d != dig[9].unwrap())
                .copied().ok_or("0 not found")?);

        dig[3] = Some(digits_235.iter()
                .find(|&d| d.difference(dig[1].unwrap()).count() == 3)
                .copied().ok_or("3 not found")?);

        dig[2] = Some(digits_235.iter()
                .filter(|&&d| d != dig[3].unwrap())
                .find(|d_25| d_25.difference(dig[9].unwrap()).count() == 1)
                .copied().ok_or("2 not found")?);

        dig[5] = Some(digits_235.iter()
                .find(|&&d| d != dig[3].unwrap() && d != dig[2].unwrap())
                .copied().ok_or("5 not found")?);

        // Build decryption map
        for (idx, symbols) in dig.iter().enumerate() {
            decryption.insert(symbols.unwrap().iter().sorted().collect::<String>(), idx as u8);
        }

        // now finally decrypt
        let decoded_number = line[1]
            .iter()
            .map(|enc|
                decryption
                    .get(&enc.iter().sorted().collect::<String>())
                    .ok_or("invalid key"))
            .collect::<Result<Vec<_>, _>>()?
            .iter().rev().enumerate()
            .fold(0, |acc, v| acc + **v.1 as usize * (10_usize).pow(v.0 as u32));

        sum += decoded_number;
    }
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_SINGLE: &str = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
    const EXAMPLE: &str = include_str!("../input/2021/day8_example.txt");

    #[test]
    fn part1_example() {
        assert_eq!(26, part1(&parse_input(EXAMPLE)));
    }

    #[test]
    fn part2_example_single_line() {
        assert_eq!(5353, part2(&parse_input(EXAMPLE_SINGLE)).unwrap());
    }

    #[test]
    fn part2_example() {
        assert_eq!(61229, part2(&parse_input(EXAMPLE)).unwrap());
    }

    #[test]
    fn part1_on_input() {
        assert_eq!(362, part1(&parse_input(INPUT)));
    }

    #[test]
    fn part2_on_input() {
        assert_eq!(1020159, part2(&parse_input(INPUT)).unwrap());
    }
}
