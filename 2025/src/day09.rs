use aoc_runner_derive::{aoc, aoc_generator};

type Number = u64;
type ParseResult = Vec<Vec2>;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Vec2 {
    x: Number,
    y: Number,
}

#[aoc_generator(day9)]
pub fn parse_input(input: &str) -> ParseResult {
    input
        .trim()
        .lines()
        .map(|line| line.parse::<Vec2>().unwrap())
        .collect()
}

impl std::str::FromStr for Vec2 {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let parts: Vec<Number> = input
            .split(',')
            .map(|nb| nb.trim().parse::<Number>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| "Failed to parse integer")?;

        if parts.len() != 2 {
            return Err("Expected three comma-separated numbers");
        }

        Ok(Vec2 {
            x: parts[0],
            y: parts[1],
        })
    }
}

impl Vec2 {
    fn size(&self, other: &Vec2) -> Number {
        (self.x.abs_diff(other.x) + 1) * (self.y.abs_diff(other.y) + 1)
    }
}

#[aoc(day9, part1)]
pub fn part1(input: &ParseResult) -> Number {
    let mut larges_size = 0;

    for idx in 0..input.len() {
        for idy in 0..input.len() {
            let size = input[idx].size(&input[idy]);
            larges_size = larges_size.max(size);
        }
    }

    larges_size
}

#[aoc(day9, part2)]
pub fn part2(input: &ParseResult) -> Number {
    let mut larges_size = 0;
    let mut found = None;

    let mut lines: Vec<_> = input.windows(2).map(|win| (win[0], win[1])).collect();
    lines.push((input[input.len() - 1], input[0]));

    for idx in 0..input.len() {
        'outer: for idy in 0..input.len() {
            let size = input[idx].size(&input[idy]);
            if size <= larges_size {
                continue;
            }

            for pos in input {
                if is_inside(&(input[idx], input[idy]), pos) {
                    continue 'outer;
                }

                for line in &lines {
                    if intersects(&(input[idx], input[idy]), line) {
                        continue 'outer;
                    }
                }
            }

            larges_size = larges_size.max(size);
            found = Some((&input[idx], &input[idy]));
        }
    }

    dbg!(found);
    larges_size
}

fn is_inside(pair: &(Vec2, Vec2), point: &Vec2) -> bool {
    (pair.0.x.min(pair.1.x) + 1..(pair.0.x.max(pair.1.x) - 1)).contains(&point.x)
        && (pair.0.y.min(pair.1.y) + 1..(pair.0.y.max(pair.1.y) - 1)).contains(&point.y)
}

fn intersects(rect_diagonale: &(Vec2, Vec2), line2: &(Vec2, Vec2)) -> bool {
    let x1 = rect_diagonale.0.x as i32;
    let y1 = rect_diagonale.0.y as i32;
    let x2 = rect_diagonale.1.x as i32;
    let y2 = rect_diagonale.1.y as i32;

    let x3 = line2.0.x as i32;
    let y3 = line2.0.y as i32;
    let x4 = line2.1.x as i32;
    let y4 = line2.1.y as i32;

    let deonimnator = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
    if deonimnator == 0 {
        return false;
    }

    let px = ((x1 * y2 - y1 * x2) * (x3 - x4) - (x1 - x2) * (x3 * y4 - y3 * x4)) as f32
        / deonimnator as f32;
    let py = ((x1 * y2 - y1 * x2) * (y3 - y4) - (y1 - y2) * (x3 * y4 - y3 * x4)) as f32
        / deonimnator as f32;

    let xmin = rect_diagonale.0.x.min(rect_diagonale.1.x) as f32;
    let ymin = rect_diagonale.0.y.min(rect_diagonale.1.y) as f32;
    let xmax = rect_diagonale.0.x.max(rect_diagonale.1.x) as f32;
    let ymax = rect_diagonale.0.y.max(rect_diagonale.1.y) as f32;

    px > xmin && px < xmax && py > ymin && py < ymax
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/2025/day9_example.txt");
    const INPUT: &str = include_str!("../input/2025/day9.txt");

    #[test]
    fn example_part1() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part1(&input), 50);
    }

    #[test]
    fn input_part1() {
        let input = parse_input(INPUT);
        assert_eq!(part1(&input), 4749672288);
    }

    #[test]
    fn example_part2() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part2(&input), 24);
    }

    //#[test]
    fn input_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), todo!());
    }
}
