//use rayon::prelude::*;
use colored::Colorize;
use std::borrow::Cow;
use std::collections::HashSet;

use ndarray::Array2;

const TOP: usize = 0;
const RIGHT: usize = 1;
const BOTTOM: usize = 2;
const LEFT: usize = 3;

struct SatPicture<'a, 'b> {
    unsorted_tiles: &'a Vec<Tile<'a>>,
    tiles: Vec<Vec<Option<Cow<'a, Tile<'a>>>>>,
    used_tiles: &'b mut HashSet<u16>,
    side_len: usize,
}

#[derive(Clone)]
struct Tile<'a> {
    id: u16,
    map: Cow<'a, Array2<char>>,
    edges: [u16; 4],
    variant: u8,
}

impl std::fmt::Display for Tile<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: [{}, {}, {}, {}]",
            self.id, self.edges[TOP], self.edges[RIGHT], self.edges[BOTTOM], self.edges[LEFT]
        )
    }
}

impl std::fmt::Debug for Tile<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

const SHIFT: u8 = 6; // u16 need shift down by 6 for tiles where edges are of size 10
fn mirrored(value: u16) -> u16 { value.reverse_bits() >> SHIFT }

const SEAMONSTER: &str = r####"                  #
#    ##    ##    ###
 #  #  #  #  #  #   "####;

impl<'a> Tile<'a> {
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
            map: Cow::Owned(Array2::from_shape_vec((map[0].len(),map.len()), map.iter().flatten().map(|c| *c).collect::<Vec<_>>()).unwrap()),
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

    fn rotate(&'a self) -> Self {
        Tile {
            edges: [mirrored(self.edges[LEFT]), self.edges[TOP], mirrored(self.edges[RIGHT]), self.edges[BOTTOM]],
            map: Cow::Borrowed(&self.map),
            ..*self
        }
    }

    fn mirror_vertical(&'a self) -> Self {
        Tile {
            edges: [mirrored(self.edges[TOP]), self.edges[LEFT], mirrored(self.edges[BOTTOM]), self.edges[RIGHT]],
            map: Cow::Borrowed(&self.map),
            ..*self
        }
    }

    fn mirror_horizontal(&'a self) -> Self {
        Tile {
            edges: [self.edges[BOTTOM], mirrored(self.edges[RIGHT]), self.edges[TOP], mirrored(self.edges[LEFT])],
            map: Cow::Borrowed(&self.map),
            ..*self
        }
    }

    fn variant(&'a self, var: u8) -> Self {
        // let mut new_variant = Cow::Owned(self);
        // let modulo_4 = var % 4;

        // if var >= 4 {
        //     new_variant = Cow::Owned(new_variant.rotate());
        // }

        // if modulo_4 == 1 || modulo_4 == 3 {
        //     new_variant = Cow::Owned(new_variant.mirror_horizontal());
        // }

        // if modulo_4 == 2 || modulo_4 == 3 {
        //     new_variant = Cow::Owned(new_variant.mirror_vertical());
        // }

        Tile { variant: var, map:  Cow::Borrowed(&self.map), ..*self }
    }
}

impl<'a> SatPicture<'a, '_> {
    fn next_unused_tile(&self, starting_at: usize) -> Option<&'a Tile> {
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

    dbg!(&tiles[0].map);

    println!(
        "Part1: {}  [{}]",
        part1_result.to_string().yellow(),
        humantime::format_duration(now.elapsed()).to_string().blue()
    );
    assert_eq!(part1_result, 11788777383197);
}

fn part1(tiles: &Vec<Tile>) -> usize {
    let side_len = (tiles.len() as f64).sqrt().round() as usize;

    // start at 1,1 easier bounds checking
    let mut pic = SatPicture {
        unsorted_tiles: tiles,
        tiles: vec![vec![None; side_len + 2]; side_len + 2],
        side_len: side_len,
        used_tiles: &mut HashSet::with_capacity(tiles.len())
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

fn solve_rec((y, x): (usize, usize), pic: &mut SatPicture, starting_at: usize) -> bool {
    let mut tiles_to_skip = starting_at;

    // loop over all potenital next tiles
    loop {
        let new_potential_tile = pic.next_unused_tile(tiles_to_skip);

        if let Some(new_potential_tile) = new_potential_tile {
            pic.used_tiles.insert(new_potential_tile.id);

            // Try all variants of the potential tile
            for var in 0..8 {
                let to_test: Cow<Tile> = Cow::Owned(new_potential_tile.variant(var));

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
                    }
                    else if solve_rec((new_y, new_x), pic, starting_at) {
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
    false
}
