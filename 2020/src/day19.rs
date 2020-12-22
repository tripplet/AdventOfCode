use std::error::Error;
use std::str::FromStr;
use std::collections::HashMap;

fn main() {
    //let data = parse(include_str!("../input/2020/day19.txt")).unwrap();
    let data_example = parse(include_str!("../input/2020/day19.txt")).unwrap();

    let now = std::time::Instant::now();
    let part1_result = part1(&data_example);
    println!("Part1: {}  [{}]", part1_result, humantime::format_duration(now.elapsed()));
    assert_eq!(part1_result, 156);

    let now = std::time::Instant::now();
    let part2 = part2(&parse(include_str!("../input/2020/day19.txt")).unwrap());
    println!("Part2: {}  [{}]", part2, humantime::format_duration(now.elapsed()));
    assert_eq!(part2, 363);
}

#[derive(Debug)]
struct InputData {
    rules: HashMap<u64, Rule>,
    messages: Vec<Vec<char>>,
}

#[derive(Debug, Clone)]
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

fn part1(data: &InputData) -> usize {
    //dbg!(data);

    let results = data.messages.iter().map(|msg|
        check_message(data, data.rules.get(&0).unwrap(),msg, 0).iter().any(|r| *r == msg.len())
    ).collect::<Vec<_>>();

    //dbg!(&results);
    results.iter().filter(|r| **r).count()
    //check_message(0, data.messages[0], backtrack_stack)
}

fn part2(orig_data: &InputData) -> usize {
    let mut patched_rules = orig_data.rules.clone();
    patched_rules.insert(8, Rule::from_str("42 | 42 8").unwrap());
    patched_rules.insert(11, Rule::from_str("42 31 | 42 11 31").unwrap());

    let data = InputData{rules: patched_rules, messages: orig_data.messages.clone()};
    //let data = InputData{rules: patched_rules, messages: vec![orig_data.messages[2].clone()]};
    //let data = InputData{rules: patched_rules, messages: vec!["bbbbbbbaaaabbbbaaabbabaaa".chars().collect()]};

    let results = data.messages.iter().map(|msg|
        (msg.len(), check_message(&data, data.rules.get(&0).unwrap(),msg, 0))
    ).collect::<Vec<_>>();

    //dbg!(&results);

    let rr = results.iter().map(|results| results.1.iter().any(|res| *res == results.0)).collect::<Vec<_>>();

    //dbg!(&rr);
    //.and_then(|res| Some(format!("{} = {}",res, msg.len()))).unwrap_or("None".into())

    //dbg!(&results);
    rr.iter().filter(|r| **r).count()
    //check_message(0, data.messages[0], backtrack_stack)
    //0
}

fn check_message(data: &InputData, rule_to_check: &Rule, msg: &Vec<char>, pos: usize) -> Vec<usize> {

    //println!("pos: {}, {:?}", pos, rule_to_check);

    let result = match rule_to_check {
        Rule::Sub(rule) => {
            let mut positions = vec![pos];

            for sub_rule in rule {
                let mut sub_result = vec![];
                for cur_position in positions {
                    if cur_position < msg.len() {
                        sub_result.extend(check_message(data, sub_rule, msg, cur_position));
                    }
                }

                positions = sub_result;
            }

            positions
        },
        Rule::Or(or_left, or_right) => {
            // try both or parts (necessary for loops) and combine them
            let left = check_message(data, or_left, msg, pos);
            let right = check_message(data, or_right, msg, pos);

            [&left[..], &right[..]].concat()
        },
        Rule::Ref(ref_rule) => check_message(data, data.rules.get(&ref_rule).unwrap(), msg, pos),
        Rule::Character(ch) => {
            if msg[pos] == *ch {
                vec![pos+1]
            }
            else {
                vec![]
            }
        }
    };

    //println!("    {:?}  <- {:?}", rule_to_check, &result);
    result
}
