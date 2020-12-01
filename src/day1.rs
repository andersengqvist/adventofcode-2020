use crate::day;

use std::io::{BufReader, BufRead};
use std::fs::File;

pub struct Day1 {

}

impl day::Day for Day1 {

    fn puzzle1(&self) {
        println!("Day 1, puzzle 1");

        let file = File::open("res/day1_1.txt").unwrap();
        let v: Vec<u32> = BufReader::new(file)
            .lines()

            .filter_map(|line_result| line_result.ok())
            .filter_map(|line| line.parse::<u32>().ok())
            .collect();

        for i in 0..v.len() {
            for j in i+1..v.len() {
                if v[i] + v[j] == 2020 {
                    println!("{}", v[i] * v[j]);
                }
            }
        }

    }

    fn puzzle2(&self) {
        println!("Day 1, puzzle 2");

        let file = File::open("res/day1_1.txt").unwrap();

        let v: Vec<u32> = BufReader::new(file)
            .lines()
            .filter_map(|line_result| line_result.ok())
            .filter_map(|line| line.parse::<u32>().ok())
            .collect();

        for i in 0..v.len() {
            for j in i+1..v.len() {
                for k in j+1..v.len() {
                    if v[i] + v[j] + v[k] == 2020 {
                        println!("{}", v[i] * v[j] * v[k]);
                    }
                }
            }
        }

    }

}