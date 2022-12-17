use ndarray::{Array, Array2, Ix2};

#[derive(Debug, Clone)]
pub struct HeightMap {
    map: Array2<i8>,
    start: (usize, usize),
    end: (usize, usize),
}

pub fn parse_input(input: &str) -> HeightMap {
    let array = input
        .lines()
        .map(|line| line.trim().chars().map(|c| c as i8 - 'a' as i8).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut array =
        Array2::from_shape_vec((array.len(), array[0].len()), array.iter().flatten().cloned().collect()).unwrap();

    //dbg!(&array);

    let end = array
        .indexed_iter()
        .find(|&(_, &value)| value == -28)
        .map(|entry| entry.0)
        .unwrap();

    let start = array
        .indexed_iter()
        .find(|&(_, &value)| value == -14)
        .map(|entry| entry.0)
        .unwrap();

    array[end] = 25i8;
    array[start] = 0i8;

    HeightMap { map: array, start, end }
}

fn run_dijkstra(input: &HeightMap, allowed_diff: impl Fn(i8) -> bool) -> Array2<u32> {
    let mut distance = Array::from_elem(input.map.shape(), u32::MAX)
        .into_dimensionality::<Ix2>()
        .unwrap();

    //println!("Start: {:?}, End: {:?}", input.start, input.end);
    //println!("Height map: {:#?}", input.map);

    distance[input.start] = 0;

    // Do a dijkstra search
    let mut queue = vec![input.start];
    while let Some((cur_y, cur_x)) = queue.pop() {
        let next_distance = distance[[cur_y, cur_x]] + 1;

        for (dy, dx) in [(-1, 0), (0, -1), (1, 0), (0, 1)].iter() {
            let y = cur_y as isize + dy;
            let x = cur_x as isize + dx;

            // Check bounds
            if y < 0 || x < 0 || y >= input.map.shape()[0] as isize || x >= input.map.shape()[1] as isize {
                continue;
            }

            let y = y as usize;
            let x = x as usize;

            // Check if we can move there
            // Only hight difference of 1 is allowed
            let diff = input.map[[y, x]] - input.map[[cur_y, cur_x]];

            if !allowed_diff(diff) {
                continue;
            }

            // Check if we have a shorter path
            if next_distance < distance[[y, x]] {
                distance[[y, x]] = next_distance;
                queue.push((y, x));
            }
        }
    }

    //println!("Distance map: {:#?}", distance);

    distance
}

pub fn part1(input: &HeightMap) -> usize {
    let distance = run_dijkstra(input, &|diff| diff <= 1);
    distance[input.end] as usize
}

pub fn part2(input: &HeightMap) -> isize {
    let swapped_start_end = &mut input.clone();
    std::mem::swap(&mut swapped_start_end.start, &mut swapped_start_end.end);

    let distances = run_dijkstra(&swapped_start_end, &|diff| diff >= -1);

    input
        .map
        .indexed_iter()
        .filter(|((_, _), &value)| value == 0)
        .map(|entry| distances[entry.0])
        .min()
        .unwrap() as isize
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/2022/day12_example.txt");
    const INPUT: &str = include_str!("../input/2022/day12.txt");

    #[test]
    fn example_part1() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part1(&input), 31);
    }

    #[test]
    fn example_part2() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part2(&input), 29);
    }

    #[test]
    fn input_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), 425);
    }

    #[test]
    fn input_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), 418);
    }
}
