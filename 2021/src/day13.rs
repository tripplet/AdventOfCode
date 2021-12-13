use std::error::Error;
use std::fmt::{self, Display};
use std::str::FromStr;
use ndarray::Array2;

const INPUT: &str = include_str!("../input/2021/day13.txt");

#[derive(Debug, PartialEq, Copy, Clone)]
enum Fold {
    Y(u16),
    X(u16),
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
struct Point {
    x: u16,
    y: u16,
}

#[derive(Debug, PartialEq, Clone)]
struct Paper {
    paper: Vec<Point>,
    instructions: Vec<Fold>,
}

impl FromStr for Point {
    type Err = Box<dyn Error>;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut parts = line.trim().split(",");
        Ok(Point {
            x: parts.next().ok_or("missing x")?.parse()?,
            y: parts.next().ok_or("missing x")?.parse()?,
        })
    }
}

impl FromStr for Fold {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut parts = line.trim().split("=");

        let axis = parts.next().and_then(|a| a.chars().last()).ok_or("invalid axis")?;
        let nb = parts
            .next()
            .and_then(|nb| nb.parse().ok())
            .ok_or("invalid row/column")?;

        match axis {
            'x' => Ok(Fold::X(nb)),
            'y' => Ok(Fold::Y(nb)),
            _ => Err(format!("Invalid axis {}", axis)),
        }
    }
}

impl FromStr for Paper {
    type Err = Box<dyn Error>;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let lines = input.trim().replace("\r", "");
        let mut parts = lines.split("\n\n");

        Ok(Paper {
            paper: parts
                .next()
                .ok_or("missing positions")?
                .lines()
                .map(|line| line.parse())
                .collect::<Result<Vec<Point>, _>>()?,
            instructions: parts
                .next()
                .ok_or("missing fold instructions")?
                .lines()
                .map(|line| line.parse())
                .collect::<Result<Vec<Fold>, _>>()?,
        })
    }
}

impl Display for Paper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let shape = self.paper.iter().fold([0_usize, 0_usize], |shape, p| {
            [
                std::cmp::max(shape[0], (p.y + 1) as usize),
                std::cmp::max(shape[1], (p.x + 1) as usize),
            ]
        });

        let mut output_matrix = Array2::from_elem(shape, ' ');
        for point in &self.paper {
            output_matrix[[point.y as usize, point.x as usize]] = '#';
        }

        for y in 0..shape[0] {
            for x in 0..shape[1] {
                write!(f, "{}", output_matrix[[y, x]])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Paper {
    fn fold_once(&mut self, fold: Fold) {
        Paper::fold(&mut self.paper, fold);
        self.compress()
    }

    fn compress(&mut self) {
        self.paper.sort_unstable();
        self.paper.dedup();
    }

    fn fold(paper: &mut Vec<Point>, fold: Fold) {
        for point in paper.iter_mut() {
            if let Fold::X(fold_x) = fold {
                if point.x > fold_x {
                    point.x = 2 * fold_x - point.x;
                }
            } else if let Fold::Y(fold_y) = fold {
                if point.y > fold_y {
                    point.y = 2 * fold_y - point.y;
                }
            }
        }
    }

    fn execute_all_folds(&mut self) {
        for fold in (&mut self.instructions).iter() {
            Paper::fold(&mut self.paper, *fold);
        }
        self.compress();
    }

    fn len(&self) -> usize {
        self.paper.len()
    }
}

pub fn main() {
    let paper = INPUT.parse::<Paper>().unwrap();

    println!("Part1: {}", part1(&paper));
    println!("Part2:\n{}", part2(&paper));
}

fn part1(paper: &Paper) -> usize {
    let mut paper = paper.clone();
    paper.fold_once(paper.instructions[0]);
    paper.len()
}

fn part2(paper: &Paper) -> impl Display {
    let mut paper = paper.clone();
    paper.execute_all_folds();
    paper
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/2021/day13_example.txt");

    #[test]
    fn parse_fold() {
        assert_eq!(Fold::X(163), "fold along x=163".parse().unwrap());
        assert_eq!(Fold::Y(22), "fold along y=22".parse().unwrap());
    }

    #[test]
    fn part1_example() {
        assert_eq!(17, part1(&EXAMPLE.parse().unwrap()));
    }

    #[test]
    fn part2_example() {
        let expected = "#####\n#   #\n#   #\n#   #\n#####";
        let result = format!("{}", part2(&EXAMPLE.parse().unwrap()));
        assert_eq!(expected, result.trim());
    }

    #[test]
    fn part1_on_input() {
        assert_eq!(618, part1(&INPUT.parse().unwrap()));
    }

    #[test]
    fn part2_input() {
        let expected =
"##  #    ###  #### #  # #### #  # #  #
#  # #    #  # #    # #  #    # #  #  #
#  # #    #  # ###  ##   ###  ##   #  #
#### #    ###  #    # #  #    # #  #  #
#  # #    # #  #    # #  #    # #  #  #
#  # #### #  # #### #  # #    #  #  ##";
        let result = format!("{}", part2(&INPUT.parse().unwrap()));
        assert_eq!(expected, result.trim());
    }
}
