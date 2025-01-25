use std::{collections::HashMap, iter, sync::LazyLock};

use aoc_runner_derive::{aoc, aoc_generator};
use glam::{i8vec2, I8Vec2};
use ndarray::iter::Iter;
use tinyvec::TinyVec;

type ParseResult = Vec<(Vec<DigitButton>, u32)>;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Path(TinyVec<[DirectionButton; 16]>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DigitButton {
    Activate,
    Digit(u8),
}

static DIGIT_COORDINATES: LazyLock<HashMap<DigitButton, I8Vec2>> = LazyLock::new(|| {
    HashMap::from([
        (DigitButton::Activate, i8vec2(0, 0)),
        (DigitButton::Digit(0), i8vec2(-1, 0)),
        (DigitButton::Digit(1), i8vec2(-2, 1)),
        (DigitButton::Digit(2), i8vec2(-1, 1)),
        (DigitButton::Digit(3), i8vec2(0, 1)),
        (DigitButton::Digit(4), i8vec2(-2, 2)),
        (DigitButton::Digit(5), i8vec2(-1, 2)),
        (DigitButton::Digit(6), i8vec2(0, 2)),
        (DigitButton::Digit(7), i8vec2(-2, 3)),
        (DigitButton::Digit(8), i8vec2(-1, 3)),
        (DigitButton::Digit(9), i8vec2(0, 3)),
    ])
});

static DIRECTION_COORDINATES: LazyLock<HashMap<DirectionButton, I8Vec2>> = LazyLock::new(|| {
    HashMap::from([
        (DirectionButton::Activate, i8vec2(0, 0)),
        (DirectionButton::Up, i8vec2(-1, 0)),
        (DirectionButton::Down, i8vec2(-1, -1)),
        (DirectionButton::Left, i8vec2(-2, -1)),
        (DirectionButton::Right, i8vec2(0, -1)),
    ])
});

const GAP: I8Vec2 = i8vec2(-2, 0);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DirectionButton {
    #[default]
    Activate,
    Up,
    Down,
    Left,
    Right,
}

impl Path {
    fn from_iter(iter: impl Iterator<Item = DirectionButton>) -> Self {
        Self(iter.collect())
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn iter(&self) -> std::slice::Iter<DirectionButton> {
        self.0.iter()
    }

    fn extend(&mut self, other: Path) {
        self.0.extend(other.0);
    }

    fn push(&mut self, dir: DirectionButton) {
        self.0.push(dir);
    }

    fn is_valid(&self, start: I8Vec2) -> bool {
        let mut pos = start;
        for dir in self.iter() {
            let next = pos + dir.as_ivec2();
            if next == GAP {
                return false;
            }
            pos = next;
        }
        true
    }

    fn find(start: I8Vec2, end: I8Vec2) -> Box<[Path]> {
        let diff = end - start;
        let mut paths = vec![];

        let vertical = iter::repeat_n(
            if diff.y > 0 {
                DirectionButton::Up
            } else {
                DirectionButton::Down
            },
            diff.y.abs() as usize,
        );

        let horizontal = iter::repeat_n(
            if diff.x > 0 {
                DirectionButton::Right
            } else {
                DirectionButton::Left
            },
            diff.x.abs() as usize,
        );

        let horizontal_then_vertical = Path::from_iter(horizontal.clone().chain(vertical.clone()));
        let vertical_then_horizontal = Path::from_iter(vertical.chain(horizontal));

        if horizontal_then_vertical.is_valid(start) {
            paths.push(horizontal_then_vertical);
        }

        if vertical_then_horizontal.is_valid(start) {
            paths.push(vertical_then_horizontal);
        }

        paths.into_boxed_slice()
    }

    fn to_robot(&self) -> Path {
        let mut dir_pos = DirectionButton::Activate;
        let mut dir_presses = Path::default();

        for dir in self.iter() {
            dir_presses.extend(dir_pos.directions_to(*dir)[0].clone());
            dir_presses.push(DirectionButton::Activate);
            dir_pos = *dir;
        }

        dir_presses
    }
}

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.iter().map(|dir| dir.to_string()).collect::<String>())
    }
}

impl std::fmt::Display for DirectionButton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DirectionButton::Up => write!(f, "^"),
            DirectionButton::Down => write!(f, "v"),
            DirectionButton::Left => write!(f, "<"),
            DirectionButton::Right => write!(f, ">"),
            DirectionButton::Activate => write!(f, "A"),
        }
    }
}

