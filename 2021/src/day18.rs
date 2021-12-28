use std::cell::{Cell, RefCell};
use std::error::Error;
use std::fmt::{self, Debug, Formatter};
use std::rc::Rc;
use std::rc::Weak;
use itertools::Itertools;

#[derive(Copy, Clone, PartialEq)]
enum Symbol {
    Number(u32),
    OpenBracket,
    ClosingBracket,
}

#[derive(Clone, PartialEq)]
struct Sfn {
    symbols: Vec<Symbol>,
}

impl Debug for Symbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Symbol::Number(nb) => write!(f, "{}", nb),
            Symbol::OpenBracket => write!(f, "["),
            Symbol::ClosingBracket => write!(f, "]"),
        }
    }
}

impl Debug for Sfn {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for symbol in &self.symbols {
            write!(f, "{:?} ", symbol);
        }
        Ok(())
    }
}

const INPUT: &str = include_str!("../input/2021/day18.txt");

pub fn main() {
    let mut now = std::time::Instant::now();
    let data = parse_input(INPUT).unwrap();
    println!("Parsing [{}]\n", humantime::format_duration(now.elapsed()));

    now = std::time::Instant::now();
    println!(
        "Part1: {} [{}]",
        part1(&data),
        humantime::format_duration(now.elapsed())
    );

    now = std::time::Instant::now();
    println!(
        "Part2: {} [{}]",
        part2(&data),
        humantime::format_duration(now.elapsed())
    );
}

fn parse_input(input: &str) -> Result<Vec<Sfn>, Box<dyn Error>> {
    input.trim().lines().map(Sfn::from_str).collect()
}

fn part1(target: &[Sfn]) -> usize {
    let mut numbers = target.to_vec();

    while numbers.len() > 1 {
        let seconds = numbers.remove(1);
        numbers[0].add(seconds);
        numbers[0].reduce();
    }

    numbers[0].magnitude()
}

fn part2(target: &[Sfn]) -> usize {
    target.iter().combinations(2).map(|numbers| {
        let mut result = numbers[0].clone();
        result.add(numbers[1].clone());
        result.reduce();
        let mag = result.magnitude();

        let mut result = numbers[1].clone();
        result.add(numbers[0].clone());
        result.reduce();
        let mag2 = result.magnitude();

        std::cmp::max(mag, mag2)
    }).max().unwrap()
}

impl Sfn {
    fn from_str(input: &str) -> Result<Self, Box<dyn Error>> {
        Sfn::from_chars(&mut input.chars().peekable())
    }

