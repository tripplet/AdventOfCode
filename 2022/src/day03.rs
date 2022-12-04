use itertools::Itertools;
use std::collections::HashSet;

type ParseResult = Vec<Vec<char>>;

pub fn parse_input(input: &str) -> ParseResult {
    input.trim().lines().map(|line| line.chars().collect()).collect()
}

fn get_priority(c: char) -> u8 {
    match c {
        'a'..='z' => c as u8 - 'a' as u8 + 1,
        'A'..='Z' => c as u8 - 'A' as u8 + 27,
        _ => panic!(),
    }
}

fn build_hashset(elements: &[char]) -> HashSet<char> {
    HashSet::from_iter(elements.iter().cloned())
}

pub fn part1(input: &ParseResult) -> usize {
    input
        .iter()
        .map(|rucksack| {
            let len = rucksack.len();
            let a: HashSet<char> = build_hashset(&rucksack[0..len / 2]);
            let b: HashSet<char> = build_hashset(&rucksack[len / 2..]);

            a.intersection(&b).map(|c| get_priority(*c) as usize).sum::<usize>()
        })
        .sum()
}

pub fn part2(input: &ParseResult) -> usize {
    let mut result = 0;

    for mut group in input.iter().chunks(3).into_iter() {
        // First elf (badges)
        let badges = build_hashset(group.next().unwrap());

        // Combined with 2nd elf
        let badges: HashSet<_> = badges
            .intersection(&build_hashset(group.next().unwrap()))
            .cloned()
            .collect();

        // Combined with 3rd elf
        let last_elf = build_hashset(group.next().unwrap());
        let mut badges = badges.intersection(&last_elf);

        if let Some(badge) = badges.next() {
            result += get_priority(*badge) as usize;
        }
    }

    result
}
