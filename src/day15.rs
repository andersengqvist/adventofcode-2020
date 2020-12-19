use crate::day;

use std::collections::HashMap;
use std::borrow::BorrowMut;

pub struct Day15 {

}

impl day::Day for Day15 {

    fn puzzle1(&self) {
        println!("Day 15, puzzle 1");

        let result = get_result_1();

        println!("{}", result);
    }

    fn puzzle2(&self) {
        println!("Day 15, puzzle 2");

        let result = get_result_2();

        println!("{}", result);
    }

}

fn get_result_1() -> u64 {
    play_game(&vec![2,20,0,4,1,17], 2020)
}

fn get_result_2() -> u64 {
    play_game(&vec![2,20,0,4,1,17], 30000000)
}

fn play_game(starting_numbers: &Vec<u64>, spoken_numbers: usize) -> u64 {
    let mut map: HashMap<u64,(usize, Option<usize>)> = HashMap::new();

    let mut turn = 1;
    // The last turn: (value, turn, previous turn the value was spoken)
    let mut prev: (u64, usize, Option<usize>) = (0, 0, Option::None);

    for &n in starting_numbers {
        prev = add(map.borrow_mut(), n, turn);
        turn = turn + 1;
    }

    while turn <= spoken_numbers {
        match prev.2 {
            Some(previous_spoken_at_turn) => {
                let n = (prev.1 - previous_spoken_at_turn) as u64;
                prev = add(map.borrow_mut(), n, turn);
            }
            None => {
                //println!("Found {} at index {} and no more: Setting number {} at index {}", prev, i, 0, turn);
                prev = add(map.borrow_mut(), 0, turn);
            }
        }
        turn = turn + 1;
    }
    prev.0
}

// Connect the number with the turns
// The map is { number -> (turn, previous_turn) }
// Returns (number, turn, previous turn)
fn add(map: &mut HashMap<u64,(usize, Option<usize>)>, number: u64, turn: usize) -> (u64, usize, Option<usize>) {
    let curr = map.get(&number);
    let mut prev_turn = Option::None;
    match curr {
        Some((i, _)) => {
            prev_turn = Option::Some(*i);
        },
        None => {
        }
    }
    map.insert(number, (turn, prev_turn));
    (number, turn, prev_turn)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_play_game_1() {
        assert_eq!(play_game(&vec![0,3,6], 2020), 436);
        assert_eq!(play_game(&vec![1,3,2], 2020), 1);
        assert_eq!(play_game(&vec![2,1,3], 2020), 10);
        assert_eq!(play_game(&vec![1,2,3], 2020), 27);
        assert_eq!(play_game(&vec![2,3,1], 2020), 78);
        assert_eq!(play_game(&vec![3,2,1], 2020), 438);
        assert_eq!(play_game(&vec![3,1,2], 2020), 1836);
    }

    #[test]
    fn test_play_game_2() {
        assert_eq!(play_game(&vec![0,3,6], 30000000), 175594);
        assert_eq!(play_game(&vec![1,3,2], 30000000), 2578);
        assert_eq!(play_game(&vec![2,1,3], 30000000), 3544142);
        assert_eq!(play_game(&vec![1,2,3], 30000000), 261214);
        assert_eq!(play_game(&vec![2,3,1], 30000000), 6895259);
        assert_eq!(play_game(&vec![3,2,1], 30000000), 18);
        assert_eq!(play_game(&vec![3,1,2], 30000000), 362);
    }

}