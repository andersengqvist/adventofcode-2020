use crate::day;
use crate::file;
use std::collections::HashSet;

use lazy_static::lazy_static;
use regex::Regex;

pub struct Day7 {

}

impl day::Day for Day7 {

    fn puzzle1(&self) {
        println!("Day 7, puzzle 1");

        let bag_rules = build_rules_from_strs(&file::lines("res/day7_1.txt"));
        let containers = count_shiny_gold_bag_carriers(&bag_rules);

        println!("{}", containers);
    }

    fn puzzle2(&self) {
        println!("Day 7, puzzle 2");

        let bag_rules = build_rules_from_strs(&file::lines("res/day7_1.txt"));
        let containers = count_bags_inside_shiny_gold_bag(&bag_rules);

        println!("{}", containers);
    }

}

pub struct BagRule {
    outer: String,
    inner: String,
    amount: u32
}

impl BagRule {

    fn new(outer: &str, inner: &str, amount: u32) -> BagRule {
        BagRule { outer: outer.to_string(), inner: inner.to_string(), amount: amount }
    }

}

lazy_static! {
    static ref RE_OUTER: Regex = Regex::new("([[:alnum:]]+\\s+[[:alnum:]]+) bags").unwrap();
    static ref RE_INNER: Regex = Regex::new("(\\d+)\\s+([[:alnum:]]+\\s+[[:alnum:]]+) bag").unwrap();
}

fn build_rules_from_str(s: &str) -> Vec<BagRule> {
    let cap_outer = RE_OUTER.captures(s).unwrap();
    let outer_name = &cap_outer[1];
    RE_INNER
        .captures_iter(s)
        .map(|cap_inner| {
            let amount = cap_inner[1].parse::<u32>().expect("Could not number");
            let inner_name = &cap_inner[2];
            BagRule::new(outer_name, inner_name, amount)
        })
        .collect()
}

fn build_rules_from_strs(l: &Vec<String>) -> Vec<BagRule> {
    l
        .iter()
        .flat_map( |s| build_rules_from_str(s).into_iter())
        .collect()
}

fn count_shiny_gold_bag_carriers(bag_rules: &Vec<BagRule>) -> usize {

    let mut visited: HashSet<&str> = HashSet::new();
    let mut to_visit: Vec<&str> = Vec::new();

    to_visit.push(&"shiny gold");

    while !to_visit.is_empty() {
        let tmp: &str = to_visit.pop().unwrap();
        visited.insert(tmp);

        for bag_rule in bag_rules {
            if &bag_rule.inner == tmp {
                if !visited.contains(bag_rule.outer.as_str()) {
                    to_visit.push(bag_rule.outer.as_str());
                }
            }
        }

    }

    visited.len() - 1 // Need to remove 1, the shiny gold bag
}

fn count_bags_inside_shiny_gold_bag(bag_rules: &Vec<BagRule>) -> u32 {
    count_bags_inside_bag(bag_rules, "shiny gold") - 1
}

fn count_bags_inside_bag(bag_rules: &Vec<BagRule>, bag_name: &str) -> u32 {
    let mut tot_bags: u32 = 1;

    for bag_rule in bag_rules {
        if &bag_rule.outer == bag_name {
            let bags = bag_rule.amount * count_bags_inside_bag(bag_rules, bag_rule.inner.as_str());
            tot_bags += bags
        }
    }

    tot_bags
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_shiny_gold_bag_carriers() {
        let bag_rules = build_rules_from_strs(
            &vec![
                "light red bags contain 1 bright white bag, 2 muted yellow bags.".to_string(),
                "dark orange bags contain 3 bright white bags, 4 muted yellow bags.".to_string(),
                "bright white bags contain 1 shiny gold bag.".to_string(),
                "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.".to_string(),
                "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.".to_string(),
                "dark olive bags contain 3 faded blue bags, 4 dotted black bags.".to_string(),
                "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.".to_string(),
                "faded blue bags contain no other bags.".to_string(),
                "dotted black bags contain no other bags.".to_string(),
            ]
        );
        assert_eq!(count_shiny_gold_bag_carriers(&bag_rules), 4);
    }

    #[test]
    fn test_count_bags_inside_shiny_gold_bag_1() {
        let bag_rules = build_rules_from_strs(
            &vec![
                "light red bags contain 1 bright white bag, 2 muted yellow bags.".to_string(),
                "dark orange bags contain 3 bright white bags, 4 muted yellow bags.".to_string(),
                "bright white bags contain 1 shiny gold bag.".to_string(),
                "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.".to_string(),
                "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.".to_string(),
                "dark olive bags contain 3 faded blue bags, 4 dotted black bags.".to_string(),
                "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.".to_string(),
                "faded blue bags contain no other bags.".to_string(),
                "dotted black bags contain no other bags.".to_string(),
            ]
        );
        assert_eq!(count_bags_inside_shiny_gold_bag(&bag_rules), 32);
    }

    #[test]
    fn test_count_bags_inside_shiny_gold_bag_2() {
        let bag_rules = build_rules_from_strs(
            &vec![
                "shiny gold bags contain 2 dark red bags.".to_string(),
                "dark red bags contain 2 dark orange bags.".to_string(),
                "dark orange bags contain 2 dark yellow bags.".to_string(),
                "dark yellow bags contain 2 dark green bags.".to_string(),
                "dark green bags contain 2 dark blue bags.".to_string(),
                "dark blue bags contain 2 dark violet bags.".to_string(),
                "dark violet bags contain no other bags.".to_string()
            ]
        );
        assert_eq!(count_bags_inside_shiny_gold_bag(&bag_rules), 126);
    }

}