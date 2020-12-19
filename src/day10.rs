use crate::day;
use crate::file;

use lazy_static::lazy_static;
use regex::Regex;

pub struct Day10 {

}

impl day::Day for Day10 {

    fn puzzle1(&self) {
        println!("Day 10, puzzle 1");

        let numbers = build_numbers(&file::lines("res/day10_1.txt"));
        let result = get_result_1(&numbers);

        println!("{}", result);
    }

    fn puzzle2(&self) {
        println!("Day 10, puzzle 2");

        let numbers = build_numbers(&file::lines("res/day10_1.txt"));
        let result = get_result_2(&numbers);

        println!("{}", result);
    }

}

fn build_numbers(input: &Vec<String>) -> Vec<usize> {
    input
        .iter()
        .map(|s| build_number(s))
        .collect()
}

lazy_static! {
    static ref RE: Regex = Regex::new("^([[:digit:]]+)$").unwrap();
}

fn build_number(n: &str) -> usize {
    let cap = RE.captures(n).unwrap();
    cap[1].parse::<usize>().expect("Could not parse number")
}

fn get_result_1(input: &Vec<usize>) -> u32 {

    let mut adapters = input.to_vec();

    adapters.sort();

    let mut one_diffs: u32 = 0;
    let mut three_diffs: u32 = 0;

    let mut prev: usize = 0;
    for adapter in adapters {
        let diff = adapter - prev;
        if diff == 1 {
            one_diffs += 1;
        } else if diff == 3 {
            three_diffs += 1;
        }
        prev = adapter;
    }

    three_diffs += 1; // Add one for the built in adapter

    one_diffs * three_diffs
}

fn get_result_2(input: &Vec<usize>) -> u64 {

    let mut adapters = input.to_vec();
    adapters.push(0); // The outlet
    adapters.sort();
    adapters.push(adapters[adapters.len()-1] + 3); // Add the built in so we don't try to remove the last adapter

    let mut begin_idx: usize = find_begin(&adapters, 0);
    let mut end_idx: usize = find_end(&adapters, begin_idx);

    let mut result: u64 = 1;
    while begin_idx < input.len() {
        //println!("Checking between index {} and {}", begin_idx, end_idx);
        let tmp = count_adapter_configurations(&adapters[begin_idx..end_idx+1], 0, 1);
        //println!("Found {} configurations", tmp);
        result *= tmp;
        begin_idx = find_begin(&adapters, end_idx);
        end_idx = find_end(&adapters, begin_idx);
    }

    result
}

fn find_begin(adapters: &[usize], idx: usize) -> usize {
    let mut prev_idx: usize = idx;
    for next_idx in idx+2..adapters.len() {
        if (adapters[next_idx] - adapters[prev_idx]) <= 3 {
            return prev_idx;
        }
        prev_idx += 1;
    }
    adapters.len()
}

fn find_end(adapters: &[usize], idx: usize) -> usize {
    let mut prev_idx: usize = idx;
    for next_idx in idx+2..adapters.len() {
        if (adapters[next_idx] - adapters[prev_idx]) > 3 {
            return prev_idx + 1;
        }
        prev_idx += 1;
    }
    adapters.len()
}

fn count_adapter_configurations(adapters: &[usize], s_idx: usize, c_idx: usize) -> u64 {
    let mut configurations: u64 = 1;

    let mut prev_idx: usize = s_idx;
    let mut curr_idx: usize = c_idx;
    for next_idx in c_idx+1..adapters.len() {
        if (adapters[next_idx] - adapters[prev_idx]) <= 3 {
            // Can remove current
            configurations += count_adapter_configurations(&adapters, prev_idx, next_idx);
        }
        prev_idx = curr_idx;
        curr_idx = next_idx;
    }
    configurations
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_result_1_1() {
        let numbers = vec![
            16,
            10,
            15,
            5,
            1,
            11,
            7,
            19,
            6,
            12,
            4
        ];
        assert_eq!(get_result_1(&numbers), 35);
    }

    #[test]
    fn test_get_result_1_2() {
        let numbers = vec![
            28,
            33,
            18,
            42,
            31,
            14,
            46,
            20,
            48,
            47,
            24,
            23,
            49,
            45,
            19,
            38,
            39,
            11,
            1,
            32,
            25,
            35,
            8,
            17,
            7,
            9,
            4,
            2,
            34,
            10,
            3
        ];
        assert_eq!(get_result_1(&numbers), 220);
    }

    #[test]
    fn test_get_result_2_1() {
        let numbers = vec![
            16,
            10,
            15,
            5,
            1,
            11,
            7,
            19,
            6,
            12,
            4
        ];
        assert_eq!(get_result_2(&numbers), 8);
    }

    #[test]
    fn test_get_result_2_2() {
        let numbers = vec![
            28,
            33,
            18,
            42,
            31,
            14,
            46,
            20,
            48,
            47,
            24,
            23,
            49,
            45,
            19,
            38,
            39,
            11,
            1,
            32,
            25,
            35,
            8,
            17,
            7,
            9,
            4,
            2,
            34,
            10,
            3
        ];
        assert_eq!(get_result_2(&numbers), 19208);
    }

}