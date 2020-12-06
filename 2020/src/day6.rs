use std::error::Error;

type Groups<'a> = Vec<Vec<&'a str>>;

pub fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../input/2020/day6.txt").trim();
    let groups = parse(input);
    drop(input);

    println!("Part1: {}", part1(&groups));
    println!("Part2: {}", part2(&groups));

    Ok(())
}

pub fn part1(groups: &Groups) -> u64 {
    let mut cnt: u64 = 0;

    for persons in groups {
        let mut answers = [0; 26];

        for person in persons {
            for answer in person.chars() {
                answers[(answer as u64 - 'a' as u64) as usize] = 1;
            }
        }

        cnt += answers.iter().sum::<u64>() as u64;
    }

    cnt
}

pub fn part2(groups: &Groups) -> u64 {
    let mut cnt: u64 = 0;

    for persons in groups {
        let mut answers = [0; 26];

        for person in persons {
            for answer in person.chars() {
                answers[(answer as u64 - 'a' as u64) as usize] += 1;
            }
        }

        cnt += answers.iter().filter(|a| **a == persons.len()).count() as u64;
        println!("");
    }

    cnt
}

pub fn parse(input: &str) -> Groups {
    let mut groups: Groups = Vec::new();
    groups.push(Vec::new());

    for line in input.lines().map(|l| l.trim()) {
        if line.is_empty() {
            groups.push(Vec::new());
            continue;
        }

        groups.last_mut().unwrap().push(line);
    }

    groups
}