impl DigitButton {
    fn from_char(c: char) -> Self {
        if c.is_digit(10) {
            Self::Digit(c.to_digit(10).unwrap() as u8)
        } else if c == 'A' {
            Self::Activate
        } else {
            panic!("Invalid character: {}", c);
        }
    }

    fn directions_to(&self, dest: DigitButton) -> Box<[Path]> {
        Path::find(DIGIT_COORDINATES[self], DIGIT_COORDINATES[&dest])
    }
}

impl DirectionButton {
    fn as_ivec2(&self) -> I8Vec2 {
        match self {
            DirectionButton::Up => I8Vec2::Y,
            DirectionButton::Down => -I8Vec2::Y,
            DirectionButton::Left => -I8Vec2::X,
            DirectionButton::Right => I8Vec2::X,
            DirectionButton::Activate => panic!("Activate has no direction"),
        }
    }

    fn directions_to(&self, dest: DirectionButton) -> Box<[Path]> {
        Path::find(DIRECTION_COORDINATES[self], DIRECTION_COORDINATES[&dest])
    }
}

#[aoc_generator(day21)]
pub fn parse_input(input: &str) -> ParseResult {
    input
        .trim()
        .lines()
        .map(|line| {
            (
                line.chars().map(|c| DigitButton::from_char(c)).collect(),
                line.replace('A', "").parse().unwrap(),
            )
        })
        .collect()
}

// fn directions_to_directions(directions: &Path) -> Path {
//     let mut dir_pos = DirectionButton::Activate;
//     let mut dir_presses = Path::default();
//     for dir in directions.iter() {
//         dir_presses.extend(dir_pos.directions_to(*dir));
//         dir_presses.push(DirectionButton::Activate);
//         dir_pos = *dir;
//     }
//     dir_presses
// }

// fn determine_keypresses(input: &Vec<DigitButton>) -> Path {
//     let mut position = DigitButton::Activate;
//     let mut dir_presses = Path::default();

//     for nb in input {
//         let dir = position.directions_to(*nb);
//         dir_presses.extend(dir);
//         dir_presses.push(DirectionButton::Activate);
//         position = *nb;
//     }

//     println!("{dir_presses}");
//     let dir_presses_2 = directions_to_directions(&dir_presses);

//     println!("{dir_presses_2}");
//     let dir_presses_3 = directions_to_directions(&dir_presses_2);

//     println!("{dir_presses_3}");
//     dir_presses_3
// }

#[aoc(day21, part1)]
pub fn part1(input: &ParseResult) -> isize {
    // let mut result = 0;

    // for (numbers_to_press, number) in input {
    //     let mut position = DigitButton::Activate;
    //     let mut dir_presses = Path::default();

    //     for nb in numbers_to_press {
    //         let dir = position.directions_to(*nb);
    //         dir_presses.extend(dir);
    //         dir_presses.push(DirectionButton::Activate);
    //         position = *nb;
    //     }

    //     println!("{dir_presses}",);

    //     let dir_presses_2 = directions_to_directions(&dir_presses);

    //     println!("{dir_presses_2}");

    //     let dir_presses_3 = directions_to_directions(&dir_presses_2);

    //     println!("{dir_presses_3}");

    //     result += dbg!(dbg!(&dir_presses_3.len()) * *number as usize);
    // }

    // result as isize
    todo!()
}

#[aoc(day21, part2)]
pub fn part2(input: &ParseResult) -> isize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/2024/day21_example.txt");
    const INPUT: &str = include_str!("../input/2024/day21.txt");

    #[test]
    fn example_part1() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part1(&input), 126384);
    }

    // #[test]
    // fn test_example_sub_numbers() {
    //     assert_eq!(
    //         determine_keypresses(&parse_input("980A")[0].0).len(),
    //         "<v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A".len()
    //     );
    //     assert_eq!(
    //         determine_keypresses(&parse_input("179A")[0].0).len(),
    //         "<v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A".len()
    //     );
    //     assert_eq!(
    //         determine_keypresses(&parse_input("456A")[0].0).len(),
    //         "<v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A".len()
    //     );
    //     assert_eq!(
    //         determine_keypresses(&parse_input("379A")[0].0).len(),
    //         "<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A".len()
    //     );
    // }

    #[test]
    fn input_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), todo!());
    }

    //#[test]
    fn example_part2() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part2(&input), todo!());
    }

    //#[test]
    fn input_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), todo!());
    }
}
