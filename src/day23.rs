use crate::day;
use std::collections::HashMap;
use std::cell::RefCell;

pub struct Day23 {

}

impl day::Day for Day23 {

    fn puzzle1(&self) {
        println!("Day 23, puzzle 1");

        let result = get_result_1();

        println!("{}", result);
    }

    fn puzzle2(&self) {
        println!("Day 23, puzzle 2");

        let result = get_result_2();

        println!("{}", result);
    }

}

fn get_result_1() -> String {
    let mut cc = CrabCups::new(vec![5,8,9,1,7,4,2,6,3]);
    for _i in 1..101 {
        cc.do_move();
    }

    cc.get_cups_order()
}

fn get_result_2() -> u64 {
    let size: usize = 1000000;
    let mut cups: Vec<u64> = Vec::with_capacity(size);
    cups.append(vec![5,8,9,1,7,4,2,6,3].as_mut());
    for i in 10..size+1 {
        cups.push(i as u64);
    }

    let mut cc = CrabCups::new(cups);
    for _i in 1..10000001 {
        cc.do_move();
    }

    let (s1, s2) = cc.get_stars();

    s1 * s2
}

struct Cup {
    valuee: u64,
    next: u64
}

// Found a much neater solution by Neil Gall
// here https://dev.to/rpalo/advent-of-code-2020-solution-megathread-day-23-crab-cups-57k8
// Using only a vec where the index is the cup value and vec[value] is the next value
struct CrabCups {
    cups: HashMap<u64, RefCell<Cup>>,
    curr: u64,
    highest_value: u64,
    lowest_value: u64
}

impl CrabCups {
    fn new(cups: Vec<u64>) -> CrabCups {
        let mut cup_map: HashMap<u64, RefCell<Cup>> = HashMap::new();

        let mut highest = 0;
        let mut lowest = 10000000;

        for i in 0..cups.len() {
            if cups[i] > highest {
                highest = cups[i];
            }
            if cups[i] < lowest {
                lowest = cups[i];
            }
            let next_idx = if i + 1 == cups.len() { 0 } else { i + 1 };
            let cup = Cup{ valuee: cups[i], next: cups[next_idx] };
            cup_map.insert(cup.valuee, RefCell::new(cup));
        }

        CrabCups { cups: cup_map, curr: cups[0], highest_value: highest, lowest_value: lowest }
    }


    fn do_move(&mut self) {
        let curr_cup = self.cups.get(&self.curr).unwrap();

        let next1 = self.cups.get(&curr_cup.borrow().next).unwrap();
        let next2 = self.cups.get(&next1.borrow().next).unwrap();
        let next3 = self.cups.get(&next2.borrow().next).unwrap();

        let mut target_value = if self.curr == self.lowest_value { self.highest_value } else { self.curr - 1 };
        while next1.borrow().valuee == target_value
            || next2.borrow().valuee == target_value
            || next3.borrow().valuee == target_value {
            target_value = if target_value == self.lowest_value { self.highest_value } else { target_value - 1 };
        }

        let destination_cup = self.cups.get(&target_value).unwrap();

        curr_cup.borrow_mut().next = next3.borrow().next;
        let tmp_next = destination_cup.borrow().next;
        destination_cup.borrow_mut().next = next1.borrow().valuee;
        next3.borrow_mut().next = tmp_next;

        self.curr = self.cups.get(&self.curr).unwrap().borrow().next;
    }

    fn get_cups_order(&self) -> String {
        let mut result = String::new();

        let mut value = self.cups.get(&1).unwrap().borrow().next;

        while value != 1 {
            result.push_str(format!("{}", value).as_str());
            value = self.cups.get(&value).unwrap().borrow().next;
        }

        result
    }

    fn get_stars(&self) -> (u64, u64) {
        let v1 = self.cups.get(&1).unwrap().borrow().next;
        let v2 = self.cups.get(&v1).unwrap().borrow().next;
        (v1, v2)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_play_crabcups_10() {
        let mut cc = CrabCups::new(vec![3,8,9,1,2,5,4,6,7]);
        for _i in 1..11 {
            cc.do_move();
        }

        assert_eq!(cc.get_cups_order(), "92658374".to_string());
    }

    #[test]
    fn test_play_crabcups_100() {
        let mut cc = CrabCups::new(vec![3,8,9,1,2,5,4,6,7]);
        for _i in 1..101 {
            cc.do_move();
        }

        assert_eq!(cc.get_cups_order(), "67384529".to_string());
    }

    #[test]
    fn test_play_crabcups_1000000() {
        let size: usize = 1000000;
        let mut cups: Vec<u64> = Vec::with_capacity(size);
        cups.append(vec![3,8,9,1,2,5,4,6,7].as_mut());
        for i in 10..size+1 {
            cups.push(i as u64);
        }

        let mut cc = CrabCups::new(cups);
        for _i in 1..10000001 {
            cc.do_move();
        }

        let (s1, s2) = cc.get_stars();
        assert_eq!(s1, 934001);
        assert_eq!(s2, 159792);
        assert_eq!(s1*s2, 149245887792);
    }

}