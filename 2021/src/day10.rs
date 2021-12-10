use itertools::Itertools;

const INPUT: &str = include_str!("../input/2021/day10.txt");

type Data = Vec<Vec<char>>;

pub fn main() {
    let data = parse_input(INPUT);

    println!("Part1: {}", part1(&data));
    println!("Part2: {}", part2(&data));
}

fn parse_input(input: &str) -> Data {
    input.trim().lines().map(|nb| nb.chars().collect()).collect()
}

fn check_symbol(stack: &mut Vec<char>, symbol: char) -> bool {
    if let Some(x) = stack.pop() {
        x == symbol
    } else {
        false
    }
}

fn symbol_to_stack(stack: &mut Vec<char>, symbol: char) -> bool {
    match symbol {
        '(' => { stack.push(')'); true },
        '[' => { stack.push(']'); true },
        '{' => { stack.push('}'); true },
        '<' => { stack.push('>'); true },
        ')' | ']' | '}' | '>' => check_symbol(stack, symbol),
        _ => panic!()
    }
}

fn part1(data: &Data) -> usize {
    let mut result = 0;

    for syntax in data {
        let mut parse_stack = Vec::with_capacity(data[0].len());

        for &symbol in syntax {
            if !symbol_to_stack(&mut parse_stack, symbol) {
                result += match symbol {
                    ')' => 3,
                    ']' => 57,
                    '}' => 1197,
                    '>' => 25137,
                    _ => panic!(),
                };
                break;
            }
        }
    }
    result
}

fn part2(data: &Data) -> usize {
    let mut scores = Vec::new();

    'syntaxline: for syntax in data {
        let mut parse_stack = Vec::with_capacity(data[0].len());

        for &symbol in syntax {
            if !symbol_to_stack(&mut parse_stack, symbol) {
                continue 'syntaxline;
            }
        }

        if parse_stack.len() != 0 {
            scores.push(parse_stack.iter().rev().fold(0, |total, s| {
                total * 5 + match s {
                    ')' => 1,
                    ']' => 2,
                    '}' => 3,
                    '>' => 4,
                    _ => panic!()
                }
            }));
        }
    }
    *scores.iter().sorted().nth(scores.len() / 2).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/2021/day10_example.txt");

    #[test]
    fn part1_example() { assert_eq!(26397, part1(&parse_input(EXAMPLE))); }

    #[test]
    fn part2_example() { assert_eq!(288957, part2(&parse_input(EXAMPLE))); }

    #[test]
    fn part1_on_input() { assert_eq!(387363, part1(&parse_input(INPUT))); }

    #[test]
    fn part2_on_input() { assert_eq!(4330777059, part2(&parse_input(INPUT))); }
}
