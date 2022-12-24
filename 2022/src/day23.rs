use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

type Number = i16;
type ParseResult = HashSet<Position>;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Direction {
    North,
    NorthWest,
    NorthEast,
    South,
    SouthWest,
    SouthEast,
    East,
    West,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Position {
    x: Number,
    y: Number,
}

const ELF_CHECK_DIRECTIONS: [Direction; 4] = [Direction::North, Direction::South, Direction::West, Direction::East];

const ALL_DIRECTIONS: [Direction; 8] = [
    Direction::North,
    Direction::NorthWest,
    Direction::NorthEast,
    Direction::South,
    Direction::SouthWest,
    Direction::SouthEast,
    Direction::East,
    Direction::West,
];

pub fn parse_input(input: &str) -> ParseResult {
    let mut elve_positions: HashSet<Position> = HashSet::new();

    input.trim().lines().enumerate().for_each(|(y, line)| {
        line.trim().chars().enumerate().for_each(|(x, c)| {
            if c == '#' {
                elve_positions.insert(Position {
                    x: x as Number,
                    y: y as Number,
                });
            }
        });
    });

    elve_positions
}

impl Position {
    fn get_proposed_move(
        &self,
        start_direction_index: u8,
        check_position: impl Fn(&Position) -> bool,
    ) -> Option<Position> {
        use Direction::*;

        // First check if we will move at all
        // > each Elf considers the eight positions adjacent to themself.
        // > If no other Elves are in one of those eight positions, the Elf does not do anything
        if !ALL_DIRECTIONS
            .iter()
            .any(|direction| check_position(&self.get_new_position(direction)))
        {
            return None;
        }

        // > Otherwise, the Elf looks in each of four directions in the following order
        // > and proposes moving one step in the first valid direction

        for direction_index_delta in 0..4 {
            let direction_index = (start_direction_index + direction_index_delta) % 4;
            let direction = &ELF_CHECK_DIRECTIONS[direction_index as usize];

            match direction {
                North => {
                    if !check_position(&self.get_new_position(&North))
                        && !check_position(&self.get_new_position(&NorthWest))
                        && !check_position(&self.get_new_position(&NorthEast))
                    {
                        return Some(self.get_new_position(&North));
                    }
                }
                South => {
                    if !check_position(&self.get_new_position(&South))
                        && !check_position(&self.get_new_position(&SouthWest))
                        && !check_position(&self.get_new_position(&SouthEast))
                    {
                        return Some(self.get_new_position(&South));
                    }
                }
                East => {
                    if !check_position(&self.get_new_position(&East))
                        && !check_position(&self.get_new_position(&NorthEast))
                        && !check_position(&self.get_new_position(&SouthEast))
                    {
                        return Some(self.get_new_position(&East));
                    }
                }
                West => {
                    if !check_position(&self.get_new_position(&West))
                        && !check_position(&self.get_new_position(&NorthWest))
                        && !check_position(&self.get_new_position(&SouthWest))
                    {
                        return Some(self.get_new_position(&West));
                    }
                }
                _ => panic!(),
            }
        }

        None
    }

    fn get_new_position(&self, direction: &Direction) -> Position {
        match direction {
            Direction::North     => Position { x: self.x + 0, y: self.y - 1, },
            Direction::NorthWest => Position { x: self.x - 1, y: self.y - 1, },
            Direction::NorthEast => Position { x: self.x + 1, y: self.y - 1, },
            Direction::South     => Position { x: self.x + 0, y: self.y + 1, },
            Direction::SouthWest => Position { x: self.x - 1, y: self.y + 1, },
            Direction::SouthEast => Position { x: self.x + 1, y: self.y + 1, },
            Direction::East      => Position { x: self.x + 1, y: self.y + 0, },
            Direction::West      => Position { x: self.x - 1, y: self.y + 0, },
        }
    }
}

fn run_elf_simulation(elf_positions: &HashSet<Position>, iterations: Option<usize>) -> (usize, HashSet<Position>) {
    let mut elf_positions = elf_positions.clone();
    let mut direction_check_position = 0_u8;
    let max_iterations = iterations.unwrap_or(usize::MAX);

    let mut new_elf_positions: HashMap<Position, (Position, bool)> = HashMap::with_capacity(elf_positions.len());

    for iteration in 1..=max_iterations {
        new_elf_positions.clear();

        for elf in elf_positions.iter() {
            if let Some(new_position) =
                elf.get_proposed_move(direction_check_position, |position| elf_positions.contains(position))
            {
                if let Some(existing) = new_elf_positions.get_mut(&new_position) {
                    existing.1 = true;
                } else {
                    new_elf_positions.insert(new_position, (elf.clone(), false));
                }
            }
        }

        for (new_position, (old_position, is_colliding)) in new_elf_positions.iter() {
            if !is_colliding {
                // Move the elf
                elf_positions.remove(old_position);
                elf_positions.insert(new_position.clone());
            }
        }

        direction_check_position = (direction_check_position + 1) % ELF_CHECK_DIRECTIONS.len() as u8;

        if new_elf_positions.is_empty() {
            return (iteration, elf_positions);
        }
    }

    (max_iterations, elf_positions)
}

pub fn part1(input: &ParseResult) -> isize {
    let (_, elf_positions) = run_elf_simulation(input, Some(10));

    let lowest_y = elf_positions.iter().min_by_key(|position| position.y).unwrap().y;
    let lowest_x = elf_positions.iter().min_by_key(|position| position.x).unwrap().x;
    let highest_x = elf_positions.iter().max_by_key(|position| position.x).unwrap().x;
    let highest_y = elf_positions.iter().max_by_key(|position| position.y).unwrap().y;

    ((highest_y - lowest_y + 1).abs() * (highest_x - lowest_x + 1).abs() - elf_positions.len() as Number) as isize
}

pub fn part2(input: &ParseResult) -> usize {
    run_elf_simulation(input, None).0
}

#[allow(dead_code)]
fn print_elf_map(elve_positions: &HashSet<Position>) {
    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;

    for elf in elve_positions.iter() {
        if elf.x < min_x {
            min_x = elf.x;
        }
        if elf.x > max_x {
            max_x = elf.x;
        }
        if elf.y < min_y {
            min_y = elf.y;
        }
        if elf.y > max_y {
            max_y = elf.y;
        }
    }

    for y in (min_y - 1)..=(max_y + 1) {
        for x in (min_x - 1)..=(max_x + 1) {
            if elve_positions.contains(&Position { x, y }) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/2022/day23_example.txt");
    const INPUT: &str = include_str!("../input/2022/day23.txt");

    #[test]
    fn example_part1() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part1(&input), 110);
    }

    #[test]
    fn example_part2() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part2(&input), 20);
    }

    #[test]
    fn input_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), 3874);
    }

    #[test]
    fn input_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), 948);
    }
}
