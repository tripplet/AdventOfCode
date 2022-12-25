use std::collections::HashMap;

use ndarray::Array2;
use nom::{
    branch::alt,
    character::complete::{char, line_ending},
    combinator::map,
    multi::{many1, separated_list1},
    IResult,
};

type Vally = Array2<Block>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Wind {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Block {
    Wall,
    Free(Vec<Wind>),
}

// Up, Down, Left, Right
const ALL_DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

impl Block {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            map(char('#'), |_| Block::Wall),
            map(char('.'), |_| Block::Free(vec![])),
            map(char('^'), |_| Block::Free(vec![Wind::North])),
            map(char('v'), |_| Block::Free(vec![Wind::South])),
            map(char('>'), |_| Block::Free(vec![Wind::East])),
            map(char('<'), |_| Block::Free(vec![Wind::West])),
        ))(input)
    }
}

pub fn parse_input(input: &str) -> Vally {
    let (_, vecvec) = separated_list1(line_ending, many1(Block::parse))(input).unwrap();

    Array2::from_shape_vec(
        (vecvec.len(), vecvec[0].len()),
        vecvec.iter().flatten().cloned().collect(),
    )
    .unwrap()
}

fn get_allowed_pos(vally: &Array2<Block>, x: usize, y: usize, dx: isize, dy: isize) -> Option<(usize, usize)> {
    let x = x as isize + dx;
    let y = y as isize + dy;

    if x < 0 || y < 0 {
        return None;
    }

    if let Some(new_position) = vally.get((y as usize, x as usize)) {
        match new_position {
            Block::Free(winds) if winds.is_empty() => Some((y as usize, x as usize)),
            _ => None,
        }
    }
    else {
        None
    }
}

fn do_wind_step(vally: &mut Array2<Block>) {
    let mut new_vally = vally.clone();
    new_vally.iter_mut().for_each(|block| match block {
        Block::Wall => (),
        Block::Free(winds) => winds.clear(),
    });

    let nrows = new_vally.nrows();
    let ncols = new_vally.ncols();

    for (y, row) in vally.rows().into_iter().enumerate() {
        for (x, block) in row.iter().enumerate() {
            match block {
                Block::Wall => (),
                Block::Free(winds) => {
                    for wind in winds {
                        match wind {
                            Wind::North => {
                                if y > 1 {
                                    match &mut new_vally[[y - 1, x]] {
                                        Block::Wall => (),
                                        Block::Free(winds) => winds.push(Wind::North),
                                    }
                                }
                                else if y == 1 {
                                    match &mut new_vally[[nrows - 2, x]] {
                                        Block::Wall => (),
                                        Block::Free(winds) => winds.push(Wind::North),
                                    }
                                }
                            }
                            Wind::South => {
                                if y < nrows - 2 {
                                    match &mut new_vally[[y + 1, x]] {
                                        Block::Wall => (),
                                        Block::Free(winds) => winds.push(Wind::South),
                                    }
                                }
                                else if y == nrows - 2 {
                                    match &mut new_vally[[1, x]] {
                                        Block::Wall => (),
                                        Block::Free(winds) => winds.push(Wind::South),
                                    }
                                }
                            }
                            Wind::East => {
                                if x < vally.ncols() - 2 {
                                    match &mut new_vally[[y, x + 1]] {
                                        Block::Wall => (),
                                        Block::Free(winds) => winds.push(Wind::East),
                                    }
                                }
                                else if x == ncols - 2 {
                                    match &mut new_vally[[y, 1]] {
                                        Block::Wall => (),
                                        Block::Free(winds) => winds.push(Wind::East),
                                    }
                                }
                            }
                            Wind::West => {
                                if x > 1 {
                                    match &mut new_vally[[y, x - 1]] {
                                        Block::Wall => (),
                                        Block::Free(winds) => winds.push(Wind::West),
                                    }
                                } else if x == 1 {
                                    match &mut new_vally[[y, ncols - 2]] {
                                        Block::Wall => (),
                                        Block::Free(winds) => winds.push(Wind::West),
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    *vally = new_vally;
}

fn display_vally(vally: &Array2<Block>, positions: &HashMap<(usize, usize), usize>) {
    for (y, row) in vally.rows().into_iter().enumerate() {
        for (x, block) in row.iter().enumerate() {

            if let Some(count) = positions.get(&(y, x)) {
                print!(" E ");
                continue;
            }

            match block {
                Block::Wall => print!(" # "),
                Block::Free(winds) => {
                    if winds.is_empty() {

                         print!(" . ");
                    } else {
                        if winds.len() == 1 {
                            match winds[0] {
                                Wind::North => print!(" ^ "),
                                Wind::South => print!(" v "),
                                Wind::East => print!(" > "),
                                Wind::West => print!(" < "),
                            }
                        } else if winds.len() <= 9{
                            print!(" {} ", winds.len());
                        }
                        else {
                            print!(" X ");
                        }
                    }
                }
            }
        }
        println!();
    }
}




pub fn find_path(mut vally: &mut Vally, start: (usize, usize), finish: (usize, usize)) -> usize {
    let mut positions: HashMap<(usize, usize), usize> = HashMap::new();
    positions.insert(start, 0);

    loop {
        do_wind_step(&mut vally);

        let mut new_positions = HashMap::new();
        for ((y, x), len) in positions {

            let next_step = len + 1;

            // Check where we can go
            for (new_y, new_x) in ALL_DIRECTIONS.iter().filter_map(|&(dy, dx)| get_allowed_pos(&vally, x, y, dx, dy)) {

                if (new_y, new_x) == finish {
                    return next_step;
                }

                if let Some(len) = new_positions.get_mut(&(new_y, new_x)) {
                    if *len > next_step {
                        *len = next_step;
                    }
                }
                else {
                    new_positions.insert((new_y, new_x), next_step);
                }
            }

            // Add wait option if possible
            if get_allowed_pos(&vally, x, y, 0, 0).is_some() {
                new_positions.insert((y, x), next_step);
            }
        }

        positions = new_positions;
    }
}

pub fn part1(vally: &Vally) -> usize {
    let mut vally = vally.clone();
    let finish = (vally.nrows() - 1, vally.ncols() - 2);
    let start = (0, 1);

    find_path(&mut vally, start, finish)
}

pub fn part2(vally: &Vally) -> usize {

    let mut vally = vally.clone();
    let finish = (vally.nrows() - 1, vally.ncols() - 2);
    let start = (0, 1);
    let mut total_steps = 0;

    // Go from start to finish
    total_steps += find_path(&mut vally, start, finish);

    // Go back to the start
    total_steps += find_path(&mut vally, finish, start);

    // And back to the finish
    total_steps += find_path(&mut vally, start, finish);

    total_steps
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/2022/day24_example.txt");
    const INPUT: &str = include_str!("../input/2022/day24.txt");

    #[test]
    fn example_part1() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part1(&input), 18);
    }

    #[test]
    fn example1_part2() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part2(&input), 54);
    }

    #[test]
    fn input_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), 240);
    }

    #[test]
    fn input_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), 717);
    }
}
