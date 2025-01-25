use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use glam::{ivec2, IVec2};

type ParseResult = Warehouse;

#[derive(Debug, Clone)]
pub struct Warehouse {
    grid: HashMap<IVec2, Tile>,
    dimensions: IVec2,
    moves: Vec<Move>,
    start: IVec2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Move {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Empty,
    SmallBox,
    BigBox,
    Wall,
    Robot,
}

impl Tile {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '.' => Some(Self::Empty),
            '#' => Some(Self::Wall),
            'O' => Some(Self::SmallBox),
            '@' => Some(Self::Robot),
            '[' => Some(Self::BigBox),
            ']' => None,
            _ => panic!("Unexpected char '{c}'"),
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            Self::Empty => ".",
            Self::SmallBox => "O",
            Self::Wall => "#",
            Self::Robot => "@",
            Self::BigBox => "[]",
        }
    }
}

impl Move {
    pub fn from_char(c: char) -> Self {
        match c {
            '<' => Self::West,
            '^' => Self::North,
            '>' => Self::East,
            'v' => Self::South,
            _ => panic!("Unexpected char '{c}'"),
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            Self::North => '^',
            Self::South => 'v',
            Self::East => '>',
            Self::West => '<',
        }
    }

    #[inline(always)]
    pub fn to_ivec2(&self) -> IVec2 {
        match self {
            Self::North => IVec2::NEG_Y,
            Self::South => IVec2::Y,
            Self::East => IVec2::X,
            Self::West => IVec2::NEG_X,
        }
    }
}

impl Warehouse {
    fn get_tile(&self, pos: IVec2) -> Option<(Tile, IVec2)> {
        let tile = self.grid.get(&pos);

        if tile.is_none() && matches!(self.grid.get(&(pos - IVec2::X)), Some(&Tile::BigBox)) {
            // Check if this is the right part of a big box
            return Some((Tile::BigBox, pos - IVec2::X));
        }

        tile.map(|t| (*t, pos))
    }

    fn move_robot(&mut self, pos: IVec2, dir: Move) -> bool {
        let new_pos = pos + dir.to_ivec2();

        match self.get_tile(new_pos) {
            None => false,
            Some((Tile::Wall, _)) => false,
            Some((Tile::Empty, _)) => true,
            Some((Tile::SmallBox, new_pos_for_box)) | Some((Tile::BigBox, new_pos_for_box)) => {
                self.try_move_box(new_pos_for_box, dir)
            }
            Some((Tile::Robot, _)) => unreachable!(),
        }
    }

    /// Try to move a box at `pos` in direction `dir`
    fn try_move_box(&mut self, box_pos: IVec2, dir: Move) -> bool {
        if matches!(self.get_tile(box_pos), Some((Tile::BigBox, _))) {
            return self.try_move_big_box(box_pos, dir, false);
        }

        let new_pos_for_box = box_pos + dir.to_ivec2();

        match self.get_tile(new_pos_for_box) {
            None | Some((Tile::Wall, _))=> false,
            Some((Tile::Empty, _)) => {
                // Just move the box
                self.grid.insert(new_pos_for_box, Tile::SmallBox);
                self.grid.insert(box_pos, Tile::Empty);
                true
            }
            Some((Tile::SmallBox, _)) => {
                // Check if this box can be moved recursively
                if self.try_move_box(new_pos_for_box, dir) {
                    self.grid.insert(new_pos_for_box, Tile::SmallBox);
                    self.grid.insert(box_pos, Tile::Empty);
                    true
                } else {
                    false
                }
            }
            Some((Tile::BigBox, _)) => self.try_move_big_box(box_pos, dir, false),
            Some((Tile::Robot, _)) => unreachable!(),
        }
    }

