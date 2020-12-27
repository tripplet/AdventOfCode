#[derive(Debug, Clone)]
struct Tile {
    id: u16,
    //map: Vec<Vec<char>>,
    edges: [u16; 4],
}

impl Tile {
    fn new(input: &str) -> Self {
        let parts = input.trim().split(":").collect::<Vec<_>>();

        fn from_binary(chars: &Vec<char>) -> u16 {
            u16::from_str_radix(chars.iter().collect::<String>().as_str(), 2).unwrap()
        }

        let map = parts[1]
            .trim()
            .lines()
            .map(|line| line.trim().chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let bin_map = map
            .iter()
            .map(|line| {
                line.iter()
                    .map(|c| if *c == '#' { '1' } else { '0' })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let top = from_binary(&bin_map[0]);
        let right = from_binary(&bin_map.iter().map(|line| *line.last().unwrap()).collect());
        let bottom = from_binary(&bin_map.last().unwrap());
        let left = from_binary(&bin_map.iter().map(|line| line[0]).collect());

        Tile {
            id: parts[0].split(" ").nth(1).unwrap().parse().unwrap(),
            //map: map,
            edges: [top, right, bottom, left],
        }
    }

    fn parse(input: &str) -> Vec<Self> {
        input
            .replace("\r", "")
            .split("\n\n")
            .map(|tile_str| Tile::new(tile_str))
            .collect()
    }
}

fn main() {
    let tiles = Tile::parse(include_str!("../input/2020/day20_example.txt"));

    let now = std::time::Instant::now();
    let part1_result = part1(&tiles);
    println!("Part1: {}  [{}]", part1_result, humantime::format_duration(now.elapsed()));
    //assert_eq!(part1_result, 33098);
}

fn part1(tiles: &Vec<Tile>) -> usize {
    for idx in 0..tiles.len() {
        let potential_corner = &tiles[idx];

        for idx2 in 0..tiles.len() {}
    }

    0
}
