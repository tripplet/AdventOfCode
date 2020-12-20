use std::error::Error;
use std::str::FromStr;
use std::collections::HashMap;

fn main() {
    //let data = parse(include_str!("../input/2020/day19.txt")).unwrap();
    let data_example = parse(include_str!("../input/2020/day19.txt")).unwrap();

    let now = std::time::Instant::now();
    part1(&data_example);
    //println!("Part1: {}  [{}]", part1, humantime::format_duration(now.elapsed()));
    //assert_eq!(part2, 825305207525452);
}

#[derive(Debug)]
struct InputData {
    rules: HashMap<u64, Rule>,
    messages: Vec<Vec<char>>,
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

fn parse(input: &str) -> Result<InputData, Box<dyn Error>> {
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
    Ok(InputData{rules: rules, messages: messages})
}

fn part1(data: &InputData) {
    //dbg!(data);

    let results = data.messages.iter().map(|msg|
        check_message(data, data.rules.get(&0).unwrap(),msg, 0).and_then(|res| Some(res == msg.len())).unwrap_or(false)
    ).collect::<Vec<_>>();

    dbg!(&results);
    dbg!(&results.iter().filter(|r| **r).count());
    //check_message(0, data.messages[0], backtrack_stack)
}

fn check_message(data: &InputData, rule_to_check: &Rule, msg: &Vec<char>, pos: usize) -> Option<usize> {
    let mut result = None;

    //println!("pos: {}, {:?}", pos, rule_to_check);


    if let Rule::Sub(sub_rules) = rule_to_check {
        let mut cur_pos = pos;
        for sub_rule in sub_rules {
            result = check_message(data, sub_rule, msg, cur_pos);
            if let Some(new_pos) = result {
                cur_pos = new_pos
            }
            else {
                break;
            }
        }
    }
    else if let Rule::Or(or_left, or_right) = rule_to_check {
        // try or parts
        if let Some(res) = check_message(data, or_left, msg, pos) {
            result = Some(res);
        } else {
            result = check_message(data, or_right, msg, pos);
        }
    }
    else if let Rule::Ref(ref_rule) = rule_to_check {
        result = check_message(data, data.rules.get(&ref_rule).unwrap(), msg, pos);
    }
    else if let Rule::Character(ch) = rule_to_check {
        if msg[pos] == *ch {
            result = Some(pos+1);
        }
        else {
            result = None;
        }
    }

    //println!("  <- {:?}", &result);
    result
}
