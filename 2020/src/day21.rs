use std::collections::{HashMap, HashSet};

use colored::Colorize;

#[derive(Debug, Clone)]
struct Food<'a> {
    ingredients: HashSet<&'a str>,
    allergens: HashSet<&'a str>,
}

impl<'a> Food<'a> {
    fn from_str(line: &'a str) -> std::result::Result<Self, ()> {
        let parts = line.trim().split(" (contains ").collect::<Vec<_>>();

        Ok(Food {
            ingredients: parts[0].trim().split(" ").map(|ingredient| ingredient).collect(),
            allergens: parts[1]
                .trim()
                .trim_end_matches(')')
                .split(",")
                .map(|alergen| alergen.trim())
                .collect(),
        })
    }
}

fn main() {
    let food_list_example = parse(include_str!("../input/2020/day21_example.txt"));
    let food_list = parse(include_str!("../input/2020/day21.txt"));

    assert_eq!(part1(&food_list_example), 5);

    let now = std::time::Instant::now();
    let result1 = part1(&food_list);
    println!( "Part1: {}  [{}]", result1.to_string().yellow(), humantime::format_duration(now.elapsed()).to_string().blue());
    assert_eq!(result1, 1977);

    assert_eq!(part2(&food_list_example), "mxmxvkd,sqjhc,fvjkl");

    let now = std::time::Instant::now();
    let result2 = part2(&food_list);
    println!( "Part2: {}  [{}]", result2.to_string().yellow(), humantime::format_duration(now.elapsed()).to_string().blue());
    assert_eq!(result2, "dpkvsdk,xmmpt,cxjqxbt,drbq,zmzq,mnrjrf,kjgl,rkcpxs");
}

fn parse(input: &str) -> Vec<Food> {
    input.trim().lines().map(|line| Food::from_str(line).unwrap()).collect()
}

fn get_safe_ingredients<'a>(food_list: &'a [Food]) -> Vec<&'a str> {
    let allergen_dict = build_allergen_dict(food_list);

    let all_ingredients = food_list.into_iter().fold(HashSet::default(), |all, cur| {
        all.union(&cur.ingredients).cloned().collect()
    });

    all_ingredients
        .into_iter()
        .filter(|ingredient| {
            !allergen_dict
                .values()
                .any(|ingredient_with_allergen| ingredient_with_allergen == ingredient)
        })
        .collect::<Vec<_>>()
}

fn build_allergen_dict<'a>(food_list: &'a [Food]) -> HashMap<&'a str, &'a str> {
    let all_allergens = food_list.iter().fold(HashSet::default(), |all, cur| {
        all.union(&cur.allergens).cloned().collect()
    });

    let mut allergen_dict = all_allergens
        .into_iter()
        .map(|cur_allergen| {
            let all_foods_with_this_allergen = food_list
                .iter()
                .filter(|f| f.allergens.contains(cur_allergen))
                .collect::<Vec<_>>();

            let common_ingredients = all_foods_with_this_allergen
                .iter()
                .skip(1)
                .fold(all_foods_with_this_allergen[0].ingredients.clone(), |all, cur| {
                    all.intersection(&cur.ingredients).cloned().collect()
                });
            (cur_allergen, common_ingredients)
        })
        .collect::<HashMap<_, _>>();

    while allergen_dict.values().any(|ingredients| ingredients.len() > 1) {
        let finished = allergen_dict
            .iter()
            .filter_map(|(allergen, ingredients)| {
                if ingredients.len() == 1 {
                    Some((allergen.to_owned(), ingredients.to_owned()))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        for allergen in finished {
            allergen_dict
                .iter_mut()
                .filter(|(k, _)| **k != allergen.0)
                .for_each(|(_, v)| *v = v.difference(&allergen.1).cloned().collect());
        }
    }

    allergen_dict
        .into_iter()
        .map(|(k, v)| (k, *v.iter().next().unwrap()))
        .collect::<HashMap<_, _>>()
}

fn part1(food_list: &[Food]) -> usize {
    get_safe_ingredients(food_list)
        .into_iter()
        .map(|safe_ingredient| {
            food_list
                .iter()
                .filter(|food| food.ingredients.contains(safe_ingredient))
                .count()
        })
        .sum()
}

fn part2(food_list: &[Food]) -> String {
    let mut dangerous_ingredients = build_allergen_dict(food_list)
        .iter().to_owned()
        .map(|(&allergen, &ingredient)| (allergen, ingredient))
        .collect::<Vec<_>>();

    dangerous_ingredients.sort_unstable_by_key(|(allergen, _)| allergen.clone());

    dangerous_ingredients
        .into_iter()
        .map(|(_, ingredient)| ingredient)
        .collect::<Vec<_>>()
        .join(",")
        .to_string()
}
