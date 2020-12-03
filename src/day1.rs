use crate::day;
use crate::file;

pub struct Day1 {

}

impl day::Day for Day1 {

    fn puzzle1(&self) {
        println!("Day 1, puzzle 1");

        let v: Vec<u32> =
            file::lines("res/day1_1.txt")
                .iter()
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

        let v: Vec<u32> =
            file::lines("res/day1_1.txt")
                .iter()
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