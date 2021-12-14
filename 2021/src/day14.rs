use itertools::Itertools;
use std::collections::HashMap;
use std::str::FromStr;

const INPUT: &str = include_str!("../input/2021/day14.txt");

type RuleInput = (char, char);
type RulesCounter = HashMap<RuleInput, (usize, usize)>;
type Rules = HashMap<RuleInput, char>;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Polymer {
    polymer: Vec<char>,
    rules: Rules,
}

impl FromStr for Polymer {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let lines = input.trim().replace("\r", "");
        let mut parts = lines.split("\n\n");

        let polymer: Vec<_> = parts.next().ok_or("missing start polymer")?.chars().collect();
        let mut rules: Rules = HashMap::new();

        for rule in parts.next().ok_or("missing insertion rules")?.lines() {
            let mut parts = rule.split(" -> ");
            let mut in_chars = parts.next().ok_or("missing inputs")?.chars();
            let in_char1 = in_chars.next().ok_or("missing char1")?;
            let in_char2 = in_chars.next().ok_or("missing char2")?;

            rules.insert(
                (in_char1, in_char2),
                parts
                    .next()
                    .ok_or("missing second part of insertion rule")?
                    .chars()
                    .next()
                    .ok_or("missing char")?,
            );
        }

        Ok(Polymer { polymer, rules })
    }
}

impl Polymer {
    fn full_polymerization(&mut self) {
        let new = self
            .polymer
            .windows(2)
            .map(|chars| self.rules[&(chars[0], chars[1])])
            .collect::<Vec<_>>();

        for (idx, n) in new.iter().enumerate() {
            self.polymer.insert((idx * 2) + 1, *n)
        }
    }

    fn get_rule<'a>(counter: &'a mut RulesCounter, rule: &RuleInput) -> &'a mut (usize, usize) {
        counter
            .get_mut(rule)
            .expect(format!("missing rule for {}{} ->", rule.0, rule.1).as_str())
    }

    fn clever_polymerization(&self, iterations: usize) -> HashMap<char, usize> {
        let mut rule_counter: RulesCounter = HashMap::new();
        rule_counter.extend(self.rules.iter().map(|(&key, _)| (key, (0_usize, 0_usize))));

        // Init
        self.polymer.windows(2).for_each(|x| {
            let (ref mut old, _) = Polymer::get_rule(&mut rule_counter, &(x[0], x[1]));
            *old += 1;
        });

        let pairs = rule_counter.keys().cloned().collect::<Vec<_>>();
        let mut counts: HashMap<char, usize> = self.polymer.iter().cloned().counts();

        for _ in 0..iterations {
            for k in &pairs {
                let new_char = self.rules[&k];

                let (ref cur_old, _) = Polymer::get_rule(&mut rule_counter, &k);
                if *cur_old == 0 {
                    continue;
                }

                let old_copy = *cur_old;
                *(counts.entry(new_char).or_default()) += old_copy;

                let (_, ref mut left_new) = Polymer::get_rule(&mut rule_counter, &(k.0, new_char));
                *left_new += old_copy;

                let (_, ref mut right_new) = Polymer::get_rule(&mut rule_counter, &(new_char, k.1));
                *right_new += old_copy;

                //print_counts(&rule_counter);
            }

            for k in &pairs {
                let (ref mut old, ref mut new) = Polymer::get_rule(&mut rule_counter, &k);
                *old = *new;
                *new = 0;
            }
        }
        counts
    }
}

#[allow(dead_code)]
fn print_counts(counter: RulesCounter) {
    for (k, v) in counter {
        if v.0 > 0 || v.1 > 0 {
            println!("{}{}: old:{}, new:{}", &k.0, &k.1, v.0, v.1);
        }
    }
}

pub fn main() {
    let mut now = std::time::Instant::now();
    let polymer = INPUT.parse::<Polymer>().unwrap();
    println!("Parsing [{}]\n", humantime::format_duration(now.elapsed()));

    now = std::time::Instant::now();
    println!("Part1: {} [{}]", part1(&polymer), humantime::format_duration(now.elapsed()));

    now = std::time::Instant::now();
    println!("Part2: {} [{}]", part2(&polymer), humantime::format_duration(now.elapsed()));
}

fn part1(polymer: &Polymer) -> usize {
    let mut polymer = polymer.clone();

    for _ in 0..10 {
        polymer.full_polymerization();
    }

    let counts = polymer.polymer.iter().counts();
    let least = counts.values().min().unwrap();
    let most = counts.values().max().unwrap();
    most - least
}

fn part2(polymer: &Polymer) -> usize {
    let result = polymer.clever_polymerization(40);

    let least = result.values().min().unwrap();
    let most = result.values().max().unwrap();
    most - least
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/2021/day14_example.txt");

    #[test]
    fn part1_example() {
        assert_eq!(1588, part1(&EXAMPLE.parse().unwrap()));
    }

    #[test]
    fn part1_on_input() {
        assert_eq!(2010, part1(&INPUT.parse().unwrap()));
    }

    #[test]
    fn check_missing_rule() {
        let input = "NNB\n\nNN -> N\nNB -> B";
        let polymer = input.parse::<Polymer>().unwrap();
        let result = std::panic::catch_unwind(|| polymer.clever_polymerization(1));

        assert!(result
            .unwrap_err()
            .downcast_ref::<String>()
            .unwrap()
            .contains("missing rule"));
    }

    #[test]
    fn custom_polymer() {
        let input = "NNB\n\nNN -> N\nNB -> N";
        let polymer = input.parse::<Polymer>().unwrap();
        assert_eq!(HashMap::from([('N', 8), ('B', 1),]), polymer.clever_polymerization(2));
    }

    #[test]
    fn part2_example() {
        let polymer = EXAMPLE.parse::<Polymer>().unwrap();

        let expected = r"NCNBCHB".chars().counts();
        assert_eq!(expected, polymer.clever_polymerization(1));

        let expected = r"NBCCNBBBCBHCB".chars().counts();
        assert_eq!(expected, polymer.clever_polymerization(2));

        let expected = r"NBBBCNCCNBBNBNBBCHBHHBCHB".chars().counts();
        assert_eq!(expected, polymer.clever_polymerization(3));

        let expected = r"NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB".chars().counts();
        assert_eq!(expected, polymer.clever_polymerization(4));
    }

    #[test]
    fn part2_on_input() {
        assert_eq!(2437698971143, part2(&INPUT.parse().unwrap()));
    }
}
