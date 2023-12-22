use aoc_runner_derive::{aoc, aoc_generator};
use glam::{ivec2, IVec2};
use itertools::Itertools;
use ndarray::{Array, Array2, Ix2};

type ParseResult = Array2<u8>;

const LEFT1: IVec2 = ivec2(-1, 0);
const LEFT2: IVec2 = ivec2(2, 0);
const LEFT3: IVec2 = ivec2(-3, 0);
const RIGHT1: IVec2 = ivec2(1, 0);
const RIGHT2: IVec2 = ivec2(2, 0);
const RIGHT3: IVec2 = ivec2(3, 0);
const DOWN1: IVec2 = ivec2(0, 1);
const DOWN2: IVec2 = ivec2(0, 2);
const DOWN3: IVec2 = ivec2(0, 3);
const UP1: IVec2 = ivec2(0, -1);
const UP2: IVec2 = ivec2(0, -2);
const UP3: IVec2 = ivec2(0, -3);

const ADJACENT: [IVec2; 12] = [
    UP1, UP2, UP3, DOWN1, DOWN2, DOWN3, LEFT1, LEFT2, LEFT3, RIGHT1, RIGHT2, RIGHT3,
];

#[aoc_generator(day17)]
pub fn parse_input(input: &str) -> ParseResult {
    let array = input
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|value| value.to_digit(10).unwrap() as u8)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    Array2::from_shape_vec((array.len(), array[0].len()), array.iter().flatten().cloned().collect()).unwrap()
}

fn is_valid(pos: IVec2, direction: IVec2, dim: IVec2, past_moves: &Array2<Vec<(u32, IVec2)>>) -> Option<(u32, IVec2)> {
    let new_pos = pos + direction;
    if new_pos.x < 0 || new_pos.y < 0 || new_pos.x >= dim.x || new_pos.y >= dim.y {
        return None;
    }

    if past_moves.is_empty() {
        return Some((u32::MAX, new_pos));
    }

    for prev_move in &past_moves[[pos.y as usize, pos.x as usize]] {
        let dot = prev_move.1.dot(direction);
        if dot == 0 {
            return Some((prev_move.0, new_pos));
        } else if dot > 0 && prev_move.1.abs().max_element() + direction.abs().max_element() <= 3 {
            return Some((prev_move.0, new_pos));
        } else {
            return None;
        }
    }

    return None;
}

fn print_dist(array: &Array2<u32>) {
    let dim = ivec2(array.ncols() as i32, array.nrows() as i32);

    for y in 0..dim.y {
        for x in 0..dim.x {
            let v = array[[y as usize, x as usize]];
            if v == u32::MAX {
                print!("    ")
            } else {
                print!("{:3} ", v)
            }
        }
        println!()
    }
}

fn print_moves(array: &Array2<Option<IVec2>>) {
    let dim = ivec2(array.ncols() as i32, array.nrows() as i32);

    for y in 0..dim.y {
        for x in 0..dim.x {
            if let Some(v) = array[[y as usize, x as usize]] {
                print!("{}", v);
            } else {
                print!("  ");
            }
        }
        println!()
    }
}

#[aoc(day17, part1)]
pub fn part1(input: &ParseResult) -> u32 {
    let dim = ivec2(input.ncols() as i32, input.nrows() as i32);

    let mut dist = Array::from_elem(input.shape(), u32::MAX)
        .into_dimensionality::<Ix2>()
        .unwrap();

    let mut prev = Array::from_elem(input.shape(), None::<IVec2>)
        .into_dimensionality::<Ix2>()
        .unwrap();

    let mut last_moves = Array::from_elem(input.shape(), vec![])
        .into_dimensionality::<Ix2>()
        .unwrap();

    dist[[0, 0]] = 0;

    let mut queue = vec![];

    for y in 0..dim.y {
        for x in 0..dim.x {
            queue.push(ivec2(x, y));
        }
    }

    while !queue.is_empty() {
        let elem_idx = queue
            .iter()
            .position_min_by_key(|q| dist[[q.y as usize, q.x as usize]])
            .unwrap();
        let u = queue.remove(elem_idx);

        for direction in ADJACENT {
            let neighbor_v = if let Some(neighbor) = is_valid(u, direction, dim, &last_moves) {
                if !queue.iter().contains(&neighbor.1) {
                    continue;
                }

                neighbor
            } else {
                continue;
            };

            let u2 = [u.y as usize, u.x as usize];
            let neighbor_v2 = [neighbor_v.1.y as usize, neighbor_v.1.x as usize];

            let normalized_direction = direction.signum();
            let mut alt = neighbor_v.0;

            for idx in 1..=direction.abs().max_element() {
                let dist = normalized_direction * idx;
                alt = alt.saturating_add(input[[(u.y + dist.y) as usize, (u.x + dist.x) as usize]] as u32);
            }

            if alt < dist[neighbor_v2] {
                dist[neighbor_v2] = alt;

                last_moves[neighbor_v2].push((alt, direction));
                prev[neighbor_v2] = Some(u);

                print_dist(&dist);

                println!();
                println!();

                //print_moves(&last_moves);
            }
        }
    }

    print_dist(&dist);

    // for y in 0..dim.y {
    //     for x in 0..dim.x {
    //         if let Some(xx) = prev[[y as usize, x as usize]] {
    //             match (xx.y - y, xx.x -x) {
    //                 (0, 1) => print!("  >  "),
    //                 (0, -1) => print!("  <  "),
    //                 (1, 0) => print!("  v  "),
    //                 (-1, 0) => print!("  ^  "),
    //                 (y, x) => print!("{y},{x} "),
    //             }
    //         }
    //         else {
    //             print!("  ");
    //         }

    //     }
    //     println!();
    // }

    dist[[dist.nrows() - 1, dist.ncols() - 1]]
}

#[aoc(day17, part2)]
pub fn part2(input: &ParseResult) -> isize {
    42
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = include_str!("../input/2023/day17_example1.txt");
    const EXAMPLE2: &str = include_str!("../input/2023/day17_example2.txt");
    const EXAMPLE3: &str = include_str!("../input/2023/day17_example3.txt");
    const INPUT: &str = include_str!("../input/2023/day17.txt");

    #[test]
    fn dir_test() {
        assert_eq!(ivec2(1, 2).dot(ivec2(3, 5)), 13);
        assert_eq!(ivec2(1, 0).dot(ivec2(0, -2)), 0);
    }

    #[test]
    fn example1_part1() {
        let input = parse_input(EXAMPLE1);
        assert_eq!(part1(&input), 102);
    }

    #[test]
    fn example2_part1() {
        let input = parse_input(EXAMPLE2);
        assert_eq!(part1(&input), 9);
    }

    #[test]
    fn example3_part1() {
        let input = parse_input(EXAMPLE3);
        assert_eq!(part1(&input), 9);
    }

    //#[test]
    fn input_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), todo!());
    }

    //#[test]
    fn example_part2() {
        let input = parse_input(EXAMPLE1);
        assert_eq!(part2(&input), todo!());
    }

    //#[test]
    fn input_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), 42);
    }
}
