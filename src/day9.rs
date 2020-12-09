use crate::day;
use crate::file;

use lazy_static::lazy_static;
use regex::Regex;

pub struct Day9 {

}

impl day::Day for Day9 {

    fn puzzle1(&self) {
        println!("Day 9, puzzle 1");

        let numbers = build_numbers(&file::lines("res/day9_1.txt"));
        let result = find_invalid_number(&numbers, 25);

        println!("{}", result);
    }

    fn puzzle2(&self) {
        println!("Day 9, puzzle 2");

        let numbers = build_numbers(&file::lines("res/day9_1.txt"));
        let invalid_number = find_invalid_number(&numbers, 25);
        let result = find_encryption_weakness(&numbers, invalid_number);

        println!("{}", result);
    }

}


fn build_numbers(input: &Vec<String>) -> Vec<u64> {
    input
        .iter()
        .map(|s| build_number(s))
        .collect()
}

lazy_static! {
    static ref RE: Regex = Regex::new("^([[:digit:]]+)$").unwrap();
}

fn build_number(n: &str) -> u64 {
    let cap = RE.captures(n).unwrap();
    cap[1].parse::<u64>().expect("Could not parse number")
}


fn find_invalid_number(input: &Vec<u64>, preamble: usize) -> u64 {
    for i in preamble..input.len() {
        let mut found = false;
        'outer: for x in i-preamble..i-1 {
            for y in x+1..i {
                if input[x] + input[y] == input[i] {
                    found = true;
                    break 'outer;
                }
            }
        }
        if !found {
            return input[i];
        }
    }
    0
}

fn find_encryption_weakness(input: &Vec<u64>, invalid_number: u64) -> u64 {
    let mut smallest: u64 = 0;
    let mut biggest: u64 = 0;
    'outer: for i in 0..input.len() {
        let mut sum: u64 = input[i];
        smallest = input[i];
        biggest = input[i];
        for j in i+1..input.len() {
            sum = sum + input[j];
            if smallest > input[j] {
                smallest = input[j]
            }
            if biggest < input[j] {
                biggest = input[j]
            }
            if sum == invalid_number {
                break 'outer;
            }
        }
        smallest = 0;
        biggest = 0;
    }
    smallest + biggest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_number() {
        let numbers = vec![
            35,
            20,
            15,
            25,
            47,
            40,
            62,
            55,
            65,
            95,
            102,
            117,
            150,
            182,
            127,
            219,
            299,
            277,
            309,
            576
        ];
        assert_eq!(find_invalid_number(&numbers, 5), 127);
    }

    #[test]
    fn test_find_encryption_weakness() {
        let numbers = vec![
            35,
            20,
            15,
            25,
            47,
            40,
            62,
            55,
            65,
            95,
            102,
            117,
            150,
            182,
            127,
            219,
            299,
            277,
            309,
            576
        ];
        assert_eq!(find_encryption_weakness(&numbers, 127), 62);
    }

}