use std::error::Error;
use regex::{Regex, Captures};
use std::collections::HashMap;

pub fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../input/2020/day4.txt").trim();

    let passports = parse(input);
    drop(input);


    assert_eq!(check_pid("0000000000"), false);
    assert_eq!(check_pid("000000000"), true);

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

    let mut checks: HashMap<&str, fn(&str) -> bool> = HashMap::new();
    checks.insert("byr", check_bry);
    checks.insert("iyr", check_iyr);
    checks.insert("eyr", check_eyr);
    checks.insert("hgt", check_hgt);
    checks.insert("hcl", check_hcl);
    checks.insert("ecl", check_ecl);
    checks.insert("pid", check_pid);
    checks.insert("cid", |_| true);

    for passport in passports {
        //for (k, v) in passport {
            //println!("{:}={:}", k, v);
        //}

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

fn check_bry(i: &str) -> bool {
    let year = i.parse::<isize>().unwrap_or(-1);
    1920 <= year && year <= 2002
}

fn check_iyr(i: &str) -> bool {
    let year = i.parse::<isize>().unwrap_or(-1);
    2010 <= year && year <= 2020
}

fn check_eyr(i: &str) -> bool {
    let year = i.parse::<isize>().unwrap_or(-1);
    2020 <= year && year <= 2030
}

fn check_pid(i: &str) -> bool {
    Regex::new(r"^\d{9}$").unwrap().is_match(i)
}

fn check_ecl(i: &str) -> bool {
    ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].iter().any(|s| **s == *i)
}

fn check_hcl(i: &str) -> bool {
    Regex::new(r"^#[a-f0-9]{6}$").unwrap().is_match(i)
}

fn check_hgt(i: &str) -> bool {
    let parsed = Regex::new(r"^(\d+)(cm|in)$").unwrap().captures(i);

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