
use std::env;

use crate::day::Day;

mod day;
mod day1;
mod day2;

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
        _ => None,
    }
}