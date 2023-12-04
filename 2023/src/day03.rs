use std::collections::hash_map::Iter;
use std::collections::HashMap;

use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

type Number = i16;
type ParseResult = Schematic;

#[derive(Debug, Clone)]
pub struct Schematic {
    normal: HashMap<Point, Value>,
    all_digits: HashMap<Point, Value>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point {
    x: Number,
    y: Number,
}

#[derive(Debug, Clone)]
pub enum Value {
    Number { value: u16, len: u8, start_x: Number },
    Symbol(char),
}

impl Value {
    fn from_char(c: char) -> Self {
        Value::Symbol(c)
    }

    fn from_digits(digits: &[u16], x: Number, len: u8) -> Self {
        Value::Number{
            start_x: x,
            len,
            value: vec_to_number(digits)
        }
    }
}

#[aoc_generator(day3)]
pub fn parse_input(input: &str) -> ParseResult {
    let mut schematic = Schematic {
        normal: HashMap::new(),
        all_digits: HashMap::new(),
    };

    for (y, line) in input.trim().lines().enumerate() {
        let mut chars = line.char_indices().peekable();

        while let Some(mut cur) = chars.next() {
            let number_start = cur.0 as Number;
            let mut current_number = Vec::with_capacity(10);

            while let Some(digit) = cur.1.to_digit(10) {
                current_number.push(digit as u16);

                if chars.peek().is_some() {
                    cur = chars.next().unwrap();
                } else {
                    break;
                }
            }

            let number_len = current_number.len() as u8;

            if number_len != 0 {
                for sx in number_start..number_start + number_len as Number {
                    schematic.all_digits.insert(
                        Point { x: sx, y: y as Number },
                        Value::Number {
                            value: vec_to_number(&current_number),
                            len: number_len,
                            start_x: number_start,
                        },
                    );
                }

                schematic.normal.insert(
                    Point {
                        x: number_start,
                        y: y as Number,
                    },
                    Value::Number {
                        value: vec_to_number(&current_number),
                        len: number_len,
                        start_x: number_start,
                    },
                );
            }

            if cur.1 == '.' {
                continue;
            }

            schematic.normal.insert(
                Point {
                    x: cur.0 as Number,
                    y: y as Number,
                },
                Value::Symbol(cur.1),
            );
        }
    }

    schematic
}

impl Schematic {
    fn is_symbol(&self, x: Number, y: Number) -> Option<char> {
        self.normal.get(&Point { x, y }).and_then(|v| match v {
            Value::Number { .. } => None,
            Value::Symbol(char) => Some(*char),
        })
    }

    fn is_number(&self, x: Number, y: Number) -> Option<(Point, u16)> {
        self.all_digits.get(&Point { x, y }).and_then(|v| match v {
            Value::Number { value, start_x, .. } => Some((Point { x: *start_x, y }, *value)),
            Value::Symbol(_) => None,
        })
    }

    fn iter(&self) -> Iter<'_, Point, Value> {
        self.normal.iter()
    }
}

fn vec_to_number(digits: &[u16]) -> u16 {
    digits
        .iter()
        .rev()
        .enumerate()
        .fold(0u16, |acc, (pos, cur)| acc + cur * 10u16.pow(pos as u32))
}

#[aoc(day3, part1)]
pub fn part1(input: &ParseResult) -> u32 {
    let mut acc = 0;

    'outer: for (k, v) in input.iter() {
        let (&number, &len) = match v {
            Value::Number { value, len, .. } => (value, len),
            _ => {
                continue;
            }
        };

        // Check left + right
        if input.is_symbol(k.x - 1, k.y).is_some() || input.is_symbol(k.x + (len as Number), k.y).is_some() {
            acc += number as u32;
            continue;
        }

        // Check above + below
        for x in (k.x - 1)..=(k.x + (len as Number)) {
            if input.is_symbol(x, k.y - 1).is_some() || input.is_symbol(x, k.y + 1).is_some() {
                acc += number as u32;
                continue 'outer;
            }
        }
    }

    acc
}

const ADJACENT: [(Number, Number); 8] = [(-1, -1), (0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0)];

#[aoc(day3, part2)]
pub fn part2(input: &ParseResult) -> u32 {
    let mut acc = 0u32;

    'outer: for (k, v) in input.iter() {
        match v {
            Value::Symbol(c) if *c == '*' => (),
            _ => {
                continue;
            }
        };

        // Found potential gear
        let mut numbers = Vec::with_capacity(2);

        for p in ADJACENT.iter() {
            if let Some(new_number) = input.is_number(k.x + p.0, k.y + p.1) {
                if numbers.iter().any(|&(pos, _)| new_number.0 == pos) {
                    continue;
                } else if numbers.len() == 2 {
                    // 3rd number adjacent to the '*' skip
                    continue 'outer;
                } else {
                    numbers.push(new_number);
                }
            }
        }

        if numbers.len() == 2 {
            acc += numbers[0].1 as u32 * numbers[1].1 as u32;
        }
    }

    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/2023/day3_example.txt");
    const INPUT: &str = include_str!("../input/2023/day3.txt");

    #[test]
    fn vec_to_number_test() {
        assert_eq!(vec_to_number(&vec![1, 2, 3]), 123);
        assert_eq!(vec_to_number(&vec![5]), 5);
    }

    #[test]
    fn example_part1() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part1(&input), 4361);
    }

    #[test]
    fn example_part2() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part2(&input), 467835);
    }

    #[test]
    fn input_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), 540131);
    }

    #[test]
    #[ignore = "reason"]
    fn input_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), 42);
    }
}
