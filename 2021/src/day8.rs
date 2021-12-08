use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::io::prelude::*;
use std::io::{self, Write};

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
                .map(|digits| {
                    digits
                        .trim()
                        .split_whitespace()
                        .map(|digits| digits.chars().collect::<HashSet<_>>())
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
            .map(|digits| {
                let len = digits.len();
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
    let mut sum = 0;
    for line in data {
        //  aaaa
        // b    c
        // b    c
        //  dddd
        // e    f
        // e    f
        //  gggg

        let mut decryption: HashMap<String, u8> = HashMap::new();

        let d_1 = line[0]
            .iter()
            .find(|digits| digits.len() == 2)
            .ok_or("No 1 found")?;
        let d_4 = line[0]
            .iter()
            .find(|digits| digits.len() == 4)
            .ok_or("No 4 found")?;
        let d_7 = line[0]
            .iter()
            .find(|digits| digits.len() == 3)
            .ok_or("No 7 found")?;
        let d_8 = line[0]
            .iter()
            .find(|digits| digits.len() == 7)
            .ok_or("No 8 found")?;

        let d_235 = line[0]
            .iter()
            .filter(|digits| digits.len() == 5)
            .collect::<Vec<_>>();
        let d_069 = line[0]
            .iter()
            .filter(|digits| digits.len() == 6)
            .collect::<Vec<_>>();
        //dbg!(&d_235);
        //dbg!(&d_069);

        let lines_cf: HashSet<_> = d_1.intersection(&d_7).cloned().collect();
        //dbg!(&lines_cf);

        let d_3 = d_235
            .iter()
            .find(|&d| lines_cf.intersection(d).count() == 2)
            .ok_or("No 3 found")?;
        let d_6 = d_069
            .iter()
            .find(|&d| lines_cf.intersection(d).count() != 2)
            .ok_or("No 6 found")?;
        let d_9 = d_069
            .iter()
            .find(|&d| d_4.intersection(d).count() == 4)
            .ok_or("No 9 found")?;
        let d_0 = d_069
            .iter()
            .find(|&d| d != d_6 && d != d_9)
            .ok_or("No 0 found")?;
        let d_2 = d_235
            .iter()
            .filter(|&d| d != d_3)
            .find(|d_25| d_25.difference(&d_9).count() == 1)
            .ok_or("No 2 found")?;
        let d_5 = d_235
            .iter()
            .find(|&d| d != d_3 && d != d_2)
            .ok_or("No 5 found")?;

        // dbg!(&d_1);
        // dbg!(&d_2);
        // dbg!(&d_3);
        // dbg!(&d_4);
        // dbg!(&d_5);
        // dbg!(&d_6);
        // dbg!(&d_7);
        // dbg!(&d_8);
        // dbg!(&d_9);
        // dbg!(&d_0);

        decryption.insert(d_0.iter().sorted().collect::<String>(), 0);
        decryption.insert(d_1.iter().sorted().collect::<String>(), 1);
        decryption.insert(d_2.iter().sorted().collect::<String>(), 2);
        decryption.insert(d_3.iter().sorted().collect::<String>(), 3);
        decryption.insert(d_4.iter().sorted().collect::<String>(), 4);
        decryption.insert(d_5.iter().sorted().collect::<String>(), 5);
        decryption.insert(d_6.iter().sorted().collect::<String>(), 6);
        decryption.insert(d_7.iter().sorted().collect::<String>(), 7);
        decryption.insert(d_8.iter().sorted().collect::<String>(), 8);
        decryption.insert(d_9.iter().sorted().collect::<String>(), 9);

        //dbg!(&decryption);

        // now finaly decrypt
        let decoded_number = line[1]
            .iter()
            .map(|enc| {
                decryption
                    .get(&enc.iter().sorted().collect::<String>())
                    .ok_or("invalid key")
            })
            .collect::<Result<Vec<_>, _>>()?.iter().rev().enumerate().fold(0, |acc, v|
            acc + **v.1 as usize * (10_usize).pow(v.0 as u32)
        );

        dbg!(decoded_number);

        sum += decoded_number;

        //io::stdout().flush().unwrap();
        //let _ = io::stdin().read(&mut [0u8]).unwrap();
    }

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/2021/day8_example.txt");

    #[test]
    fn part1_example() {
        assert_eq!(26, part1(&parse_input(EXAMPLE)));
    }

    #[test]
    fn part2_example() {
        assert_eq!(61229, part2(&parse_input(EXAMPLE)).unwrap());
    }

    #[test]
    fn part2_example2() {
        assert_eq!(5353, part2(&parse_input("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf")).unwrap());
    }

    //#[test]
    // fn part1_on_input() {
    //     //assert_eq!(, part1(&parse_input(INPUT)));
    // }

    //#[test]
    // fn part2_on_input() {
    //     //assert_eq!(, part2(&parse_input(INPUT)));
    // }
}
