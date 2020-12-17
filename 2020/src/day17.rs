#[macro_use]
extern crate itertools;

use std::collections::HashMap;

type Dimension3 = HashMap<(i32, i32, i32), bool>;
type Dimension4 = HashMap<(i32, i32, i32, i32), bool>;

fn main() {
    let data = parse(include_str!("../input/2020/day17.txt"));

    let now = std::time::Instant::now();
    let part1 = part1(&data);
    println!(
        "Part1: {}  [{}]",
        part1,
        humantime::format_duration(now.elapsed())
    );
    //assert_eq!(part1, 26026);


    let now = std::time::Instant::now();
    let part2 = part2(&data);
    println!(
        "Part2: {}  [{}]",
        part2,
        humantime::format_duration(now.elapsed())
    );
}

fn parse(input: &str) -> Dimension3 {
    let mut dim = HashMap::new();

    input.trim().lines().enumerate().for_each(|(y, line)| {
        line.trim().chars().enumerate().for_each(|(x, c)| {
            if c == '#' {
                &dim.insert((x as i32, y as i32, 0), true);
            }
        })
    });

    dim
}

fn part2(data: &Dimension3) -> usize {
    let mut dim: Dimension4 = HashMap::new();

    data.iter().for_each(|((x, y, z), v)| {
        dim.insert((*x, *y, *z, 0), *v);
    });


    let mut neighbors = iproduct!(-1..=1, -1..=1, -1..=1, -1..=1).collect::<Vec<(i32, i32, i32, i32)>>();
    neighbors.retain(|x| *x != (0, 0, 0, 0));
    let neighbors = &neighbors;

    //dbg!(neighbors);

    for cycle in 1..=6 {
        let mut dim_clone = dim.clone();

        // Extend dimension around active cubes
        for (x, y, z, w) in dim.keys() {
            if *dim.get(&(*x, *y, *z, *w)).unwrap() == true {
                for (dx, dy, dz, dw) in neighbors {
                    if dim.get(&(*x + *dx, *y + *dy, *z + *dz, *w + *dw)).is_none() {
                        dim_clone.insert((*x + *dx, *y + *dy, *z + *dz, *w + *dw), false);
                    }
                }
            }
        }

        dim = dim_clone;
        let mut dim_clone = dim.clone();

        // Run cube update
        for (x, y, z, w) in dim.keys() {
            let mut active_neighbors = 0;
            for (dx, dy, dz, dw) in neighbors {
                if let Some(value) = dim.get(&(*x + *dx, *y + *dy, *z + *dz, *w + *dw)) {
                    if *value {
                        active_neighbors += 1;
                    }
                } else {
                    dim_clone.insert((*x + *dx, *y + *dy, *z + *dz,*w + *dw), false);
                }
            }

            let cur = dim_clone.get_mut(&(*x, *y, *z, *w)).unwrap();

            if *cur == true {
                if active_neighbors != 2 && active_neighbors != 3 {
                    *cur = false;
                }
            } else if active_neighbors == 3 {
                *cur = true;
            }
        }

        dim = dim_clone;

        // println!("Cycle: {}", cycle);
        // print_map(&dim);
        // println!();
        // println!("-------------------------");
        // println!();
    }


    dim.values().filter(|v| **v).count()
}

fn part1(data: &Dimension3) -> usize {
    let mut dim = data.clone();

    let mut neighbors = iproduct!(-1..=1, -1..=1, -1..=1).collect::<Vec<(i32, i32, i32)>>();
    neighbors.retain(|x| *x != (0, 0, 0));
    let neighbors = &neighbors;

    //dbg!(neighbors);

    for cycle in 1..=6 {
        let mut dim_clone = dim.clone();

        // Extend dimension around active cubes
        for (x, y, z) in dim.keys() {
            if *dim.get(&(*x, *y, *z)).unwrap() == true {
                for (dx, dy, dz) in neighbors {
                    if dim.get(&(*x + *dx, *y + *dy, *z + *dz)).is_none() {
                        dim_clone.insert((*x + *dx, *y + *dy, *z + *dz), false);
                    }
                }
            }
        }

        dim = dim_clone;
        let mut dim_clone = dim.clone();

        // Run cube update
        for (x, y, z) in dim.keys() {
            let mut active_neighbors = 0;
            for (dx, dy, dz) in neighbors {
                if let Some(value) = dim.get(&(*x + *dx, *y + *dy, *z + *dz)) {
                    if *value {
                        active_neighbors += 1;
                    }
                } else {
                    dim_clone.insert((*x + *dx, *y + *dy, *z + *dz), false);
                }
            }

            let cur = dim_clone.get_mut(&(*x, *y, *z)).unwrap();

            if *cur == true {
                if active_neighbors != 2 && active_neighbors != 3 {
                    *cur = false;
                }
            } else if active_neighbors == 3 {
                *cur = true;
            }
        }

        dim = dim_clone;

        // println!("Cycle: {}", cycle);
        // print_map(&dim);
        // println!();
        // println!("-------------------------");
        // println!();
    }


    dim.values().filter(|v| **v).count()
}

fn print_map(dim: &Dimension3) {
    for z in -50..50 {
        let mut any2 = false;
        for y in -50..50 {
            let mut any = false;
            for x in -50..50 {
                if let Some(v) = dim.get(&(x, y, z)) {
                    if *v {
                        print!("{}", "#");
                    } else {
                        print!("{}", ".");
                    }

                    any = true;
                }
            }

            if any {
                println!();
                any2 = true;
            }
        }

        if any2 {
            println!("z = {}", z);
            println!();
        }
    }
}