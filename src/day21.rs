use crate::day;
use crate::file;
use std::collections::{HashMap, HashSet};

use lazy_static::lazy_static;
use regex::Regex;

type Allergen = String;
type Food = String;

pub struct Day21 {

}

impl day::Day for Day21 {

    fn puzzle1(&self) {
        println!("Day 21, puzzle 1");

        let result = get_result_1(&file::lines("res/day21_1.txt"));

        println!("{}", result);
    }

    fn puzzle2(&self) {
        println!("Day 21, puzzle 2");

        let result = get_result_2(&file::lines("res/day21_1.txt"));

        println!("{}", result);
    }

}

fn get_result_1(input: &[String]) -> usize {
    let (all_foods, allergen_map) = parse_input(&input);

    let allergen_foods: HashSet<Food> =
        allergen_map
            .values()
            .flat_map(|h| h.iter())
            .map(|f| f.clone())
            .collect();

    let mut result = 0;
    for food in &all_foods {
        if !allergen_foods.contains(food) {
            result += 1;
        }
    }

    result
}

fn get_result_2(input: &[String]) -> String {
    let (_all_foods, allergen_map) = parse_input(&input);

    let mut allergens: Vec<Allergen> =
        allergen_map
            .keys()
            .map(|a| a.clone())
            .collect();
    allergens.sort_by(|a1, a2| {
        a1.cmp(a2)
    });

    let mut result = String::new();

    for allergen in &allergens {
        if !result.is_empty() {
            result.push_str(",");
        }
        result.push_str(allergen_map.get(allergen).unwrap().iter().next().unwrap());
    }

    result
}

/// Parse the input and return a list of all foods and a map of allergen to possible foods
fn parse_input(input: &[String]) -> (Vec<Food>, HashMap<Allergen, HashSet<Food>>) {
    let mut allergen_map: HashMap<Allergen, HashSet<Food>> = HashMap::new();
    let mut all_foods: Vec<Food> = Vec::new();

    for line in input {
        let foods = parse_foods(line);
        let allergens = parse_allergens(line);
        for allergen in &allergens {
            allergen_map
                .entry(allergen.clone())
                .and_modify(|f| {
                    *f = f.intersection(&foods).map(|food| food.clone()).collect()
                })
                .or_insert(foods.iter().map(|food| food.clone()).collect());
        }
        for food in &foods {
            all_foods.push(food.clone());
        }
    }

    let mut not_cleared_set: HashSet<Allergen> = HashSet::new();
    let mut all_allergens: HashSet<Allergen> = HashSet::new();
    for allergen in allergen_map.keys() {
        not_cleared_set.insert(allergen.clone());
        all_allergens.insert(allergen.clone());
    }

    let mut changed = true;
    while changed && not_cleared_set.len() != 0 {
        changed = false;
        let not_cleared_allergens: HashSet<Allergen> = not_cleared_set.iter().map(|a| a.clone()).collect();

        for allergen in &not_cleared_allergens {
            let mut remove_from_others: HashSet<Food> = HashSet::new();

            let foods = allergen_map.get(allergen).unwrap();
            if foods.len() == 0 {
                panic!("Allergen {} has no foods!", allergen);
            } else if foods.len() == 1 {
                not_cleared_set.remove(allergen);
                remove_from_others.insert(foods.iter().next().unwrap().clone());
                changed = true;
            }
            for food in &remove_from_others {
                for a in &all_allergens {
                    if a != allergen {
                        allergen_map
                            .entry(a.clone())
                            .and_modify(|f| {
                                f.remove(food);
                            });
                    }
                }
            }
        }
    }

    (all_foods, allergen_map)
}

lazy_static! {
    static ref RE_FOOD: Regex = Regex::new("([[:alpha:]]+)").unwrap();
    static ref RE_ALLERGEN: Regex = Regex::new("([[:alpha:]]+)").unwrap();
}

fn parse_foods(input: &String) -> HashSet<Food> {
    let e_idx = input.find("(").unwrap();
    RE_FOOD
        .captures_iter(&input[0..e_idx])
        .map(|food| {
            food[1].to_string()
        })
        .collect()
}

fn parse_allergens(input: &String) -> HashSet<Allergen> {
    let b_idx = input.find("(").unwrap() + 9;
    let e_idx = input.find(")").unwrap();
    RE_FOOD
        .captures_iter(&input[b_idx..e_idx])
        .map(|allergen| {
            allergen[1].to_string()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_result_1() {
        let input = vec![
            "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)".to_string(),
            "trh fvjkl sbzzf mxmxvkd (contains dairy)".to_string(),
            "sqjhc fvjkl (contains soy)".to_string(),
            "sqjhc mxmxvkd sbzzf (contains fish)".to_string(),
        ];
        assert_eq!(get_result_1(&input), 5);
    }

    #[test]
    fn test_get_result_2() {
        let input = vec![
            "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)".to_string(),
            "trh fvjkl sbzzf mxmxvkd (contains dairy)".to_string(),
            "sqjhc fvjkl (contains soy)".to_string(),
            "sqjhc mxmxvkd sbzzf (contains fish)".to_string(),
        ];
        assert_eq!(get_result_2(&input), "mxmxvkd,sqjhc,fvjkl".to_string());
    }

}