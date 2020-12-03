
use std::io::{BufReader, BufRead};
use std::fs::File;

pub fn lines(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).unwrap();
    return BufReader::new(file)
        .lines()
        .filter_map(|line_result| line_result.ok())
        .collect();
}