use std::collections::HashMap;
use std::error::Error;
use std::ops::Range;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct Image {
    algorithm: Vec<bool>,
    image: HashMap<Point, bool>,
    x: Range<i32>,
    y: Range<i32>,
    infinite: bool,
}

impl FromStr for Image {
    type Err = Box<dyn Error>;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut lines = input.trim().lines();

        let algorithm = lines
                .next()
                .ok_or("missing enhancement algorithm data")?
                .chars()
                .map(|symbol| symbol == '#')
                .collect::<Vec<_>>();
        let mut image = HashMap::new();
        let mut y_max = 0;
        let mut x_max = 0;
        for (y, line) in lines.skip(1).enumerate() {
            for (x, symbol) in line.char_indices() {
                image.insert(
                    Point {
                        y: y as i32,
                        x: x as i32,
                    },
                    symbol == '#',
                );
                x_max = x;
            }
            y_max = y;
        }

        Ok(Image {
            image,
            algorithm,
            x: 0..(x_max+1) as i32,
            y: 0..(y_max+1) as i32,
            infinite: false,
        })
    }
}

impl Image {
    fn get_pixel(&self, p: &Point) -> bool {
        if self.x.contains(&p.x) && self.y.contains(&p.y) {
            *self.image.get(p).unwrap_or(&false)
        }
        else {
            self.infinite
        }
    }

    fn get_pixel_area_as_number(&self, p: &Point) -> usize {
        [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 0),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ]
        .iter()
        .map(|(dy, dx)| {
            self.get_pixel(&Point {
                y: p.y + dy,
                x: p.x + dx,
            }) as usize
        })
        .rev()
        .enumerate()
        .fold(0, |sum, (pos, value)| sum + value * (1 << pos))
    }

    fn enhance(&mut self) {
        let mut new_image = self.image.clone();

        for y in self.y.start-2..self.y.end+2 {
            for x in self.x.start-2..self.x.end+2 {
                let point = Point{y, x};
                new_image.insert(point.clone(), self.algorithm[self.get_pixel_area_as_number(&point)]);
            }
        }

        self.x = self.x.start-2..self.x.end+2;
        self.y = self.y.start-2..self.y.end+2;

        if !self.infinite {
            self.infinite = self.algorithm[0];
        } else {
            self.infinite = self.algorithm[(1<<9)-1];
        }

        self.image = new_image;
    }

    fn active_pixel(&self) -> usize {
        self.image.values().filter(|&&value| value).count()
    }

    #[allow(dead_code)]
    fn print(&self) -> String {
        let mut pic = String::from("");
        for y in self.y.start-2..self.y.end+2 {
            for x in self.x.start-2..self.x.end+2 {
                pic.push(if self.get_pixel(&Point{y,x}) {'#'} else {'.'});
            }
            pic.push('\n');
        }
        pic
    }
}

const INPUT: &str = include_str!("../input/2021/day20.txt");

pub fn main() {
    let mut now = std::time::Instant::now();
    let data = INPUT.parse::<Image>().unwrap();
    println!("Parsing [{}]\n", humantime::format_duration(now.elapsed()));

    now = std::time::Instant::now();
    println!(
        "Part1: {} [{}]",
        part1(&data),
        humantime::format_duration(now.elapsed())
    );

    now = std::time::Instant::now();
    println!(
        "Part2: {} [{}]",
        part2(&data),
        humantime::format_duration(now.elapsed())
    );
}

fn part1(input: &Image) -> usize {
    let mut image = input.clone();

    image.enhance();
    image.enhance();

    image.active_pixel()
}

fn part2(input: &Image) -> usize {
    let mut image = input.clone();
    for _ in 0..50 {
        image.enhance();
    }
    image.active_pixel()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/2021/day20_example.txt");

    #[test]
    fn example() {
        assert_eq!(35, part1(&EXAMPLE.parse::<Image>().unwrap()));
        assert_eq!(3351, part2(&EXAMPLE.parse::<Image>().unwrap()));
    }

    #[test]
    fn input() {
        assert_eq!(4928, part1(&INPUT.parse::<Image>().unwrap()));
        assert_eq!(16605, part2(&INPUT.parse::<Image>().unwrap()));
    }
}