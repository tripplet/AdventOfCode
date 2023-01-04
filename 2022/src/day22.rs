use std::{collections::{HashMap, HashSet}, ops::Add};

use nom::{
    branch::alt,
    character::complete::{char, line_ending, u8},
    combinator::{iterator, map},
    multi::many1,
    sequence::tuple,
    IResult,
};

type Number = i32;

#[derive(Debug)]
pub struct InputData {
    start: Coordinate,
    panel: HashMap<Coordinate, Content>,
    instructions: Vec<Instruction>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Coordinate {
    x: Number,
    y: Number,
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Right = 0,
    Down,
    Left,
    Up,
}

#[derive(Debug)]
enum Content {
    Void,
    Empty,
    Wall,
}

#[derive(Debug)]
enum Instruction {
    TurnLeft,
    TurnRight,
    Forward(u8),
}

impl Direction {
    fn turn_left(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

impl Add<Direction> for Coordinate {
    type Output = Coordinate;

    fn add(self, direction: Direction) -> Coordinate {
        match direction {
            Direction::Up => Coordinate {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Down => Coordinate {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Left => Coordinate {
                x: self.x - 1,
                y: self.y,
            },
            Direction::Right => Coordinate {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}

impl Content {
    fn is_void(&self) -> bool { matches!(self, Content::Void) }
    fn is_empty(&self) -> bool { matches!(self, Content::Empty) }
    fn is_wall(&self) -> bool { matches!(self, Content::Wall) }
}

fn parse(mut input: &str) -> IResult<&str, InputData> {
    let mut start = None;
    let mut panel = HashMap::new();

    for y in 0.. {
        let mut it = iterator(
            input,
            alt((
                map(char(' '), |_| Content::Void),
                map(char('#'), |_| Content::Wall),
                map(char('.'), |_| Content::Empty),
            )),
        );

        it.enumerate().for_each(|(x, content)| {
            let pos = Coordinate {
                x: x as Number,
                y: y as Number,
            };
            if start.is_none() && !content.is_void() && !content.is_wall() {
                start = Some(pos);
            }
            panel.insert(pos, content);
        });

        input = (it.finish() as IResult<_, _>)?.0;

        // Is end of map?
        let end = tuple((line_ending, line_ending))(input) as IResult<_, _>;

        if let Ok((res, _)) = end {
            input = res;
            break;
        } else {
            let (res, _) = line_ending(input)?;
            input = res;
        }
    }

    panel.shrink_to_fit();

    let (_, instructions) = many1(alt((
        map(char('L'), |_| Instruction::TurnLeft),
        map(char('R'), |_| Instruction::TurnRight),
        map(u8, Instruction::Forward),
    )))(input)?;

    Ok((
        "",
        InputData {
            start: start.unwrap(),
            panel,
            instructions,
        },
    ))
}

pub fn parse_input(input: &str) -> InputData {
    parse(input).unwrap().1
}

pub fn part1(input: &InputData) -> isize {
    let rows = input.panel.keys().map(|c| c.y).max().unwrap() + 1;
    let ncol = input.panel.keys().map(|c| c.x).max().unwrap() + 1;

    let mut position = input.start;
    let mut direction = Direction::Right;

    let mut path = HashMap::new();

    for instruction in &input.instructions {
        match instruction {
            Instruction::TurnLeft => { direction = direction.turn_left(); }
            Instruction::TurnRight => { direction = direction.turn_right(); }
            Instruction::Forward(n) => {
                let mut step = *n;
                while step > 0 {
                    let new_position = position + direction;

                    let at_postition = input.panel.get(&new_position);
                    if let Some(content) = at_postition {
                        if content.is_empty() {
                            // Move normally to empty space
                            position = new_position;
                            path.insert(position, direction);
                            step -= 1;
                            continue;
                        }
                        else if content.is_wall() {
                            // Hitting a wall
                            break;
                        }
                        else {
                            // Hitting a void -> Skip it
                            if let Some(pos_after_void) = skip_void(new_position, direction, &input.panel, rows, ncol) {
                                position = pos_after_void;
                                path.insert(position, direction);
                                step -= 1;
                                continue;
                            }
                            else {
                                break;
                            }
                        }
                    }
                    else {
                        // Hitting a void -> Skip it
                        if let Some(pos_after_void) = skip_void(new_position, direction, &input.panel, rows, ncol) {
                            position = pos_after_void;
                            path.insert(position, direction);
                            step -= 1;
                            continue;
                        }
                        else {
                            break;
                        }
                    }
                }
            }
        }

        //print_panel(rows, ncol, input, &path);
    }

    //print_panel(rows, ncol, input, &path);

    dbg!(Coordinate{ x: position.x + 1, y: position.y + 1 });

    ((position.y + 1) * 1000 + (position.x + 1) * 4 + direction as Number) as isize
}

fn print_panel(rows: i32, ncol: i32, input: &InputData, path: &HashMap<Coordinate, Direction>) {
    println!();
    println!();

    for y in 0..rows {
        for x in 0..ncol {
            let pos = Coordinate { x, y };
            let content = input.panel.get(&pos).unwrap_or(&Content::Void);

            if let Some(dir) = path.get(&pos) {
                match dir {
                    Direction::Up => print!("^"),
                    Direction::Down => print!("v"),
                    Direction::Left => print!("<"),
                    Direction::Right => print!(">"),
                }
                continue;
            }

            let c = match content {
                Content::Void => ' ',
                Content::Empty => '.',
                Content::Wall => '#',
            };
            print!("{}", c);
        }
        println!();
    }
}

fn skip_void(mut position: Coordinate, direction: Direction, panel: &HashMap<Coordinate, Content>, rows: i32, ncol: i32) -> Option<Coordinate> {
    loop {
        if let Some(content) = panel.get(&position) {
            if content.is_empty() {
                return Some(position);
            }
            else if content.is_wall() {
                return None;
            }
        }

        if position.x < 0 {
            position.x = ncol - 1;
        }
        else if position.x >= ncol {
            position.x = 0;
        }
        else if position.y < 0 {
            position.y = rows - 1;
        }
        else if position.y >= rows {
            position.y = 0;
        }
        else {
            position = position + direction;
        }
    }
}

pub fn part2(input: &InputData) -> isize {
    42
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/2022/day22_example.txt");
    const INPUT: &str = include_str!("../input/2022/day22.txt");

    #[test]
    fn example_part1() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part1(&input), 6032);
    }

    #[test]
    fn example_part2() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part2(&input), 42);
    }

    #[test]
    fn input_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), 80392);
    }

    #[test]
    fn input_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), 42);
    }
}
