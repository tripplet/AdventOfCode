use ndarray::Array2;
use once_cell::sync::Lazy;
use tinyvec::ArrayVec;

type Number = i32;
type ParseResult = Vec<Direction>;

#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Piece {
    width: usize,
    height: usize,
    data: ArrayVec::<[(u8, u8); 5]>,
}

static PIECES: Lazy<[Piece; 5]> = Lazy::new(|| {
    [
        Piece::new(&[(0, 0), (0, 1), (0, 2), (0, 3)]),         // ----
        Piece::new(&[(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)]), // +
        Piece::new(&[(0, 0), (0, 1), (0, 2), (1, 2), (2, 2)]), // mirrored L
        Piece::new(&[(0, 0), (1, 0), (2, 0), (3, 0)]),         // |
        Piece::new(&[(0, 0), (0, 1), (1, 0), (1, 1)]),         // square
    ]
});

impl Piece {
    #[allow(dead_code)]
    fn new(data: &[(u8, u8)]) -> Self {
        Piece {
            width: (data.iter().map(|&(_, x)| x).max().unwrap() + 1) as usize,
            height: (data.iter().map(|&(y, _)| y).max().unwrap() + 1) as usize,
            data: data.iter().copied().collect(),
        }
    }
}

pub fn parse_input(input: &str) -> ParseResult {
    input
        .trim()
        .chars()
        .map(|c| match c {
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!("Unknown direction: {}", c),
        })
        .collect()
}

pub fn part1(directions: &ParseResult) -> isize {
    let mut level = Array2::from_elem((10000, 7), false);

    let mut piece = Some(&PIECES[0]);
    let mut piece_idx = 0;

    dbg!(&piece);

    for (idx, direction) in directions.iter().cycle().enumerate() {
        if idx > 2022 {
            break;
        }



        match direction {
            Direction::Left => {

            }
            Direction::Right => {

            }
        }
    }

    42
}

pub fn part2(input: &ParseResult) -> isize {
    42
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/2022/day17_example.txt");
    const INPUT: &str = include_str!("../input/2022/day17.txt");

    #[test]
    fn example_part1() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part1(&input), 42);
    }

    #[test]
    fn example_part2() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part2(&input), 42);
    }

    #[test]
    fn input_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), 42);
    }

    #[test]
    fn input_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), 42);
    }
}
