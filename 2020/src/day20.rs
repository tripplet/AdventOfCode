//use rayon::prelude::*;
#[macro_use(s)]
extern crate ndarray;

use colored::Colorize;
use std::collections::HashSet;
use std::rc::Rc;

use ndarray::{Array2, Axis};

const TOP: usize = 0;
const RIGHT: usize = 1;
const BOTTOM: usize = 2;
const LEFT: usize = 3;

struct SatPicture<'a> {
    unsorted_tiles: &'a [Tile],
    tiles: Vec<Vec<Option<Tile>>>,
    used_tiles: HashSet<u16>,
    side_len: usize,
}

#[derive(Clone)]
struct Tile {
    id: u16,
    map: Rc<Array2<char>>,
    edges: [u16; 4],
    variant: u8,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: [{}, {}, {}, {}]",
            self.id, self.edges[TOP], self.edges[RIGHT], self.edges[BOTTOM], self.edges[LEFT]
        )
    }
}

impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

const SHIFT: u8 = 6; // u16 need shift down by 6 for tiles where edges are of size 10
fn mirrored(value: u16) -> u16 { value.reverse_bits() >> SHIFT }


impl Tile {
    fn new(input: &str) -> Self {
        let parts = input.trim().split(":").collect::<Vec<_>>();

        fn from_binary(chars: &Vec<char>) -> u16 {
            u16::from_str_radix(
                chars.iter().map(|c| if *c == '#' { '1' } else { '0' }).collect::<String>().as_str(),
                2,
            ).unwrap()
        }

        let map = parts[1]
            .trim()
            .lines()
            .map(|line| line.trim().chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let top = from_binary(&map[0]);
        let right = from_binary(&map.iter().map(|line| *line.last().unwrap()).collect());
        let bottom = from_binary(&map.last().unwrap());
        let left = from_binary(&map.iter().map(|line| line[0]).collect());

        Tile {
            id: parts[0].split(" ").nth(1).unwrap().parse().unwrap(),
            map: Rc::new(
                Array2::from_shape_vec(
                    (map[0].len(), map.len()),
                    map.iter().flatten().map(|c| *c).collect::<Vec<_>>(),
                )
                .unwrap(),
            ),
            edges: [top, right, bottom, left],
            variant: 0,
        }
    }

    fn parse(input: &str) -> Vec<Self> {
        input
            .trim()
            .replace("\r", "")
            .split("\n\n")
            .map(|tile_str| Tile::new(tile_str))
            .collect()
    }

    fn rotate(&self) -> Self {
        Tile {
            edges: [mirrored(self.edges[LEFT]), self.edges[TOP], mirrored(self.edges[RIGHT]), self.edges[BOTTOM]],
            map: self.map.clone(),
            ..*self
        }
    }

    fn mirror_vertical(&self) -> Self {
        Tile {
            edges: [mirrored(self.edges[TOP]), self.edges[LEFT], mirrored(self.edges[BOTTOM]), self.edges[RIGHT]],
            map: self.map.clone(),
            ..*self
        }
    }

    fn mirror_horizontal(&self) -> Self {
        Tile {
            edges: [self.edges[BOTTOM], mirrored(self.edges[RIGHT]), self.edges[TOP], mirrored(self.edges[LEFT])],
            map: self.map.clone(),
            ..*self
        }
    }

    fn variant(&self, var: u8) -> Self {
        let mut new_variant = self.clone();
        let modulo_4 = var % 4;

        if var >= 4 {
            new_variant = self.rotate();
        }

        if modulo_4 == 1 || modulo_4 == 3 {
            new_variant = new_variant.mirror_horizontal();
        }

        if modulo_4 == 2 || modulo_4 == 3 {
            new_variant = new_variant.mirror_vertical();
        }

        Tile { variant: var, map: self.map.clone(), ..new_variant }
    }

    fn updated_map(&self) -> Self {
        let mut new_map: Array2<char> = (&*self.map).to_owned();
        let modulo_4 = self.variant % 4;

        if self.variant >= 4 {
            new_map = new_map.reversed_axes();
            new_map.invert_axis(Axis(1));
        }

        if modulo_4 == 1 || modulo_4 == 3 {
            new_map.invert_axis(Axis(0));
        }

        if modulo_4 == 2 || modulo_4 == 3 {
            new_map.invert_axis(Axis(1));
        }

        Tile { variant: self.variant, map: Rc::new(new_map.to_owned()), ..*self }
    }
}

impl SatPicture<'_> {
    fn next_unused_tile(&self, starting_at: usize) -> Option<&Tile> {
        self.unsorted_tiles.iter().filter(|t| !self.used_tiles.contains(&t.id)).skip(starting_at).next()
    }

