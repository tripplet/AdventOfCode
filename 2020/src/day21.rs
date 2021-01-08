use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug)]
struct Food {
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}

impl FromStr for Food {
    type Err = ();

    fn from_str(line: &str) -> std::result::Result<Self, Self::Err> {
        let parts = line.trim().split(" (contains ").collect::<Vec<_>>();

        Ok(Food {
            ingredients: parts[0].trim().split(" ").map(|ingredient| ingredient.to_owned()).collect(),
            allergens: parts[1].trim().trim_end_matches(')')
                .split(",")
                .map(|alergen| alergen.trim().to_owned())
                .collect(),
        })
    }
}

fn main() {
    let food_list_example = parse(include_str!("../input/2020/day21_example.txt"));
    let food_list = parse(include_str!("../input/2020/day21.txt"));

    let result1 = part1(&food_list);

    dbg!(&food_list_example);
}

fn parse(input: &str) -> Vec<Food> {
    input.trim().lines().map(|line| Food::from_str(line).unwrap()).collect()
}

fn part1(food_list: &[Food]) -> usize {
    0
}
