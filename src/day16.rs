use crate::day;
use crate::file;
use std::collections::{HashMap, HashSet};

use lazy_static::lazy_static;
use regex::Regex;
use std::borrow::BorrowMut;

pub struct Day16 {

}

impl day::Day for Day16 {

    fn puzzle1(&self) {
        println!("Day 16, puzzle 1");

        let result = get_result_1(&file::lines("res/day16_1.txt"));

        println!("{}", result);
    }

    fn puzzle2(&self) {
        println!("Day 16, puzzle 2");

        let result = get_result_2(&file::lines("res/day16_1.txt"));

        println!("{}", result);
    }

}

fn get_result_1(input: &[String]) -> u64 {
    let (rules_input, _, tickets_input) = split_input(input);
    let rules = build_rules(&rules_input);
    let tickets = build_tickets(&tickets_input);

    tickets
        .iter()
        .flat_map(|t| get_invalid_numbers(t, &rules))
        .sum()
}

fn get_result_2(input: &[String]) -> u64 {
    let (rules_input, my_ticket_input, tickets_input) = split_input(input);
    let rules = build_rules(&rules_input);
    let my_ticket = build_ticket(&my_ticket_input);
    let tickets = build_tickets(&tickets_input);
    let filtered_tickets = tickets
        .iter()
        .filter(|t| is_valid_ticket(*t, &rules))
        .collect::<Vec<&Ticket>>();

    let candidates = get_field_candidates_for_tickets(&filtered_tickets, &rules);
    let filtered_candidates = filter_candidates(&candidates);

    let mut result = 1;
    for i in 0..filtered_candidates.len() {
        let hs = &filtered_candidates[i];
        if hs.len() == 1 {
            let s = hs.iter().next().unwrap().clone();
            if s.starts_with("departure") {
                result *= my_ticket.fields[i];
            }
        } else {
            panic!("Could not determine fields on ticket");
        }
    }
    result
}

fn split_input(input: &[String]) -> (&[String], String, &[String]) {
    for i in 0..input.len() {
        if input[i].is_empty() {
            return (&input[0..i], input[i+2].clone(), &input[i+5..]);
        }
    }
    panic!("Empty line not found");
}

struct ClosedInterval {
    begin: u64,
    end: u64
}

impl ClosedInterval {

    fn new(begin: u64, end: u64) -> ClosedInterval {
        ClosedInterval { begin, end }
    }

    fn contains(&self, i: u64) -> bool {
        i >= self.begin && i <= self.end
    }
}

struct Rule {
    name: String,
    ranges: Vec<ClosedInterval>
}

impl Rule {

    fn new(name: &str, ranges: Vec<ClosedInterval>) -> Rule {
        Rule { name: name.to_string(), ranges }
    }

    fn matches(&self, i: u64) -> bool {
        self.ranges.iter().any(|interval| interval.contains(i))
    }
}

struct Rules {
    rules: HashMap<String, Rule>
}

impl Rules {

    fn new(rules: HashMap<String, Rule>) -> Rules {
        Rules { rules }
    }

    fn any_match(&self, i: u64) -> bool {
        self.rules.values().any(|rule| rule.matches(i))
    }

    fn get_matching_rule_names(&self, i: u64) -> HashSet<String> {
        self
            .rules
            .values()
            .filter(|rule| rule.matches(i))
            .map(|rule| rule.name.clone())
            .collect()
    }
}

lazy_static! {
    static ref RE_INTERVAL: Regex = Regex::new("(\\d+)-(\\d+)").unwrap();
}

fn build_rules(input: &[String]) -> Rules {
    let mut rules_map: HashMap<String, Rule> = HashMap::new();

    for s in input {
        let c_idx = s.find(":").unwrap();
        let name = &s[0..c_idx];

        let ranges: Vec<ClosedInterval> = RE_INTERVAL
            .captures_iter(&s[c_idx..])
            .map(|cap_inner| {
                let begin = cap_inner[1].parse::<u64>().expect("Could not parse begin");
                let end = cap_inner[2].parse::<u64>().expect("Could not parse end");
                ClosedInterval::new(begin, end)
            })
            .collect();
        let rule = Rule::new(name, ranges);
        rules_map.insert(name.to_string(), rule);
    }

    Rules::new(rules_map)
}

struct Ticket {
    fields: Vec<u64>
}

impl Ticket {
    fn new(fields: Vec<u64>) -> Ticket {
        Ticket { fields }
    }
}

fn build_ticket(input: &String) -> Ticket {
    let v = input
        .split(',')
        .map(|i| i.parse::<u64>().expect("Could not parse begin"))
        .collect::<Vec<u64>>();
    Ticket::new(v)
}

fn build_tickets(input: &[String]) -> Vec<Ticket> {
    input
        .iter()
        .map(|s| {
            let v = s
                .split(',')
                .map(|i| i.parse::<u64>().expect("Could not parse begin"))
                .collect::<Vec<u64>>();
            Ticket::new(v)
        })
        .collect()
}

fn get_invalid_numbers(ticket: &Ticket, rules: &Rules) -> Vec<u64> {
    ticket
        .fields
        .iter()
        .filter_map(|&n| {
            if rules.any_match(n) {
                Option::None
            } else {
                Option::Some(n)
            }
        })
        .collect()
}

