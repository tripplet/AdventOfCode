use regex::{Regex, Captures};
use std::error::Error;
use std::time::Instant;


#[derive(Debug)]
pub struct PasswordReq {
    min: usize,
    max: usize,
    character: char,
    password: String,
}

impl PasswordReq {
    fn parse(cap: &Captures) -> Option<PasswordReq> {
        Some(PasswordReq {
            min: cap.name("min")?.as_str().parse::<usize>().unwrap(),
            max: cap.name("max")?.as_str().parse::<usize>().unwrap(),
            character: cap.name("character")?.as_str().chars().nth(0)?,
            password: String::from(cap.name("passwd")?.as_str()),
        })
    }

    fn valid_part1(&self) -> bool {
        let char_count = self.password.matches(self.character).count();
        self.min <= char_count && char_count <= self.max
    }

    fn valid_part2(&self) -> bool {
        let pw_chars = self.password.chars().collect::<Vec<char>>();

        return (pw_chars.len() >= self.min && pw_chars[self.min - 1] == self.character)
            ^ (pw_chars.len() >= self.max && pw_chars[self.max - 1] == self.character)
    }
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../input/2020/day2.txt");
    let re = Regex::new(r"^(?P<min>\d+)-(?P<max>\d+) (?P<character>\w):\s* (?P<passwd>\w+)$").unwrap();

    let mut now = Instant::now();
    let passwords = input.trim()
        .lines()
        .map(|line| re.captures(line.trim()).unwrap())
        .map(|cap| PasswordReq::parse(&cap))
        .flatten().collect::<Vec<_>>();

    println!("parsing took: {}", humantime::format_duration(now.elapsed()));
    drop(input);

    now = Instant::now();
    println!("Part1: {}, {}", &passwords.iter().filter(|p| p.valid_part1()).count(), humantime::format_duration(now.elapsed()));

    now = Instant::now();
    println!("Part2: {}, {}", &passwords.iter().filter(|p| p.valid_part2()).count(), humantime::format_duration(now.elapsed()));

    Ok(())
}