    fn does_fit(&self, to_test: &Tile, (y, x): (usize, usize)) -> bool {
        let top = self.tiles[y - 1][x].as_ref();
        let left = self.tiles[y][x - 1].as_ref();

        (top.is_none() || top.unwrap().edges[BOTTOM] == to_test.edges[TOP])
            && (left.is_none() || left.unwrap().edges[RIGHT] == to_test.edges[LEFT])
    }
}

fn main() {
    let example_tiles = Tile::parse(include_str!("../input/2020/day20_example.txt"));
    let tiles = Tile::parse(include_str!("../input/2020/day20.txt"));
    assert_eq!(part1(&example_tiles), 20899048083289);

    let now = std::time::Instant::now();
    let part1_result = part1(&tiles);

    println!(
        "Part1: {}  [{}]",
        part1_result.to_string().yellow(),
        humantime::format_duration(now.elapsed()).to_string().blue()
    );
    assert_eq!(part1_result, 11788777383197);

    assert_eq!(part2(&example_tiles), 273);

    let now = std::time::Instant::now();
    let part2_result = part2(&tiles);
    println!(
        "Part2: {}  [{}]",
        part2_result.to_string().yellow(),
        humantime::format_duration(now.elapsed()).to_string().blue()
    );
    assert_eq!(part2_result, 2242);
}

fn part1(tiles: &Vec<Tile>) -> usize {
    let side_len = (tiles.len() as f64).sqrt().round() as usize;

    // start at 1,1 easier bounds checking
    let mut pic = SatPicture {
        unsorted_tiles: tiles,
        tiles: vec![vec![None; side_len + 2]; side_len + 2],
        side_len,
        used_tiles: HashSet::with_capacity(tiles.len()),
    };

    let result = solve_rec((1, 1), &mut pic, 0);

    if result {
        pic.tiles[1][1].as_ref().unwrap().id as usize
            * pic.tiles[1][side_len].as_ref().unwrap().id as usize
            * pic.tiles[side_len][1].as_ref().unwrap().id as usize
            * pic.tiles[side_len][side_len].as_ref().unwrap().id as usize
    } else {
        0
    }
}

