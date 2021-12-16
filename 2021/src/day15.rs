
use ndarray::{s, Array, Array2, Ix2};

const INPUT: &str = include_str!("../input/2021/day15.txt");

type Data = Array2<u8>;

pub fn main() {
    let data = parse_input(INPUT);

    println!("Part1: {}", part1(&data));
    println!("Part2: {}", part2(&data));
}

fn parse_input(input: &str) -> Data {
    let input_data: Vec<Vec<u8>> = input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10).and_then(|d| Some(d as u8)))
                .collect()
        })
        .collect();

    Array2::from_shape_vec(
        (input_data.len(), input_data[0].len()),
        input_data.iter().flatten().cloned().collect(),
    )
    .unwrap()
}

fn replicate_matrix(repeat: &[usize], data: &Array2<u8>) -> Array2<u8> {
    let mut new_data = Array::from_elem([data.shape()[0] * repeat[0], data.shape()[1] * repeat[1]], 0)
        .into_dimensionality::<Ix2>()
        .unwrap();

    let ly = data.shape()[0];
    let lx = data.shape()[1];

    for y in 0..repeat[0] {
        for x in 0..repeat[1] {
            let mut copied_matrix = data + x as u8 + y as u8;

            for elem in copied_matrix.iter_mut() {
                if *elem > 9 {
                    *elem = *elem - 9;
                }
            }

            new_data
                .slice_mut(s![
                    ly * y as usize..ly * (y + 1) as usize,
                    lx * x as usize..lx * (x + 1) as usize
                ])
                .assign(&copied_matrix);
        }
    }
    new_data
}

fn get_adjescent(shape: &[usize], point: (usize, usize)) -> Box<impl Iterator<Item = (usize, usize)>> {
    let y = point.0;
    let x = point.1;

    let adjescent = [
        if y as isize + 1 < shape[0] as isize { Some((y + 1, x))} else { None },
        if x as isize + 1 < shape[1] as isize { Some((y, x + 1)) } else { None },
        if y as isize - 1 >= 0 { Some((y - 1, x)) } else { None },
        if x as isize - 1 >= 0 { Some((y, x - 1)) } else { None },
    ];

    Box::new(adjescent.into_iter().filter_map(std::convert::identity))
}

fn part2(data: &Data) -> usize {
    part1(&replicate_matrix(&[5, 5], data))
}

fn part1(data: &Data) -> usize {
    let mut costs = Array::from_elem(data.shape(), u32::MAX)
        .into_dimensionality::<Ix2>()
        .unwrap();

    costs[[0, 0]] = 0;
    let mut queue = vec![(0, 0)];
    let bottom_right = (data.shape()[0] - 1, data.shape()[1] - 1);

    // Run Dijkstra algorithm
    loop {
        queue.sort_unstable_by(|&a, &b| costs[a].partial_cmp(&costs[b]).unwrap());

        let current = if queue.len() > 0 {
            queue.remove(0)
        } else {
            break;
        };

        // Stop when reaching bottom right
        if current == bottom_right {
            return costs[bottom_right] as usize
        }

        let cur_v = costs[current];

        for adj in get_adjescent(data.shape(), current) {
            let adj_v = data[adj];
            let costs = &mut costs[adj];

            if (cur_v + adj_v as u32) < *costs {
                *costs = cur_v + adj_v as u32;
                queue.push(adj);
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/2021/day15_example.txt");

    #[test]
    fn part1_example() { assert_eq!(40, part1(&parse_input(EXAMPLE))); }

    #[test]
    fn part2_example() { assert_eq!(315, part2(&parse_input(EXAMPLE))); }

    #[test]
    fn part1_on_input() { assert_eq!(769, part1(&parse_input(INPUT))); }

    //#[cfg(not(debug_assertions))]
    #[test]
    fn part2_on_input() { assert_eq!(2963, part2(&parse_input(INPUT))); }
}
