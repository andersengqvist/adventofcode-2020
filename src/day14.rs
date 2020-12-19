use crate::day;
use crate::file;
use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

pub struct Day14 {

}

impl day::Day for Day14 {

    fn puzzle1(&self) {
        println!("Day 14, puzzle 1");

        let result = get_result_1(&file::lines("res/day14_1.txt"));

        println!("{}", result);
    }

    fn puzzle2(&self) {
        println!("Day 14, puzzle 2");

        let result = get_result_2(&file::lines("res/day14_1.txt"));

        println!("{}", result);
    }

}

lazy_static! {
    static ref RE_MASK: Regex = Regex::new("mask = (.+)$").unwrap();
    static ref RE_MEM: Regex = Regex::new("mem\\[(\\d+)\\] = (\\d+)$").unwrap();
}

fn get_result_1(input: &Vec<String>) -> u64 {
    let mut computer = Computer1::new();

    for l in input {
        let mask_cap_opt = RE_MASK.captures(l);
        if mask_cap_opt.is_some() {
            let mask_cap = mask_cap_opt.unwrap();
            computer.set_bitmask(&mask_cap[1].to_string());
        } else {
            let mem_cap = RE_MEM.captures(l).unwrap();
            let address = mem_cap[1].parse::<u64>().expect("Could not parse address");
            let value = mem_cap[2].parse::<u64>().expect("Could not parse value");
            computer.write_to_memory(address, value);
        }
    }

    computer.sum_memory()
}

struct Computer1 {
    memory: HashMap<u64, u64>,
    one_mask: u64,
    zero_mask: u64
}

impl Computer1 {

    fn new() -> Computer1 {
        Computer1 { memory: HashMap::new(), one_mask: 0, zero_mask: !0 }
    }

    fn set_bitmask(&mut self, bitmask: &str) {
        self.one_mask = 0;
        self.zero_mask = !0;
        let mut idx = 0u64;
        for c in bitmask.chars().rev() {
            if c == '1' {
                self.one_mask = self.one_mask | (1 << idx);
            }
            if c == '0' {
                self.zero_mask = self.zero_mask & !(1 << idx);
            }
            idx += 1;
        }
    }

    fn write_to_memory(&mut self, address: u64, value: u64) {
        let v1 = value & self.zero_mask;
        let v2 = v1 | self.one_mask;
        self.memory.insert(address, v2);
    }

    fn sum_memory(&self) -> u64 {
        self.memory.values().sum()
    }
}

fn get_result_2(input: &Vec<String>) -> u64 {
    let mut computer = Computer2::new();

    for l in input {
        let mask_cap_opt = RE_MASK.captures(l);
        if mask_cap_opt.is_some() {
            let mask_cap = mask_cap_opt.unwrap();
            computer.set_bitmask(&mask_cap[1].to_string());
        } else {
            let mem_cap = RE_MEM.captures(l).unwrap();
            let address = mem_cap[1].parse::<u64>().expect("Could not parse address");
            let value = mem_cap[2].parse::<u64>().expect("Could not parse value");
            computer.write_to_memory(address, value);
        }
    }

    computer.sum_memory()
}

struct Computer2 {
    memory: HashMap<u64, u64>,
    one_mask: u64,
    floating_mask: u64
}

impl Computer2 {

    fn new() -> Computer2 {
        Computer2 { memory: HashMap::new(), one_mask: 0, floating_mask: 0 }
    }

    fn set_bitmask(&mut self, bitmask: &str) {
        self.one_mask = 0;
        self.floating_mask = 0;
        let mut idx = 0u64;
        for c in bitmask.chars().rev() {
            if c == '1' {
                self.one_mask = self.one_mask | (1 << idx);
            }
            if c == 'X' {
                self.floating_mask = self.floating_mask | (1 << idx);
            }
            idx += 1;
        }
    }

    fn write_to_memory(&mut self, address: u64, value: u64) {
        let a = address | self.one_mask;
        self.write_like_crazy(a, self.floating_mask, 1, value);
    }

    fn write_like_crazy(&mut self, address: u64, floatings: u64, idx: u64, value: u64) {
        if floatings == 0 {
            self.memory.insert(address, value);
        } else {
            let new_idx = idx << 1;
            if floatings & idx != 0 {
                let new_floatings = floatings & !idx;
                self.write_like_crazy(address | idx, new_floatings, new_idx, value);
                self.write_like_crazy(address & !idx, new_floatings, new_idx, value);
            } else {
                self.write_like_crazy(address, floatings, new_idx, value);
            }
        }
    }

    fn sum_memory(&self) -> u64 {
        self.memory.values().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_computer1() {
        let mut computer = Computer1::new();
        computer.set_bitmask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X");
        computer.write_to_memory(8, 11);
        computer.write_to_memory(7, 101);
        computer.write_to_memory(8, 0);
        assert_eq!(computer.sum_memory(), 165);
    }

    #[test]
    fn test_computer2() {
        let mut computer = Computer2::new();
        computer.set_bitmask("000000000000000000000000000000X1001X");
        computer.write_to_memory(42, 100);
        computer.set_bitmask("00000000000000000000000000000000X0XX");
        computer.write_to_memory(26, 1);
        assert_eq!(computer.sum_memory(), 208);
    }

}