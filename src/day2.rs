use crate::day;
use crate::file;

use lazy_static::lazy_static;
use regex::Regex;

pub struct Day2 {

}

impl day::Day for Day2 {

    fn puzzle1(&self) {
        println!("Day 2, puzzle 1");

        let num_valid =
            file::lines("res/day2_1.txt")
                .iter()
                .filter(|str| is_valid_1(str))
                .count();

        println!("{}", num_valid);
    }

    fn puzzle2(&self) {
        println!("Day 2, puzzle 2");

        let num_valid =
            file::lines("res/day2_1.txt")
                .iter()
                .filter(|str| is_valid_2(str))
                .count();

        println!("{}", num_valid);
    }

}

lazy_static! {
    static ref RE: Regex = Regex::new("(\\d+)-(\\d+)\\s*(\\w):\\s*(\\w+)").unwrap();
}

fn is_valid_1(str: &str) -> bool {
    let cap = RE.captures(str).unwrap();
    let low = cap[1].parse::<usize>().expect("Could not parse lower bound");
    let high = cap[2].parse::<usize>().expect("Could not parse higher bound");
    let letter = &cap[3];
    let pwd = &cap[4];

    let re2 = Regex::new(&format!("({})", letter)).unwrap();
    let instances = re2.captures_iter(pwd).count();

    return instances >= low && instances <= high;
}

fn is_valid_2(str: &str) -> bool {
    let cap = RE.captures(str).unwrap();
    let pos_1 = cap[1].parse::<usize>().expect("Could not parse pos 1");
    let pos_2 = cap[2].parse::<usize>().expect("Could not parse pos 2");
    let letter = &cap[3];
    let pwd = &cap[4];

    let w1 = &pwd[pos_1-1..pos_1];
    let w2 = &pwd[pos_2-1..pos_2];

    return (w1 == letter) != (w2 == letter);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_2() {
        assert_eq!(is_valid_2("1-3 a: abcde") , true);
        assert_eq!(is_valid_2("1-3 b: cdefg") , false);
        assert_eq!(is_valid_2("2-9 c: ccccccccc") , false);
    }

}