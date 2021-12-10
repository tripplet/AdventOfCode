use itertools::Itertools;

const DAY: &str = file!();
const INPUT: &str = include_str!("../input/2021/day10.txt");
const EXAMPLE: &str = include_str!("../input/2021/day10_example.txt");

type Data = Vec<Vec<char>>;

pub fn main() {
    let data = parse_input(INPUT);

    println!("{}", DAY);
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


fn part1(data: &Data) -> usize {
    let mut result = 0;

    for syntax in data {
        let mut parse_stack = Vec::with_capacity(data[0].len());

        for (idx, &symbol) in syntax.iter().enumerate() {
            let ok = match symbol {
                '(' => { parse_stack.push(')'); true },
                '[' => { parse_stack.push(']'); true },
                '{' => { parse_stack.push('}'); true },
                '<' => { parse_stack.push('>'); true },
                ')' | ']' | '}' | '>' => check_symbol(&mut parse_stack, symbol),
                _ => panic!()
            };

            if !ok {
                println!("Invalid {} at pos {}", symbol, idx);
                result += match symbol {
                    ')' => 3,
                    ']' => 57,
                    '}' => 1197,
                    '>' => 25137,
                    _ => unreachable!(),
                };
                break
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
            let ok = match symbol {
                '(' => { parse_stack.push(')'); true },
                '[' => { parse_stack.push(']'); true },
                '{' => { parse_stack.push('}'); true },
                '<' => { parse_stack.push('>'); true },
                ')' | ']' | '}' | '>' => check_symbol(&mut parse_stack, symbol),
                _ => panic!()
            };

            if !ok {
                continue 'syntaxline
            }
        }

        if parse_stack.len() != 0 {
            println!("Imcomplete line: ");
            dbg!(&parse_stack);

            let total: usize = parse_stack.iter().rev().fold(0, |total, s| {
                total * 5 + match s {
                    ')' => 1,
                    ']' => 2,
                    '}' => 3,
                    '>' => 4,
                    _ => unreachable!()
                }
            });

            dbg!(total);
            scores.push(total);
        }
    }
    let idx = scores.len() / 2;

    dbg!(&scores);
    dbg!(idx);
    *scores.iter().sorted().nth(idx).unwrap()
}



// fn part2(data: ) -> usize {

// }

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "";

    // #[test]
    // fn part1_example() { assert_eq!(, part1(&parse_input(EXAMPLE).unwrap())); }

    // #[test]
    // fn part2_example() { assert_eq!(, part2(&parse_input(EXAMPLE).unwrap())); }

    // #[test]
    // fn part1_on_input() { assert_eq!(, part1(&parse_input(INPUT).unwrap())); }

    // #[test]
    // fn part2_on_input() { assert_eq!(, part2(&parse_input(INPUT).unwrap())); }
}