    /// Try to move a big box at `pos` in direction `dir`
    fn try_move_big_box(&mut self, box_pos: IVec2, dir: Move, check_only: bool) -> bool {
        let new_pos_for_box = box_pos + dir.to_ivec2();

        let tile_at_new_pos = self.get_tile(new_pos_for_box);

        match dir {
            Move::North | Move::South => {
                // Check if both spaces are empty or contain a big box wich can be moved
                let mut move_possible = true;
                let mut positions = vec![self.get_tile(new_pos_for_box), self.get_tile(new_pos_for_box + IVec2::X)];

                match (positions[0], positions[1]) {
                    (Some((Tile::BigBox, pos_left)), Some((Tile::BigBox, pos_right))) => {
                        if pos_left == pos_right {
                            positions = vec![positions[0].clone()];
                        }
                    }
                    _ => {}
                }

                for pos in &positions {
                    match pos {
                        None | Some((Tile::Wall, _)) => move_possible = false,
                        Some((Tile::Empty, _)) => {},
                        Some((Tile::BigBox, pos)) => {
                            if !self.try_move_big_box(*pos, dir, true) {
                                move_possible = false;
                            }
                        }
                        Some((Tile::Robot, _)) | Some((Tile::SmallBox, _)) => unreachable!(),
                    }
                }

                if !move_possible {
                    return false;
                }

                if !check_only {
                    for pos in positions {
                        match pos {
                            Some((Tile::Empty, _)) => {
                                self.grid.insert(new_pos_for_box, Tile::BigBox);
                                self.grid.insert(box_pos, Tile::Empty);
                            },
                            Some((Tile::BigBox, origin_pos)) => {
                                self.try_move_big_box(origin_pos, dir, false);
                                self.grid.insert(new_pos_for_box, Tile::BigBox);
                                self.grid.insert(box_pos, Tile::Empty);
                            }
                            Some((Tile::Robot, _)) | Some((Tile::SmallBox, _)) | None | Some((Tile::Wall, _)) => unreachable!(),
                        }
                    }
                }
                true
            }
            Move::East => {
                match tile_at_new_pos {
                    None | Some((Tile::Wall, _))=> false,
                    Some((Tile::Empty, _)) => {
                        if !check_only {
                            self.grid.insert(new_pos_for_box, Tile::BigBox);
                            self.grid.remove(&box_pos);
                            self.grid.insert(box_pos - IVec2::X, Tile::Empty);
                        }
                        true
                    },
                    Some((Tile::BigBox, _)) => {
                        if self.try_move_box(new_pos_for_box + IVec2::X, dir) {
                            if !check_only {
                                self.grid.insert(new_pos_for_box, Tile::BigBox);
                                self.grid.remove(&box_pos);
                                self.grid.insert(box_pos - IVec2::X, Tile::Empty);
                            }
                            true
                        } else {
                            false
                        }
                    },
                    Some((Tile::Robot, _)) | Some((Tile::SmallBox, _)) => unreachable!(),
                }
            }
            Move::West => {
                match tile_at_new_pos {
                    None | Some((Tile::Wall, _))=> false,
                    Some((Tile::Empty, _)) => {
                        if !check_only {
                            self.grid.insert(new_pos_for_box, Tile::BigBox);
                            self.grid.remove(&box_pos);
                            self.grid.insert(box_pos + IVec2::X, Tile::Empty);
                        }
                        true
                    }
                    Some((Tile::BigBox, origin_pos)) => {
                        if self.try_move_box(origin_pos, dir) {
                            if !check_only {
                                self.grid.insert(new_pos_for_box, Tile::BigBox);
                                self.grid.remove(&box_pos);
                                self.grid.insert(box_pos + IVec2::X, Tile::Empty);
                            }
                            true
                        } else {
                            false
                        }
                    },
                    Some((Tile::Robot, _)) | Some((Tile::SmallBox, _)) => unreachable!(),
                }
            }
        }
    }

    #[allow(unused)]
    pub fn print(&self) {
        for y in 0..self.dimensions.y {
            for x in 0..self.dimensions.x {
                let pos = ivec2(x as i32, y as i32);

                if pos == self.start {
                    print!("@");
                } else {
                    if let Some((tile, pos_origin)) = self.get_tile(pos) {
                        if pos == pos_origin {
                            print!("{}", tile.to_str());
                        }
                    }
                }
            }
            println!();
        }
    }
}

