use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type Number = usize;
type Vec2 = crate::utils::Vec2<Number>;
type ParseResult = Vec<Vec2>;

#[aoc_generator(day9)]
pub fn parse_input(input: &str) -> ParseResult {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut parts = line
                .split(',')
                .map(|nb| nb.trim().parse::<Number>().unwrap());
            Vec2 {
                x: parts.next().unwrap(),
                y: parts.next().unwrap(),
            }
        })
        .collect()
}

// All line segments are either horizontal or vertical
#[rustfmt::skip]
#[derive(Debug)]
enum Edge {
    Vertical   { x: Number, y_min: Number, y_max: Number },
    Horizontal { y: Number, x_min: Number, x_max: Number },
}

impl Edge {
    #[rustfmt::skip]
    fn from(p1: &Vec2, p2: &Vec2) -> Self {
        if p1.y == p2.y {
            Edge::Horizontal { y: p1.y, x_min: p1.x.min(p2.x), x_max: p1.x.max(p2.x) }
        } else {
            Edge::Vertical { x: p1.x, y_min: p1.y.min(p2.y), y_max: p1.y.max(p2.y) }
        }
    }

    fn intersects(&self, other: &Edge) -> bool {
        match (self, other) {
            (Edge::Vertical { x, y_min, y_max }, Edge::Horizontal { y, x_min, x_max })
            | (Edge::Horizontal { y, x_min, x_max }, Edge::Vertical { x, y_min, y_max }) => {
                (x_min <= x && x <= x_max) && (y_min <= y && y <= y_max)
            }
            (Edge::Horizontal { .. }, Edge::Horizontal { .. })
            | (Edge::Vertical { .. }, Edge::Vertical { .. }) => false,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Rect {
    p1: Vec2,
    p2: Vec2,
}

impl Rect {
    fn size(&self) -> Number {
        (self.p1.x.abs_diff(self.p2.x) + 1) * (self.p1.y.abs_diff(self.p2.y) + 1)
    }

    fn crosses_rect_inside(&self, line: &Edge) -> bool {
        let rmin_x = self.p1.x.min(self.p2.x);
        let rmax_x = self.p1.x.max(self.p2.x);
        let rmin_y = self.p1.y.min(self.p2.y);
        let rmax_y = self.p1.y.max(self.p2.y);

        match *line {
            Edge::Vertical { x, y_min, y_max } => {
                // Edge is between the left and right walls
                // AND the vertical span of the edge overlaps the vertical interior of the rect
                (rmin_x < x && x < rmax_x) && (y_min < rmax_y && y_max > rmin_y)
            }
            Edge::Horizontal { y, x_min, x_max } => {
                // Edge is between the top and bottom walls
                // AND the horizontal span of the edge overlaps the horizontal interior of the rect
                (rmin_y < y && y < rmax_y) && (x_min < rmax_x && x_max > rmin_x)
            }
        }
    }
}

#[aoc(day9, part1)]
pub fn part1(input: &ParseResult) -> usize {
    input
        .iter()
        .tuple_combinations()
        .map(|(&p1, &p2)| Rect { p1, p2 }.size())
        .max()
        .unwrap()
}

#[aoc(day9, part2)]
pub fn part2(coordinates: &ParseResult) -> usize {
    let mut larges_size = 0;

    let edges = coordinates
        .iter()
        .chain(&[coordinates[0]]) // Append the first to the end to close polygon
        .tuple_windows()
        .map(|(p1, p2)| Edge::from(p1, p2))
        .collect::<Vec<Edge>>();

    // Go over all possible rects
    'outer: for (&p1, &p2) in coordinates.iter().tuple_combinations() {
        let rect = Rect { p1, p2 };
        let size = rect.size();
        if size <= larges_size {
            continue;
        }

        // Reject if polgon edge reaches inside the rect
        for polygon_edge in &edges {
            if rect.crosses_rect_inside(polygon_edge) {
                continue 'outer;
            }
        }

        // Check if rect is inside the polygon via raycast from top
        let check_pos_inside_x = p1.x.min(p2.x) + (p1.x.abs_diff(p2.x) / 2);
        let check_pos_inside_y = p1.y.min(p2.y) + (p1.y.abs_diff(p2.y) / 2);

        let ray = Edge::Vertical {
            x: check_pos_inside_x,
            y_min: 0,
            y_max: check_pos_inside_y,
        };

        // Odd intersection count means inside
        if edges.iter().filter(|edge| edge.intersects(&ray)).count() % 2 != 0 {
            larges_size = size;
        }
    }

    larges_size
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

    #[test]
    fn input_part2() {
        let input = parse_input(INPUT);
        assert_eq!(part2(&input), 1479665889);
    }
}
