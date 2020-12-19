use std::error::Error;
use std::str::FromStr;
use std::collections::HashMap;

fn main() {

    let data = parse(include_str!("../input/2020/day19.txt"));
    dbg!(data);
}

#[derive(Debug)]
enum Rule {
    Character(char),
    Ref(u64),
    Or(Box<Rule>, Box<Rule>),
    Sub(Vec<Rule>),
}

impl FromStr for Rule {
    type Err = Box<dyn Error>;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input.contains("|") {
            let parts = input.trim().split("|").collect::<Vec<_>>();

            Ok(Rule::Or(
                Box::new(Rule::from_str(parts[0].trim())?),
                Box::new(Rule::from_str(parts[1].trim())?),
            ))
        } else if input.contains("\"") {
            Ok(Rule::Character(input.trim().chars().nth(1).ok_or("invalid \"_\"")?))
        } else {
            input.trim()
                .split(" ")
                .map(|nb| nb.parse::<u64>().and_then(|nb| Ok(Rule::Ref(nb))))
                .collect::<Result<Vec<_>, _>>()
                .and_then(|sub_rules| Ok(Rule::Sub(sub_rules)))
                .map_err(|e| e.into())
        }
    }
}

fn parse(input: &str) -> Result<(HashMap<u64, Rule>, Vec<Vec<char>>), Box<dyn Error>> {
    let mut rules = HashMap::new();
    let mut messages = vec![];
    let mut rule_section = true;

    for line in input.trim().lines() {
        if line.trim().is_empty() {
            rule_section = false;
            continue;
        }

        if rule_section {
            let split_pos = line.find(":").ok_or("missing ':' for id")?;
            let id = line.get(0..split_pos).and_then(|nb| Some(nb.parse::<u64>())).ok_or("invalid id")?;
            rules.insert(id?, Rule::from_str(line.get(split_pos+1..).ok_or("line too short")?)?);
        }
        else {
            messages.push(line.trim().chars().collect::<Vec<_>>());
        }
    }
    Ok((rules, messages))
}
