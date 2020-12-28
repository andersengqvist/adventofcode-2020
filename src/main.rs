
use std::env;

use crate::day::Day;

mod file;

mod day;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day23;
mod day24;

fn main() {
    println!("Advent of Code 2020");

    let args: Vec<String> = env::args().collect();

    let day_select = &args[1];
    //let puzzleSelect = &args[2];

    let day_opt = get_day(day_select);

    if day_opt.is_none() {
        // Error
    } else {
        let day = day_opt.unwrap();
        day.puzzle1();
        day.puzzle2()
    }

}

fn get_day (day_select:&str) -> Option<Box<dyn Day>> {
    match day_select {
        "day1" => Some(Box::new(day1::Day1 {})),
        "day2" => Some(Box::new(day2::Day2 {})),
        "day3" => Some(Box::new(day3::Day3 {})),
        "day4" => Some(Box::new(day4::Day4 {})),
        "day5" => Some(Box::new(day5::Day5 {})),
        "day6" => Some(Box::new(day6::Day6 {})),
        "day7" => Some(Box::new(day7::Day7 {})),
        "day8" => Some(Box::new(day8::Day8 {})),
        "day9" => Some(Box::new(day9::Day9 {})),
        "day10" => Some(Box::new(day10::Day10 {})),
        "day11" => Some(Box::new(day11::Day11 {})),
        "day12" => Some(Box::new(day12::Day12 {})),
        "day13" => Some(Box::new(day13::Day13 {})),
        "day14" => Some(Box::new(day14::Day14 {})),
        "day15" => Some(Box::new(day15::Day15 {})),
        "day16" => Some(Box::new(day16::Day16 {})),
        "day17" => Some(Box::new(day17::Day17 {})),
        "day18" => Some(Box::new(day18::Day18 {})),
        "day19" => Some(Box::new(day19::Day19 {})),
        "day23" => Some(Box::new(day23::Day23 {})),
        "day24" => Some(Box::new(day24::Day24 {})),
        _ => None,
    }
}