#[aoc_generator(day15, part1)]
pub fn parse_input_part1(input: &str) -> ParseResult {
    let mut start = None;

    let input = input.trim().replace('\r', "");
    let input = input.split_once("\n\n").unwrap();

    let moves = input
        .1
        .replace('\n', "")
        .chars()
        .map(Move::from_char)
        .collect::<Vec<_>>();

    let mut grid = HashMap::new();

    input.0.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            if let Some(tile) = Tile::from_char(c) {
                let pos = ivec2(x as i32, y as i32);
                if tile == Tile::Robot {
                    start = Some(pos);
                    grid.insert(pos, Tile::Empty);
                } else  {
                    grid.insert(pos, tile);
                }
            }
        });
    });

    assert!(start.is_some());

    let dimensions = ivec2(input.0.lines().next().unwrap().len() as i32, input.0.lines().count() as i32);

    Warehouse {
        grid,
        dimensions,
        start: start.unwrap(),
        moves,
    }
}

#[aoc_generator(day15, part2)]
pub fn parse_input_part2(input: &str) -> ParseResult {
    let mut input = parse_input_part1(input).clone();
    let mut new_grid = HashMap::new();

    input.grid.into_iter().for_each(|(pos, tile)| {
        match tile {
            Tile::Empty => {
                new_grid.insert(ivec2(pos.x * 2, pos.y), tile);
                new_grid.insert(ivec2(pos.x * 2 + 1, pos.y), tile);
            },
            Tile::SmallBox => {
                new_grid.insert(ivec2(pos.x * 2, pos.y), Tile::BigBox);
            },
            Tile::Wall => {
                new_grid.insert(ivec2(pos.x * 2, pos.y), tile);
                new_grid.insert(ivec2(pos.x * 2 + 1, pos.y), tile);
            },
            Tile::Robot => {
                new_grid.insert(ivec2(pos.x * 2, pos.y), tile);
                new_grid.insert(ivec2(pos.x * 2 + 1, pos.y), Tile::Empty);
            },
            Tile::BigBox => unreachable!(),
        }
    });

    input.grid = new_grid;
    input.dimensions.x *= 2;
    input.start.x *= 2;

    input
}

#[aoc(day15, part1)]
pub fn part1(input: &ParseResult) -> i32 {
    let mut warehouse = input.clone();

    // Move out the moves to avoid partial borrow
    let moves = warehouse.moves.clone();
    warehouse.moves = vec![];
    warehouse.print();
    println!();

    for dir in moves {
        println!("Moving {:?}", dir);

        if warehouse.move_robot(warehouse.start, dir) {
            warehouse.start += dir.to_ivec2();
        }

        warehouse.print();
        println!();
        println!();
    }

    // Calculate the GPS of the boxes
    warehouse
        .grid
        .iter()
        .filter(|(_, &tile)| tile == Tile::SmallBox)
        .map(|(pos, _)| 100 * pos.y + pos.x)
        .sum()
}

#[aoc(day15, part2)]
pub fn part2(input: &ParseResult) -> i32 {
    part1(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_BIG: &str = include_str!("../input/2024/day15_example1.txt");
    const EXAMPLE_SMALL: &str = include_str!("../input/2024/day15_example2.txt");
    const EXAMPLE_SMALL_PART2: &str = include_str!("../input/2024/day15_example3.txt");
    const INPUT: &str = include_str!("../input/2024/day15.txt");

    #[test]
    fn example_big_part1() {
        let input = parse_input_part1(EXAMPLE_BIG);
        assert_eq!(part1(&input), 10092);
    }

    #[test]
    fn example_small_part1() {
        let input = parse_input_part1(EXAMPLE_SMALL);
        assert_eq!(part1(&input), 2028);
    }

    #[test]
    fn input_part1() {
        let input = parse_input_part1(INPUT);
        assert_eq!(part1(&input), 1437174);
    }

    #[test]
    fn example_small_part2() {
        let input = parse_input_part2(EXAMPLE_SMALL_PART2);
        input.print();
        assert_eq!(part2(&input), 618);
    }

    #[test]
    fn example_big_part2() {
        let input = parse_input_part2(EXAMPLE_BIG);
        assert_eq!(part2(&input), 2028);
    }

    //#[test]
    fn input_part2() {
        let input = parse_input_part2(INPUT);
        assert_eq!(part2(&input), todo!());
    }
}
