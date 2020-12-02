use std::error::Error;
use std::time::{Instant};
//use std::fs;

use itertools::Itertools;

const SUM: usize = 2020;

pub fn main() -> Result<(), Box<dyn Error>> {
    //let input = fs::read_to_string("input/2020/day1.txt")?;
    let input = include_str!("../input/2020/day1.txt");

    let entries = input
        .trim()
        .lines()
        .map(|line| line.trim().parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    drop(input);

    let mut now = Instant::now();
    println!("Part1: {}, {}", part1(&entries, SUM).unwrap(), humantime::format_duration(now.elapsed()));

    now = Instant::now();
    println!("Part2: {}, {}", part2(&entries, SUM).unwrap(), humantime::format_duration(now.elapsed()));

    now = Instant::now();
    println!("Part2 (itertools): {}, {}", part2_itertools(&entries, SUM).unwrap(), humantime::format_duration(now.elapsed()));

    Ok(())
}

pub fn part1(entries: &[usize], target_sum: usize) -> Option<usize> {
    for idx in 0..entries.len() {
        for idx2 in 0..entries.len() {
            if idx == idx2 {
                continue;
            }

            if entries[idx] + entries[idx2] == target_sum {
                return Some(entries[idx] * entries[idx2]);
            }
        }
    }

    None
}

fn part2(entries: &[usize], target_sum: usize) -> Option<usize> {
    let len = entries.len();

    for idx in 0..len {
        'level2: for idx2 in 0..len {
            if idx == idx2 {
                continue 'level2;
            }

            'level3: for idx3 in 0..len {
                if idx3 == idx2 || idx3 == idx {
                    continue 'level3;
                }

                if entries[idx] + entries[idx2] + entries[idx3] == target_sum {
                    return Some(entries[idx] * entries[idx2] * entries[idx3]);
                }
            }
        }
    }

    None
}

fn part2_itertools(entries: &Vec<usize>, target_sum: usize) -> Option<usize> {
    let pairs = entries.into_iter().combinations(3);

    for pair in pairs {
        if pair.iter().map(|&v| v).sum1::<usize>()? == target_sum {
            return pair.into_iter().map(|&v| v).fold1(|a, b| a * b);
        }
    }

    None
}