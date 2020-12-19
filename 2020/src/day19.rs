
#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::str::FromStr;
use std::error::Error;

fn main() {

}

lazy_static! {
    static ref REGEX_RULE: Regex = Regex::new(
        r#"(?m)^((?>(?P<r1>\d+) (?P<r2>\d+))|(?>(?P<r3>\d+) (?P<r4>\d+) \| (?P<r5>\d+) (?P<r6>\d+))|(?>"(?P<char>\w)"))$"#
    )
    .unwrap();
}

enum Rule {
    Character(char),
    Ref(u64),
    Or(Box<Rule>, Box<Rule>),
    Sub(Vec<Rule>),
}

trait ToNumber {
    fn to_number(self) -> Option<u64>;
}

impl ToNumber for Option<regex::Match<'_>> {
    fn to_number(self) -> Option<u64> {
        self.and_then(|m| Some(m.as_str())).and_then(|s| s.parse::<u64>().ok())
    }
}

impl FromStr for Rule {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if (s.contains("|")) {
            OK(Rule::Or(Rule::from_str(s: &str)))
        }

        let rule = REGEX_RULE.captures(s);

        if let Some(rule) = rule {
            let r1 = rule.name("r1").to_number();
            let r2 = rule.name("r2").to_number();
            let r3 = rule.name("r3").to_number();
            let r4 = rule.name("r4").to_number();
            let r5 = rule.name("r5").to_number();
            let r6 = rule.name("r6").to_number();

            if let Some(ch) = rule.name("char") {
                Ok(Rule::Character(ch.as_str().chars().nth(0).ok_or("invalid char")?))
            }
            else if let Some(r1) = r1 {
                Ok(Rule::Sub(
                    Box::new(Rule::Ref(r1)),
                    Box::new(Rule::Ref(r2.ok_or("invalid")?)),
                ))
            }
            else if let Some(r2) = rule.name("r1") {
                Ok(Rule::Sub(
                    Box::new(Rule::Ref(r1.as_str().parse()?)),
                    Box::new(Rule::Ref(rule.name("r2").ok_or("invalid")?.as_str().parse()?))
                ))
            }
            // else if let Some(r1) = rule.name("r3") {
            //     Ok(Rule::Or(
            //         Box::new(Rule::Or(r1.as_str().parse()?)),
            //         Box::new(Rule::Ref(rule.name("r2").ok_or("invalid")?.as_str().parse()?))
            //     ))
            // }
            // else {
            //     Err("Malformed".into())
            // }

            Err("Malformed".into())
        }
        else {
            Err("Malformed".into())
        }
    }
}


fn parse(input: &str) -> (Vec<Rule>, Vec<char>) {
    let mut rules = vec![];

    input.trim().lines().for_each(|line| {
        let l = line.trim();

        Rule::from_str(l);

    })
}