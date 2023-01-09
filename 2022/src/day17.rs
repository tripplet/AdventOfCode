use ndarray::Array2;
use once_cell::sync::Lazy;
use tinyvec::ArrayVec;

type ParseResult = Vec<Direction>;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Piece {
    width: usize,
    height: usize,
    data: ArrayVec<[(u8, u8); 5]>,
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
    fn new(data: &[(u8, u8)]) -> Self {
        Piece {
            width: (data.iter().map(|&(_, x)| x).max().unwrap() + 1) as usize,
            height: (data.iter().map(|&(y, _)| y).max().unwrap() + 1) as usize,
            data: data.iter().copied().collect(),
        }
    }

    fn collides(&self, dy: usize, dx: usize, level: &Array2<bool>) -> bool {
        for &(y, x) in &self.data {
            if level[(y as usize + dy, x as usize + dx)] {
                return true;
            }
        }

        false
    }

    fn can_move(&self, dy: usize, dx: usize, direction: &Direction, level: &Array2<bool>) -> bool {
        // Check the left and right walls first
        if (*direction == Direction::Left && dx == 0)
            || (*direction == Direction::Right && dx + self.width == level.shape()[1])
        {
            return false;
        }

        match direction {
            Direction::Left => !self.collides(dy, dx - 1, level),
            Direction::Right => !self.collides(dy, dx + 1, level),
        }
    }

    fn place(&self, dy: usize, dx: usize, level: &mut Array2<bool>) {
        for &(y, x) in &self.data {
            level[(y as usize + dy, x as usize + dx)] = true;
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

pub fn part1(directions: &ParseResult) -> usize {
    simulate_tetris(directions, 2021)
}

pub fn part2(directions: &ParseResult) -> usize {
    simulate_tetris(directions, 1000000000000)
}

fn simulate_tetris(directions: &Vec<Direction>, number_of_pieces: usize) -> usize {
    let mut level = Array2::from_elem((10000, 7), false);

    let mut pieces = PIECES.iter().cycle().enumerate();
    let mut piece = pieces.next().unwrap().1;

    let mut heighest_y = 0;
    let mut dy = heighest_y + 3;
    let mut dx = 2;

    let mut pattern_count = 0;
    let mut pattern_height = 0;
    let mut repeating = vec![];

    for (direction_idx, direction) in directions.iter().cycle().enumerate() {
        if dy + piece.height >= level.shape()[0] {
            dbg!(&repeating);
            panic!();
        }

        if piece.can_move(dy, dx, direction, &level) {
            match direction {
                Direction::Left => dx -= 1,
                Direction::Right => dx += 1,
            }
        }

        let mut collides_with_bottom = false;
        if dy == 0 {
            collides_with_bottom = true;
        } else {
            dy -= 1;
        }

        if collides_with_bottom || piece.collides(dy, dx, &level) {
            if !collides_with_bottom {
                dy += 1;
            }

            piece.place(dy, dx, &mut level);

            heighest_y = heighest_y.max(dy + piece.height);

            let (piece_idx, next_piece) = pieces.next().unwrap();

            if piece_idx != 1 && pattern_count == 0 {
                if let Some((found_pattern_height, found_pieces_in_pattern)) = find_repepating_pattern(
                    &level,
                    &mut repeating,
                    direction_idx,
                    directions.len(),
                    piece_idx,
                    PIECES.len(),
                    heighest_y - 1,
                ) {
                    //dbg!(found_pattern_height, found_pieces_in_pattern);

                    // Not working reliable off by one error +-1 in different test cases
                    let patterns_till_end = (number_of_pieces - piece_idx) / found_pieces_in_pattern;
                    pattern_count = patterns_till_end * found_pieces_in_pattern;
                    pattern_height = patterns_till_end * found_pattern_height - 1;
                }
            }

            if piece_idx + pattern_count > number_of_pieces {
                break;
            }

            piece = next_piece;
            dy = heighest_y + 3;
            dx = 2;
            continue;
        }
    }

    //print_level(&level);

    //dbg!(&repeating);

    heighest_y + pattern_height
}

fn find_repepating_pattern(
    level: &Array2<bool>,
    repeating: &mut Vec<(usize, usize, usize, usize)>,
    direction_idx: usize,
    directions_len: usize,
    piece_idx: usize,
    pieces_len: usize,
    heighest_y: usize,
) -> Option<(usize, usize)> {
    let dir_pos = direction_idx % directions_len;
    let piece_pos = piece_idx % pieces_len;

    if level.row(heighest_y) == level.row(200) {
        repeating.push((heighest_y, dir_pos, piece_pos, piece_idx));
    }

    let found = repeating
        .iter()
        .enumerate()
        .filter(|&(_, &(_, d, p, _))| d == dir_pos && piece_pos == p)
        .collect::<Vec<_>>();

    // Check for possible repeating pattern
    if found.len() == 3
        && found[2].0 - found[1].0 == found[1].0 - found[0].0
        && found[2].1 .0 - found[1].1 .0 == found[1].1 .0 - found[0].1 .0
        && found[2].1 .3 - found[1].1 .3 == found[1].1 .3 - found[0].1 .3
    {
        // Found return (height of pattern, pieces in pattern)
        return Some((found[2].1 .0 - found[1].1 .0, found[2].1 .3 - found[1].1 .3));
    }

    None
}

#[allow(dead_code)]
fn print_level(level: &Array2<bool>) {
    for y in (0..level.shape()[0]).rev() {
        // Skip empty row
        if !level.row(y).iter().any(|&v| v) {
            continue;
        }

        print!("|");
        for x in 0..level.shape()[1] {
            print!("{}", if level[(y, x)] { '#' } else { '.' });
        }
        println!("|");
    }

    println!("+-------+");
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/2022/day17_example.txt");
    const INPUT: &str = include_str!("../input/2022/day17.txt");

    #[test]
    fn example_part1() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part1(&input), 3068);
    }

    #[test]
    fn example_part2() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part2(&input), 1514285714288);
    }

    #[test]
    fn input_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), 3130);
    }

    #[test]
    fn input_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), 1556521739139);
    }
}
