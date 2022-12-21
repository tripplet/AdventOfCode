use std::collections::VecDeque;

type Number = i16;
type ParseResult = VecDeque<Number>;

pub fn parse_input(input: &str) -> ParseResult {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn unscramble(indexed: &mut VecDeque<(usize, Number)>) {
    let len = indexed.len() as Number;

    for idx in 0..len as usize {
        let pos = indexed.iter().position(|&(index, _)| index == idx).unwrap();

        let (_, move_by) = indexed.remove(pos).unwrap();
        let mut new_position = (pos as Number + move_by) % (len - 1) as Number;

        if new_position < 0 {
            new_position += len - 1;
        }

        indexed.insert(new_position as usize, (idx, move_by));
    }
}

fn get_coordinate_positions<'a>(
    mut array: impl ExactSizeIterator<Item = &'a (usize, Number)>,
) -> (usize, usize, usize) {
    // Get length first, as the iterator will be consumed by position()
    let len = array.len();
    let zero_pos = array.position(|&(_, nb)| nb == 0).unwrap();

    (
        (zero_pos + 1000) % len as usize,
        (zero_pos + 2000) % len as usize,
        (zero_pos + 3000) % len as usize,
    )
}

pub fn part1(input: &ParseResult) -> isize {
    let mut indexed: VecDeque<_> = VecDeque::from_iter(input.iter().cloned().enumerate());
    unscramble(&mut indexed);

    let (a, b, c) = get_coordinate_positions(indexed.iter());
    indexed[a].1 as isize + indexed[b].1 as isize + indexed[c].1 as isize
}

pub fn part2(input: &ParseResult) -> isize {
    let mut after_mult = input
        .iter()
        .map(|&nb| ((nb as isize * 811589153) % (input.len() - 1) as isize) as Number)
        .enumerate()
        .collect::<VecDeque<_>>();

    for _ in 0..10 {
        unscramble(&mut after_mult);
    }

    let (a, b, c) = get_coordinate_positions(after_mult.iter());
    (input[after_mult[a].0] as isize + input[after_mult[b].0] as isize + input[after_mult[c].0] as isize) * 811589153
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/2022/day20_example.txt");
    const INPUT: &str = include_str!("../input/2022/day20.txt");

    #[test]
    fn example_part1() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part1(&input), 3);
    }

    #[test]
    fn example_part2() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part2(&input), 1623178306);
    }

    #[test]
    fn input_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), 14526);
    }

    #[test]
    fn input_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), 9738258246847);
    }
}
