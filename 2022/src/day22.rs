use std::{collections::HashMap, ops::Add};

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
    block_size: Number,
    rows: Number,
    ncol: Number,
    panel: HashMap<Coordinate, Content>,
    instructions: Vec<Instruction>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Coordinate {
    x: Number,
    y: Number,
}

#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
#[repr(u8)]
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
        unsafe { ::std::mem::transmute((*self as u8).overflowing_sub(1).0 % 4) }
    }

    fn turn_right(&self) -> Direction {
        unsafe { ::std::mem::transmute((*self as u8 + 1) % 4) }
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
    let mut block_size = i32::MAX;
    let mut panel = HashMap::new();
    let mut rows: Number = 0;
    let mut ncol: Number = 0;

    for y in 0.. {
        let mut it = iterator(
            input,
            alt((
                map(char(' '), |_| Content::Void),
                map(char('#'), |_| Content::Wall),
                map(char('.'), |_| Content::Empty),
            )),
        );

        let mut cur_block_size = 0;

        it.enumerate().for_each(|(x, content)| {
            ncol = ncol.max(x as Number);
            rows = rows.max(y);

            let pos = Coordinate {
                x: x as Number,
                y: y as Number,
            };
            if start.is_none() && content.is_empty() {
                start = Some(pos);
            }

            if content.is_void() {
                if cur_block_size != 0 {
                    block_size = block_size.min(cur_block_size);
                }

                cur_block_size = 0;
            } else {
                cur_block_size += 1;
            }

            panel.insert(pos, content);
        });

        block_size = block_size.min(cur_block_size);

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
            block_size,
            rows,
            ncol,
            panel,
            instructions,
        },
    ))
}

pub fn parse_input(input: &str) -> InputData {
    parse(input).unwrap().1
}

pub fn part1(input: &InputData) -> isize {
    follow_path(input, &skip_void_part1)
}

pub fn part2(input: &InputData) -> isize {

    // Create flat cube faces
    let mut cube_coord = Vec::with_capacity(6);

    for y in (0..input.rows).step_by(input.block_size as usize) {
        for x in (0..input.ncol).step_by(input.block_size as usize) {
            if let Some(content) = input.panel.get(&Coordinate { y, x }) {
                if !content.is_void() {
                    cube_coord.push((y / input.block_size, x / input.block_size, 0));
                    print!("X");
                    continue;
                }
            }
            print!(" ");
        }
        println!();
    }

    dbg!(cube_coord);

    follow_path(input, &skip_void_part1)
}

#[allow(dead_code)]
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

fn follow_path(
    input: &InputData,
    skip_void: &dyn Fn(
        Coordinate,
        Direction,
        &InputData
    ) -> Option<(Coordinate, Direction)>,
) -> isize {
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
                        } else if content.is_wall() {
                            // Hitting a wall
                            break;
                        } else {
                            // Hitting a void -> Skip it
                            if let Some(pos_after_void) =
                                skip_void(new_position, direction, input)
                            {
                                (position, direction) = pos_after_void;
                                path.insert(position, direction);
                                step -= 1;
                                continue;
                            } else {
                                break;
                            }
                        }
                    } else {
                        // Hitting a void -> Skip it
                        if let Some(pos_after_void) =
                            skip_void(new_position, direction, input)
                        {
                            (position, direction) = pos_after_void;
                            path.insert(position, direction);
                            step -= 1;
                            continue;
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        //print_panel(rows, ncol, input, &path);
    }

    //print_panel(rows, ncol, input, &path);

    //dbg!(Coordinate{ x: position.x + 1, y: position.y + 1 });

    ((position.y + 1) * 1000 + (position.x + 1) * 4 + direction as Number) as isize
}

fn skip_void_part1(
    mut position: Coordinate,
    direction: Direction,
    data: &InputData,
) -> Option<(Coordinate, Direction)> {
    loop {
        if let Some(content) = data.panel.get(&position) {
            if content.is_empty() {
                return Some((position, direction));
            } else if content.is_wall() {
                return None;
            }
        }

        if position.x < 0 {
            position.x = data.ncol - 1;
        } else if position.x >= data.ncol {
            position.x = 0;
        } else if position.y < 0 {
            position.y = data.rows - 1;
        } else if position.y >= data.rows {
            position.y = 0;
        } else {
            position = position + direction;
        }
    }
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
        assert_eq!(part2(&input), 5031);
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
