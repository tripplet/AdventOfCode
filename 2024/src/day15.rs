use aoc_runner_derive::{aoc, aoc_generator};
use glam::{ivec2, IVec2};
use ndarray::Array2;

type Number = i32;
type ParseResult = Warehouse;

#[derive(Debug, Clone)]
pub struct Warehouse {
    grid: Array2<Tile>,
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
    Box,
    Wall,
    Robot,
}

impl Tile {
    pub fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '#' => Self::Wall,
            'O' => Self::Box,
            '@' => Self::Robot,
            _ => panic!("Unexpected char '{c}'"),
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            Self::Empty => '.',
            Self::Box => 'O',
            Self::Wall => '#',
            Self::Robot => '@',
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

    pub fn to_ivec2(&self) -> IVec2 {
        match self {
            Self::North => IVec2::NEG_Y,
            Self::South => IVec2::Y,
            Self::East => IVec2::X,
            Self::West => IVec2::NEG_X,
        }
    }
}

#[inline(always)]
fn index(pos: IVec2) -> [usize; 2] {
    [pos.y as usize, pos.x as usize]
}

impl Warehouse {
    #[inline(always)]
    pub fn get_tile(&self, pos: IVec2) -> Option<Tile> {
        self.grid.get(index(pos)).copied()
    }

    fn move_robot(&mut self, pos: IVec2, dir: Move) -> bool {
        let new_pos = pos + dir.to_ivec2();

        match self.get_tile(new_pos) {
            None => false,
            Some(Tile::Wall) => false,
            Some(Tile::Empty) => {
                true
            }
            Some(Tile::Box) => {
                self.try_move_box(new_pos, dir)
            }
            Some(Tile::Robot) => unreachable!(),
        }
    }

    fn try_move_box(&mut self, pos: IVec2, dir: Move) -> bool {
        let new_pos_for_box = pos + dir.to_ivec2();

        match self.get_tile(new_pos_for_box) {
            None => false,
            Some(Tile::Wall) => false,
            Some(Tile::Empty) => {
                // Just move the box
                self.grid[index(new_pos_for_box)] = Tile::Box;
                self.grid[index(pos)] = Tile::Empty;
                true
            }
            Some(Tile::Box) => {
                // Check if this box can be moved recursively
                if self.try_move_box(new_pos_for_box, dir) {
                    self.grid[index(new_pos_for_box)] = Tile::Box;
                    self.grid[index(pos)] = Tile::Empty;
                    true
                } else {
                    false
                }
            }
            Some(Tile::Robot) => unreachable!(),
        }
    }

    #[allow(unused)]
    pub fn print(&self) {
        for y in 0..self.grid.ncols() {
            for x in 0..self.grid.nrows() {
                if ivec2(x as i32, y as i32) == self.start {
                    print!("@");
                } else {
                    print!("{}", self.grid[index(ivec2(x as i32, y as i32))].to_char());
                }
            }
            println!();
        }
    }
}

#[aoc_generator(day15)]
pub fn parse_input(input: &str) -> ParseResult {
    let mut start = None;

    let input = input.trim().replace('\r', "");
    let input = input.split_once("\n\n").unwrap();

    let array: Vec<Vec<Tile>> = input.0
        .trim()
        .lines()
        .map(|line| line.chars().map(Tile::from_char).collect())
        .collect();

    let mut grid = Array2::from_shape_vec((array.len(), array[0].len()), array.into_iter().flatten().collect()).unwrap();

    for ((y, x), tile) in grid.indexed_iter_mut() {
        if *tile == Tile::Robot {
            start = Some(ivec2(x as i32, y as i32));
            *tile = Tile::Empty;
            break;
        }
    }

    assert!(start.is_some());

    let moves = input.1.replace('\n', "").chars().map(Move::from_char).collect::<Vec<_>>();

    Warehouse {
        grid,
        start: start.unwrap(),
        moves,
    }
}

#[aoc(day15, part1)]
pub fn part1(input: &ParseResult) -> usize {
    let mut warehouse = input.clone();
    let moves = warehouse.moves.clone();

    for dir in moves {
        if warehouse.move_robot(warehouse.start, dir) {
            warehouse.start += dir.to_ivec2();
        }
    }

    // Calculate the GPS of the boxes
    warehouse.grid.indexed_iter().filter(|(_, &tile)| tile == Tile::Box).map(|((y, x), _)| 100 * y + x).sum()
}

#[aoc(day15, part2)]
pub fn part2(input: &ParseResult) -> isize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_BIG: &str = include_str!("../input/2024/day15_example1.txt");
    const EXAMPLE_SMALL: &str = include_str!("../input/2024/day15_example2.txt");
    const INPUT: &str = include_str!("../input/2024/day15.txt");

    #[test]
    fn example_big_part1() {
        let input = parse_input(EXAMPLE_BIG);
        assert_eq!(part1(&input), 10092);
    }

    #[test]
    fn example_small_part1() {
        let input = parse_input(EXAMPLE_SMALL);
        assert_eq!(part1(&input), 2028);
    }

    #[test]
    fn input_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), 1437174);
    }

    //#[test]
    fn example_1_part2() {
        let input = parse_input(EXAMPLE_BIG);
        assert_eq!(part2(&input), todo!());
    }

    //#[test]
    fn input_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), todo!());
    }
}
