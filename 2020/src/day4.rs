use std::error::Error;
use regex::{Regex};
use std::collections::HashMap;

pub fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../input/2020/day4.txt").trim();

    let passports = parse(input);
    drop(input);

    println!("Part1: {}", part1(&passports));
    println!("Part2: {}", part2(&passports));

    Ok(())
}

fn part1(passports: &Vec<HashMap<&str, &str>>) -> isize {
    let mut valid_passports = 0;
    let tags = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]; // "cid"

    for passport in passports {
        let valid_count: usize = tags.iter().filter(|t| passport.get(*t).is_some()).count();
        if valid_count >= 7 {
            valid_passports += 1;
        }
    }

    valid_passports
}

fn part2(passports: &Vec<HashMap<&str, &str>>) -> isize {
    let mut valid_passports = 0;

    let tags = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]; // "cid"
    let re_pid = Regex::new(r"^\d{9}$").unwrap();
    let re_hgt = Regex::new(r"^(\d+)(cm|in)$").unwrap();
    let re_hcl = Regex::new(r"^#[a-f0-9]{6}$").unwrap();

    let mut checks: HashMap<&str, Box<dyn Fn(&str) -> bool>> = HashMap::new();
    checks.insert("byr", Box::new(|i| check_min_max(i, 1920, 2002)));
    checks.insert("iyr", Box::new(|i| check_min_max(i, 2010, 2020)));
    checks.insert("eyr", Box::new(|i| check_min_max(i, 2020, 2030)));
    checks.insert("hgt", Box::new(|i| check_hgt(i, &re_hgt)));
    checks.insert("hgt", Box::new(|i| check_hgt(i, &re_hgt)));
    checks.insert("hcl", Box::new(|i| re_hcl.is_match(i)));
    checks.insert("ecl", Box::new(check_ecl));
    checks.insert("pid", Box::new(|i| re_pid.is_match(i)));
    checks.insert("cid", Box::new(|_| true));

    for passport in passports {
        let valid_count: usize = tags.iter().filter(|t| passport.get(*t).is_some()).count();

        if valid_count >= 7 {
            if passport.iter().map(|(k, v)| checks.get(*k).unwrap()(*v)).all(|c| c == true) {
                valid_passports += 1;
            }
        }
    }

    valid_passports
}

fn parse(input: &str) -> Vec<HashMap<&str, &str>> {
    let re = Regex::new(r"(?m)(([^\s:]+:[^\s:]+)\s?)+").unwrap();
    let re2 = Regex::new(r"(?P<key>[^\s:]+):(?P<value>[^\s:]+)").unwrap();

    let mut passports = Vec::new();

    for set in re.find_iter(input) {
        let mut passport: HashMap<&str, &str> = HashMap::new();
        for kvp in re2.captures_iter(set.as_str().trim()) {
            let key = kvp.name("key").unwrap().as_str();

            let existing = passport.get(key);
            if existing.is_some() {
                panic!("invalid");
            }

            passport.insert(key, kvp.name("value").unwrap().as_str());
        }

        passports.push(passport);
    }

    passports
}

fn check_min_max(i: &str, min: isize, max: isize) -> bool {
    let number = i.parse::<isize>().unwrap_or(-1);
    min <= number && number <= max
}

fn check_ecl(i: &str) -> bool {
    ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].iter().any(|s| **s == *i)
}

fn check_hgt(i: &str, re: &Regex) -> bool {
    let parsed = re.captures(i);

    if parsed.is_none() {
        return false;
    }

    let height = parsed.as_ref().unwrap().get(1).unwrap().as_str().parse::<isize>().unwrap();

    match parsed.unwrap().get(2).unwrap().as_str() {
        "cm" => 150 <= height && height <= 193,
        "in" => 59 <= height && height <= 76,
        _ => false
    }
}