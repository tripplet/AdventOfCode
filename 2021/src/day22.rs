use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::{max, min};
use std::error::Error;
use std::ops::RangeInclusive;
use std::str::FromStr;

#[derive(PartialEq, Clone)]
struct Cuboid {
    x: RangeInclusive<i32>,
    y: RangeInclusive<i32>,
    z: RangeInclusive<i32>,
    on: bool,
}

const INPUT: &str = include_str!("../input/2021/day22.txt");

pub fn main() {
    let mut now = std::time::Instant::now();
    let cubiods = parse_input(INPUT).unwrap();
    println!("Parsing [{}]\n", humantime::format_duration(now.elapsed()));

    now = std::time::Instant::now();
    println!(
        "Part1: {} [{}]",
        part1(&cubiods),
        humantime::format_duration(now.elapsed())
    );

    now = std::time::Instant::now();
    println!(
        "Part2: {} [{}]",
        part2(&cubiods),
        humantime::format_duration(now.elapsed())
    );
}

lazy_static! {
    // on x=10..12,y=10..12,z=10..12
    // off x=-48..-32,y=26..41,z=-47..-37
    static ref REGEX_CUBOID: Regex = Regex::new(r"(?P<state>(on)|(off))\s+x=(?P<x_start>-?\d+)..(?P<x_end>-?\d+),y=(?P<y_start>-?\d+)..(?P<y_end>-?\d+),z=(?P<z_start>-?\d+)..(?P<z_end>-?\d+)").unwrap();
}

impl FromStr for Cuboid {
    type Err = Box<dyn Error>;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if let Some(values) = REGEX_CUBOID.captures(input.trim()) {
            return Ok(Cuboid {
                x: values.name("x_start").unwrap().as_str().parse()?
                    ..=values.name("x_end").unwrap().as_str().parse()?,
                y: values.name("y_start").unwrap().as_str().parse()?
                    ..=values.name("y_end").unwrap().as_str().parse()?,
                z: values.name("z_start").unwrap().as_str().parse()?
                    ..=values.name("z_end").unwrap().as_str().parse()?,
                on: values.name("state").unwrap().as_str() == "on",
            });
        }
        Err("input does not match valid format".into())
    }
}

impl std::fmt::Debug for Cuboid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cuboid")
            .field("x", &self.x)
            .field("y", &self.y)
            .field("z", &self.y)
            .field("on", &self.on)
            .field("volume", &self.volume())
            .finish()
    }
}

impl Cuboid {
    fn intersection(&self, other: &Cuboid) -> Option<Cuboid> {
        if self.x.start() > other.x.end() || self.y.start() > other.y.end() || self.z.start() > other.z.end() ||
            other.x.start() > self.x.end() || other.y.start() > self.y.end() || other.z.start() > self.z.end() {
            None
        } else {
            Some(Cuboid {
                x: max(*self.x.start(), *other.x.start())..=min(*self.x.end(), *other.x.end()),
                y: max(*self.y.start(), *other.y.start())..=min(*self.y.end(), *other.y.end()),
                z: max(*self.z.start(), *other.z.start())..=min(*self.z.end(), *other.z.end()),
                on: match (other.on, self.on) {
                    (false, true) => true,
                    (true, true) => false,
                    (false, false) => true,
                    (true, false) => false
                },
            })
        }
    }

    fn volume(&self) -> i128 {
        (self.x.end() - self.x.start() + 1) as i128
            * (self.y.end() - self.y.start() + 1) as i128
            * (self.z.end() - self.z.start() + 1) as i128
    }
}

fn parse_input(input: &str) -> Result<Vec<Cuboid>, Box<dyn Error>> {
    input.trim().lines().map(Cuboid::from_str).collect()
}

fn part1(cubiods: &[Cuboid]) -> i128 {
    let limit = Cuboid{x: -50..=50, y:-50..=50, z:-50..=50, on: false};

    let filtered = cubiods.iter().filter(|&c| {
        let inter = c.intersection(&limit);
        if let Some(mut mut_inter) = inter {
            mut_inter.on = c.on;
            &mut_inter == c
        } else {
            false
        }
    }).cloned().collect::<Vec<_>>();
    part2(&filtered)
}

fn part2(cubiods: &[Cuboid]) -> i128 {
    let mut all_cubes: Vec<Cuboid> = vec![];
    let mut on_cubiods = 0_i128;

    for cub in cubiods {
        // println!("Applying ---------------------------------");
        // dbg!(&cub);
        // println!("------------------------------------------");

        let mut new_cubes = vec![];

        for existing_cub in &all_cubes {
            if let Some(intersect) = cub.intersection(existing_cub) {
                let vv = intersect.volume();

                if vv == 0 {
                    panic!();
                }

                on_cubiods += match (existing_cub.on, cub.on) {
                    (false, true) => vv,
                    (true, true) => -vv,
                    (false, false) => vv,
                    (true, false) => -vv
                } as i128;

                new_cubes.push(intersect);
            }
        }

        if cub.on {
            new_cubes.push(cub.clone());
            on_cubiods += cub.volume() as i128;
        }

        // dbg!(&new_cubes);

        all_cubes.append(&mut new_cubes);

        // println!("total on: {}", on_cubiods);
        // use std::io::{stdin, stdout, Read, Write};
        // stdout().flush().unwrap();
        // //stdin().read(&mut [0]).unwrap();
        // println!("#########################################################################\n\n\n");
    }

    on_cubiods
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = include_str!("../input/2021/day22_example1.txt");
    const EXAMPLE_2: &str = include_str!("../input/2021/day22_example2.txt");
    const EXAMPLE_3: &str = include_str!("../input/2021/day22_example3.txt");

    #[test]
    fn parse() {
        let c1 = "off x=-48..-32,y=26..41,z=-47..-37".parse::<Cuboid>().unwrap();
        let c2 = "on x=-12..35,y=6..50,z=-50..-2".parse::<Cuboid>().unwrap();

        assert_eq!(
            c1,
            Cuboid {
                x: -48..=-32,
                y: 26..=41,
                z: -47..=-37,
                on: false
            }
        );
        assert_eq!(
            c2,
            Cuboid {
                x: -12..=35,
                y: 6..=50,
                z: -50..=-2,
                on: true
            }
        );
    }

    #[test]
    fn volume() {
        assert_eq!(27, "on x=10..12,y=10..12,z=10..12".parse::<Cuboid>().unwrap().volume());
        assert_eq!(1, "on x=10..10,y=10..10,z=10..10".parse::<Cuboid>().unwrap().volume());
    }

    #[test]
    fn examples() {
        assert_eq!(39, part1(&parse_input(&EXAMPLE_1).unwrap()));
        assert_eq!(590784, part1(&parse_input(&EXAMPLE_2).unwrap()));

        assert_eq!(2758514936282235, part2(&parse_input(&EXAMPLE_3).unwrap()));
    }

    #[test]
    fn input() {
        assert_eq!(620241, part1(&parse_input(&INPUT).unwrap()));
        assert_eq!(1284561759639324, part2(&parse_input(&INPUT).unwrap()));
    }

    #[test]
    fn intersect() {
        let c1 = "on x=10..12,y=10..12,z=10..12".parse::<Cuboid>().unwrap();
        let c2 = "off x=11..13,y=11..13,z=11..13".parse::<Cuboid>().unwrap();

        assert_eq!("on x=11..12,y=11..12,z=11..12".parse::<Cuboid>().unwrap(), c1.intersection(&c2).unwrap());
    }
}
