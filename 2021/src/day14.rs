use itertools::Itertools;
use std::collections::HashMap;
use std::error::Error;
use std::str::FromStr;

const INPUT: &str = include_str!("../input/2021/day14.txt");
const EXAMPLE: &str = include_str!("../input/2021/day14_example.txt");

#[derive(Debug, PartialEq, Eq, Clone)]
struct Polymer {
    polymer: Vec<char>,
    rules: HashMap<(char, char), char>,
}

impl FromStr for Polymer {
    type Err = Box<dyn Error>;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let lines = input.trim().replace("\r", "");
        let mut parts = lines.split("\n\n");

        let polymer: Vec<_> = parts.next().ok_or("missing start polymer")?.chars().collect();
        let mut rules: HashMap<(char, char), char> = HashMap::new();

        for rule in parts.next().ok_or("missing insertion rules")?.lines() {
            let mut parts = rule.split(" -> ");
            let mut in_chars = parts.next().ok_or("missing inputs")?.chars();
            let a = in_chars.next().ok_or("missing char1")?;
            let b = in_chars.next().ok_or("missing char2")?;

            rules.insert(
                (a, b),
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

    fn clever_polymerization(&self, iterations: usize) -> HashMap<char, usize> {
        let mut rule_counter: HashMap<(char, char), (usize, usize)> = HashMap::new();

        rule_counter.extend(self.rules.iter().map(|(&key, _)| (key, (0_usize, 0_usize))));

        // Init
        self.polymer.windows(2).for_each(|x| {
            let (ref mut old, ref mut new) = rule_counter.get_mut(&(x[0], x[1])).unwrap();
            *old += 1;
            //*val2 += 1;
        });

        let pairs = rule_counter.keys().cloned().collect::<Vec<_>>();
        //dbg!(&rule_counter);
        //dbg!(&pairs);

        let mut counts: HashMap<char, usize> = self.polymer.iter().cloned().counts();
        //dbg!(&counts);


        for idx in 0..iterations {
            //println!();
            //println!();
            //println!("{} ##############", idx + 1);
            print_counts(&rule_counter);

            for k in &pairs {
                let new_char = self.rules[&k];
                //println!("-----------");
                //println!("Checking {}{} -> {}", k.0, k.1, new_char);
                ////dbg!(&k);
                ////dbg!(new_char);

                let (ref cur_old, ref mut cur_new) = rule_counter.get_mut(&k).unwrap();

                ////dbg!((&cur_old, &cur_new));
                if *cur_old == 0 {
                    //println!("skipping");
                    continue;
                }

                let x = *cur_old;

                *(counts.entry(new_char).or_default()) += x;

                let (ref mut left_old, ref mut left_new) = rule_counter.get_mut(&(k.0, new_char)).unwrap();
                *left_new += x;

                let (ref mut right_old, ref mut right_new) = rule_counter.get_mut(&(new_char, k.1)).unwrap();
                *right_new += x;

                print_counts(&rule_counter);
                //dbg!(&counts);
            }

            for k in &pairs {
                let (ref mut old, ref mut new) = rule_counter.get_mut(&k).unwrap();
                *old = *new;
                *new = 0;
            }
        }
        counts
    }

}


fn print_counts(counter: &HashMap<(char, char), (usize, usize)>) {
    //println!("++++");
    for (k, v) in counter {
        //if v.0 > 0 || v.1 > 0 {
            //println!("{}{}: old:{}, new:{}", &k.0, &k.1, v.0, v.1);
        //}
    }
    //println!("++++");
}

pub fn main() {
    let polymer = INPUT.parse::<Polymer>().unwrap();

    println!("Part1: {}", part1(&polymer));
    println!("Part2:\n{}", part2(&polymer));
}

fn part1(polymer: &Polymer) -> usize {
    let mut polymer = polymer.clone();

    for idx in 0..10 {
        polymer.full_polymerization();
    }

    let counts = polymer.polymer.iter().counts();

    //dbg!(&counts);

    let least = counts.values().min().unwrap();
    let most = counts.values().max().unwrap();

    most - least
}

fn part2(polymer: &Polymer) -> usize {
    let mut polymer = polymer.clone();

    //println!("{}", polymer.polymer.iter().collect::<String>());
    //for idx in 0..10 {
    let result = polymer.clever_polymerization(40);
    ////println!("idx: {}", polymer.polymer.iter().collect::<String>())
    //}

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
    fn debug1() {
        let polymer = EXAMPLE.parse::<Polymer>().unwrap();

        let r1 = r"NCNBCHB".chars().counts();
        assert_eq!(r1, polymer.clever_polymerization(1));

        let r2 = r"NBCCNBBBCBHCB".chars().counts();
        assert_eq!(r2, polymer.clever_polymerization(2));

        let r3 = r"NBBBCNCCNBBNBNBBCHBHHBCHB".chars().counts();
        assert_eq!(r3, polymer.clever_polymerization(3));

        let r4 = r"NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB".chars().counts();
        assert_eq!(r4, polymer.clever_polymerization(4));
    }

    #[test]
    fn debug2() {
        let p = "NNNNB\n\nNN -> N\nNB -> B\nBB -> N\nNB -> N"
            .parse::<Polymer>()
            .unwrap();

        p.clever_polymerization(2);
    }
}