    fn from_chars(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<Sfn, Box<dyn Error>> {
        let mut symbols: Vec<Symbol> = vec![];

        loop {
            if let Some(token) = chars.next() {
                if token == ',' {
                    continue;
                } else if token == '[' {
                    symbols.push(Symbol::OpenBracket);
                } else if token == ']' {
                    symbols.push(Symbol::ClosingBracket);
                } else if token.is_digit(10) {
                    if let Some(next_char) = chars.peek() {
                        if next_char.is_digit(10) {
                            symbols.push(Symbol::Number(format!("{}{}", token, chars.next().unwrap()).parse()?));
                            continue;
                        }
                    }
                    symbols.push(Symbol::Number(token.to_digit(10).unwrap() as u32));
                }
            } else {
                break;
            }
        }
        Ok(Sfn { symbols })
    }

    #[allow(dead_code)]
    fn add(&mut self, other: Sfn) {
        self.symbols.insert(0, Symbol::OpenBracket);
        self.symbols.append(&mut other.symbols.clone());
        self.symbols.push(Symbol::ClosingBracket);
    }

    #[allow(dead_code)]
    fn reduce(&mut self) {
        loop {
            if self.explode() {
                continue;
            }

            if self.split() {
                continue;
            }

            break;
        }
    }

    #[allow(dead_code)]
    fn split(&mut self) -> bool {
        for idx in 0..self.symbols.len() {
            if let Symbol::Number(number) = self.symbols[idx] {
                if number > 9 {
                    self.symbols[idx] = Symbol::Number((number as f64 / 2.0).floor() as u32);
                    self.symbols
                        .insert(idx + 1, Symbol::Number((number as f64 / 2.0).ceil() as u32));
                    self.symbols.insert(idx, Symbol::OpenBracket);
                    self.symbols.insert(idx + 3, Symbol::ClosingBracket);
                    return true;
                }
            }
        }
        false
    }

    fn find_next_number_idx(&mut self, start_index: usize, direction: isize) -> Option<&mut Symbol> {
        let mut index: isize = start_index as isize + direction;
        loop {
            if index < 0 || index >= self.symbols.len() as isize {
                return None;
            }

            if let Some(Symbol::Number(_)) = self.symbols.get_mut(index as usize) {
                return self.symbols.get_mut(index as usize);
            }

            // if let Some(Symbol::Number(ref mut nb)) = a {
            //     //return Some(nb);
            // }
            index += direction;
        }
        unreachable!();
    }

    fn explode(&mut self) -> bool {
        let mut level = 0;

        for idx in 0..self.symbols.len() {
            if self.symbols[idx] == Symbol::OpenBracket {
                level += 1;
            } else if self.symbols[idx] == Symbol::ClosingBracket {
                level -= 1;
            } else if level > 4 {
                if let Symbol::Number(left) = self.symbols[idx] {
                    if let Some(Symbol::Number(right)) = self.symbols.get(idx + 1).cloned() {
                        if let Some(Symbol::Number(ref mut left_nb)) = self.find_next_number_idx(idx, -1) {
                            *left_nb += left; // Explode number left
                        }
                        if let Some(Symbol::Number(ref mut right_nb)) = self.find_next_number_idx(idx + 1, 1) {
                            *right_nb += right; // Explode number right
                        }
                        self.symbols[idx] = Symbol::Number(0); // set own value to 0
                        self.symbols.remove(idx + 1);
                        self.symbols.remove(idx + 1);
                        self.symbols.remove(idx - 1);
                        return true;
                    }
                }
            }
        }
        false
    }

    fn magnitude(&self) -> usize {
        let mut magnitudes = self.symbols.clone();

        while magnitudes.len() > 1 {
            for idx in 0..magnitudes.len() {
                if let Symbol::Number(left) = magnitudes[idx] {
                    if let Some(Symbol::Number(right)) = magnitudes.get(idx + 1).cloned() {

                        magnitudes[idx] = Symbol::Number(3*left + 2 * right); // set own value to 0
                        magnitudes.remove(idx + 1);
                        magnitudes.remove(idx + 1);
                        magnitudes.remove(idx - 1);
                        break;
                    }
                }
            }
        }

        match magnitudes[0] {
            Symbol::Number(number) => number as usize,
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/2021/day18_example.txt");

    #[test]
    fn parse() {
        assert_eq!(
            "[ [ [ 1 2 ] 4 ] [ 7 3 ] ] ",
            format!("{:?}", Sfn::from_str("[[[1,2],4],[7,3]]").unwrap())
        );
    }

    #[test]
    fn add() {
        let expected = Sfn::from_str("[[[1,2],4],[7,3]]").unwrap();

        let mut left = Sfn::from_str("[[1,2],4]").unwrap();
        left.add(Sfn::from_str("[7,3]").unwrap());

        assert_eq!(expected, left);
    }

    #[test]
    fn check_input() {
        for (idx, line) in INPUT.trim().lines().enumerate() {
            assert!(Sfn::from_str(line).is_ok(), "Error parsing line {}", idx + 1);
        }
    }

    #[test]
    fn split() {
        let mut check = Sfn::from_str("[13,2]").unwrap();
        assert_eq!(true, check.split());
        assert_eq!(Sfn::from_str("[[6,7],2]").unwrap(), check);

        let mut check = Sfn::from_str("[4,17]").unwrap();
        assert_eq!(true, check.split());
        assert_eq!(Sfn::from_str("[4,[8,9]]").unwrap(), check);

        let mut check = Sfn::from_str("[4,18]").unwrap();
        assert_eq!(true, check.split());
        assert_eq!(Sfn::from_str("[4,[9,9]]").unwrap(), check);

        let mut check = Sfn::from_str("[13,18]").unwrap();
        assert_eq!(true, check.split());
        assert_eq!(Sfn::from_str("[[6,7],18]").unwrap(), check);

        let mut check = Sfn::from_str("[3,2]").unwrap();
        assert_eq!(false, check.split());
        assert_eq!(Sfn::from_str("[3,2]").unwrap(), check);
    }

    #[test]
    fn explode() {
        let mut check = Sfn::from_str("[[6,[5,[4,[3,2]]]],1]").unwrap();
        assert_eq!(true, check.explode());
        assert_eq!(Sfn::from_str("[[6,[5,[7,0]]],3]").unwrap(), check);

        let mut check = Sfn::from_str("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]").unwrap();
        assert_eq!(true, check.explode());
        assert_eq!(Sfn::from_str("[[[[0,7],4],[7,[[8,4],9]]],[1,1]]").unwrap(), check);
    }

    #[test]
    fn example() {
        let mut check = Sfn::from_str("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]").unwrap();
        check.reduce();
        assert_eq!(Sfn::from_str("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").unwrap(), check);
    }

    #[test]
    fn example_add() {
        let mut numbers = parse_input(EXAMPLE).unwrap();

        while numbers.len() > 1 {
            let seconds = numbers.remove(1);
            numbers[0].add(seconds);
            numbers[0].reduce();
        }

        assert_eq!(
            Sfn::from_str("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]").unwrap(),
            numbers[0]
        );
    }

    #[test]
    fn magnitudes() {
        let mut check = Sfn::from_str("[[1,2],[[3,4],5]]").unwrap();
        assert_eq!(143, check.magnitude());

        let mut check = Sfn::from_str("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").unwrap();
        assert_eq!(1384, check.magnitude());

        let mut check = Sfn::from_str("[[[[1,1],[2,2]],[3,3]],[4,4]]").unwrap();
        assert_eq!(445, check.magnitude());

        let mut check = Sfn::from_str("[[[[3,0],[5,3]],[4,4]],[5,5]]").unwrap();
        assert_eq!(791, check.magnitude());

        let mut check = Sfn::from_str("[[[[5,0],[7,4]],[5,5]],[6,6]]").unwrap();
        assert_eq!(1137, check.magnitude());

        let mut check = Sfn::from_str("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]").unwrap();
        assert_eq!(3488, check.magnitude());
    }


    #[test]
    fn example_part2() {
        assert_eq!(3993, part2(&parse_input(EXAMPLE).unwrap()));
    }
}