fn part2(tiles: &Vec<Tile>) -> usize {
    let seamonster_chars = (
 "                  # ".to_owned() +
&"#    ##    ##    ###".to_owned() +
&" #  #  #  #  #  #   ").chars().filter(|c| *c == ' ' || *c == '#').collect::<Vec<_>>();
    let seamonster = Array2::from_shape_vec((3, 20), seamonster_chars).unwrap();

    let side_len = (tiles.len() as f64).sqrt().round() as usize;

    // start at 1,1 easier bounds checking
    let mut pic = SatPicture {
        unsorted_tiles: tiles,
        tiles: vec![vec![None; side_len + 2]; side_len + 2],
        side_len,
        used_tiles: HashSet::with_capacity(tiles.len()),
    };

    assert_eq!(true, solve_rec((1, 1), &mut pic, 0));

    // Pre allocate image
    let tile_size = tiles[0].map.as_ref().shape()[0] - 2;
    let mut orig_pic = Array2::from_elem((pic.side_len * tile_size, pic.side_len * tile_size), '-');

    // Create correct orientations of the inner map data in the tiles
    // and build complete image
    for y in 1..pic.tiles.len() {
        for x in 1..pic.tiles[0].len() {
            if let Some(t) = &pic.tiles[y][x] {
                let mut img_slice = orig_pic.slice_mut(s![
                    tile_size * (y - 1)..tile_size * y,
                    tile_size * (x - 1)..tile_size * x
                ]);

                // Remove borders of tile
                let final_tile = t
                    .updated_map()
                    .map
                    .as_ref()
                    .slice(s![1..tile_size + 1, 1..tile_size + 1])
                    .to_owned();

                // Assign to final image
                img_slice.assign(&final_tile);
            }
        }
    }

    for variant in 0..8 {
        let modulo_4 = variant % 4;
        let mut result_image = orig_pic.clone();

        if variant >= 4 {
            result_image = result_image.reversed_axes();
            result_image.invert_axis(Axis(1));
        }

        if modulo_4 == 1 || modulo_4 == 3 {
            result_image.invert_axis(Axis(0));
        }

        if modulo_4 == 2 || modulo_4 == 3 {
            result_image.invert_axis(Axis(1));
        }

        // println!();
        // println!("Variant: {}", variant);
        // for y in 0..result_image.shape()[0] {
        //     for x in 0..result_image.shape()[0] {
        //         print!("{} ", result_image[[y,x]])
        //     }
        //     println!();
        // }

        let mut nb_seamonsters = 0;

        for y in 0..result_image.shape()[0] - 3 {
            'img_loop: for x in 0..result_image.shape()[1] - 20 {
                for sea_y in 0..seamonster.shape()[0] {
                    for sea_x in 0..seamonster.shape()[1] {
                        if seamonster[[sea_y, sea_x]] == '#'
                            && result_image[[y + sea_y, x + sea_x]] != seamonster[[sea_y, sea_x]]
                        {
                            continue 'img_loop;
                        }
                    }
                }

                nb_seamonsters += 1;
            }
        }

        if nb_seamonsters != 0 {
            let all_hashes = orig_pic.iter().filter(|c| **c == '#').count();
            let seamonster_hashes = seamonster.iter().filter(|c| **c == '#').count();

            return all_hashes - nb_seamonsters * (seamonster_hashes);
        }
    }

    0
}

fn solve_rec((y, x): (usize, usize), pic: &mut SatPicture, starting_at: usize) -> bool {
    let mut tiles_to_skip = starting_at;

    // loop over all potenital next tiles
    loop {
        let new_potential_tile_2 = pic.next_unused_tile(tiles_to_skip);

        if let Some(new_potential_tile_2) = new_potential_tile_2 {
            let new_potential_tile = Tile {
                variant: new_potential_tile_2.variant,
                map: new_potential_tile_2.map.clone(),
                ..*new_potential_tile_2
            };
            pic.used_tiles.insert(new_potential_tile.id);

            // Try all variants of the potential tile
            for var in 0..8 {
                let to_test = new_potential_tile.variant(var);

                if pic.does_fit(&to_test, (y, x)) {
                    pic.tiles[y][x] = Some(to_test);

                    let mut new_x = x;
                    let new_y = if y < pic.side_len {
                        y + 1
                    } else {
                        new_x += 1;
                        1
                    };

                    // Stop recursion if all tiles are placed
                    if new_x > pic.side_len {
                        return true;
                    } else if solve_rec((new_y, new_x), pic, starting_at) {
                        return true;
                    }
                }
            }

            // Try with next tile
            pic.tiles[y][x] = None;
            pic.used_tiles.remove(&new_potential_tile.id);
            tiles_to_skip += 1;
        } else {
            return false;
        }
    }
}
