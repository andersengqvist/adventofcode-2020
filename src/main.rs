
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
        _ => None,
    }
}