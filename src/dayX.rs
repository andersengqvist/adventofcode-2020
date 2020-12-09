use crate::day;
use crate::file;

pub struct DayX {

}

impl day::Day for DayX {

    fn puzzle1(&self) {
        println!("Day X, puzzle 1");

        let result = get_result_1(&file::lines("res/dayX_1.txt"));

        println!("{}", accumulated);
    }

    fn puzzle2(&self) {
        println!("Day X, puzzle 2");

        let result = get_result_2(&file::lines("res/dayX_1.txt"));

        println!("{}", result);
    }

}

fn get_result_1(input: &Vec<String>) -> i32 {
    1
}

fn get_result_2(input: &Vec<String>) -> i32 {
    2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_result_1() {
        assert_eq!(get_result_1(&vec![]), 1);
    }

    #[test]
    fn test_get_result_2() {
        assert_eq!(get_result_2(&vec![]), 2);
    }

}