fn is_valid_ticket(ticket: &Ticket, rules: &Rules) -> bool {
    !ticket
        .fields
        .iter()
        .any(|&n| !rules.any_match(n))
}

fn get_field_candidates_for_ticket(ticket: &Ticket, rules: &Rules) -> Vec<HashSet<String>> {
    let m = ticket
        .fields
        .iter()
        .map(|&i| {
            rules.get_matching_rule_names(i)
        })
        .collect();
    m
}

fn get_field_candidates_for_tickets(tickets: &[&Ticket], rules: &Rules) -> Vec<HashSet<String>> {
    let a = tickets
        .iter()
        .map(|ticket| {
            get_field_candidates_for_ticket(ticket, rules)
        })
        .fold(
            Option::None,
            |h1, h2| {
                if h1.is_none() {
                    Option::Some(h2)
                } else {
                    Option::Some(merge_candidates(&h1.unwrap(), &h2))
                }
            }
        );
    a.unwrap()
}

fn merge_candidates(h1: &[HashSet<String>], h2: &[HashSet<String>]) -> Vec<HashSet<String>> {
    h1.iter()
        .zip(h2)
        .map(|(s1, s2)| {
            s1.intersection(s2).map(|s| s.clone()).collect::<HashSet<String>>()
        })
        .collect()
}

fn filter_candidates(h: &[HashSet<String>]) -> Vec<HashSet<String>> {
    let mut result: Vec<HashSet<String>> = vec![];
    let mut cleared: Vec<bool> = vec![];

    for i in 0..h.len() {
        let mut hs = HashSet::new();
        for val in &h[i] {
            hs.insert(val.clone());
        }
        result.push(hs);
        cleared.push(false);
    }

    let mut changed = true;
    while changed && !cleared.iter().all(|&v| v) {
        changed = false;
        for i in 0..h.len() {
            if !cleared[i] {
                if result[i].len() == 1 {
                    cleared[i] = true;
                    changed = true;
                    let s = result[i].iter().next().unwrap().clone();
                    remove_others(result.borrow_mut(), &s, i);
                } else {
                    let mut the_string = Option::None;
                    for st in result[i].iter() {
                        if count_rules(&result, st) == 1 {
                            cleared[i] = true;
                            changed = true;
                            let s = st.clone();
                            the_string = Option::Some(s);
                            break;
                        }
                    }
                    if the_string.is_some() {
                        let s = the_string.unwrap();
                        let hs = &mut result[i];
                        hs.clear();
                        hs.insert(s);
                    }
                }
            }
        }
    }
    result
}

fn remove_others(h: &mut [HashSet<String>], s: &String, idx: usize) {
    for i in 0..h.len() {
        if i != idx {
            h[i].remove(s);
        }
    }
}

fn count_rules(h: &[HashSet<String>], s: &String) -> usize {
    let mut result = 0;
    for hs in h {
        if hs.contains(s) {
            result += 1;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_result_1() {
        let input = vec![
            "class: 1-3 or 5-7".to_string(),
            "row: 6-11 or 33-44".to_string(),
            "seat: 13-40 or 45-50".to_string(),
            "".to_string(),
            "your ticket:".to_string(),
            "7,1,14".to_string(),
            "".to_string(),
            "nearby tickets:".to_string(),
            "7,3,47".to_string(),
            "40,4,50".to_string(),
            "55,2,20".to_string(),
            "38,6,12".to_string()
        ];
        assert_eq!(get_result_1(&input), 71);
    }

    #[test]
    fn test_sum_invalid() {
        let rules_input = vec![
            "class: 1-3 or 5-7".to_string(),
            "row: 6-11 or 33-44".to_string(),
            "seat: 13-40 or 45-50".to_string()
        ];
        let tickets_input = vec![
            "7,3,47".to_string(),
            "40,4,50".to_string(),
            "55,2,20".to_string(),
            "38,6,12".to_string()
        ];
        let rules = build_rules(&rules_input);
        let tickets = build_tickets(&tickets_input);

        let sum: u64 = tickets
            .iter()
            .flat_map(|t| get_invalid_numbers(t, &rules))
            .sum();
        assert_eq!(sum, 71);
    }

    #[test]
    fn test_filter_candidates() {
        let rules_input = vec![
            "class: 1-3 or 5-7".to_string(),
            "row: 6-11 or 33-44".to_string(),
            "seat: 13-40 or 45-50".to_string()
        ];
        let tickets_input = vec![
            "7,3,47".to_string(),
            "40,4,50".to_string(),
            "55,2,20".to_string(),
            "38,6,12".to_string()
        ];
        let rules = build_rules(&rules_input);
        let tickets = build_tickets(&tickets_input);

        let filtered_tickets = tickets
            .iter()
            .filter(|t| is_valid_ticket(*t, &rules))
            .collect::<Vec<&Ticket>>();

        let candidates = get_field_candidates_for_tickets(&filtered_tickets, &rules);
        let filtered_candidates = filter_candidates(&candidates);

        let mut h0 = HashSet::new();
        h0.insert("row".to_string());
        let mut h1 = HashSet::new();
        h1.insert("class".to_string());
        let mut h2 = HashSet::new();
        h2.insert("seat".to_string());
        assert_eq!(filtered_candidates, vec![h0, h1, h2]);
    }

}