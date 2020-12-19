use crate::day;
use crate::file;

use lazy_static::lazy_static;
use regex::Regex;

pub struct Day12 {

}

impl day::Day for Day12 {

    fn puzzle1(&self) {
        println!("Day 12, puzzle 1");

        let result = get_result_1(&file::lines("res/day12_1.txt"));

        println!("{}", result);
    }

    fn puzzle2(&self) {
        println!("Day 12, puzzle 2");

        let result = get_result_2(&file::lines("res/day12_1.txt"));

        println!("{}", result);
    }

}

fn build_instructions(instructions: &Vec<String>) -> Vec<Instruction> {
    instructions
        .iter()
        .map(|s| build_instruction(s))
        .collect()
}

lazy_static! {
    static ref RE: Regex = Regex::new("([[:alpha:]])([0-9]+)").unwrap();
}

fn build_instruction(instruction: &str) -> Instruction {
    let cap = RE.captures(instruction).unwrap();
    let action = match &cap[1] {
        "N" => Action::N,
        "S" => Action::S,
        "E" => Action::E,
        "W" => Action::W,
        "L" => Action::L,
        "R" => Action::R,
        "F" => Action::F,
        _ => panic!("Unknown action {}", &cap[1])
    };
    let value = cap[2].parse::<i32>().expect("Could not parse value");
    Instruction { action, value }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug, Copy)]
enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST
}

#[derive(PartialEq, Eq, Hash, Clone, Debug, Copy)]
enum Action {
    N,
    S,
    E,
    W,
    L,
    R,
    F
}

struct Instruction {
    action: Action,
    value: i32
}

struct Ship1 {
    direction: Direction, // The direction the ship is facing
    ns_pos: i32, // north/south position
    ew_pos: i32 // east/west position
}

impl Ship1 {

    fn new() -> Ship1 {
        Ship1 { direction: Direction::EAST, ns_pos: 0, ew_pos: 0 }
    }

    fn move_direction(&mut self, direction: Direction, value: i32) {
        match direction {
            Direction::NORTH => self.ns_pos += value,
            Direction::SOUTH => self.ns_pos -= value,
            Direction::EAST => self.ew_pos += value,
            Direction::WEST => self.ew_pos -= value,
        }
    }

    fn turn_right(&mut self, value: i32) {
        if value < 0 {
            self.turn_right(360 + value);
        } else if value > 0 {
            let dir = self.direction;
            match dir {
                Direction::NORTH => self.direction = Direction::EAST,
                Direction::SOUTH => self.direction = Direction::WEST,
                Direction::EAST => self.direction = Direction::SOUTH,
                Direction::WEST => self.direction = Direction::NORTH,
            }
            self.turn_right(value - 90);
        }
    }

    fn perform(&mut self, action: Action, value: i32) {
        match action {
            Action::N => self.move_direction(Direction::NORTH, value),
            Action::S => self.move_direction(Direction::SOUTH, value),
            Action::E => self.move_direction(Direction::EAST, value),
            Action::W => self.move_direction(Direction::WEST, value),
            Action::L => self.turn_right(360 - value),
            Action::R => self.turn_right(value),
            Action::F => self.move_direction(self.direction, value)
        }
    }

    fn manhattan_distance(&self) -> i32 {
        self.ns_pos.abs() + self.ew_pos.abs()
    }
}

struct Ship2 {
    wp_ns_pos: i32, // north/south position of waypoint, relative to the ship
    wp_ew_pos: i32, // east/west position of waypoint, relative to the ship
    ns_pos: i32, // north/south position
    ew_pos: i32 // east/west position
}

impl Ship2 {

    fn new() -> Ship2 {
        Ship2 { wp_ns_pos: 1, wp_ew_pos: 10, ns_pos: 0, ew_pos: 0 }
    }

    fn move_waypoint(&mut self, direction: Direction, value: i32) {
        match direction {
            Direction::NORTH => self.wp_ns_pos += value,
            Direction::SOUTH => self.wp_ns_pos -= value,
            Direction::EAST => self.wp_ew_pos += value,
            Direction::WEST => self.wp_ew_pos -= value,
        }
    }

    fn rotate_waypoint_right(&mut self, value: i32) {
        if value < 0 {
            self.rotate_waypoint_right(360 + value);
        } else if value > 0 {
            let tmp = self.wp_ns_pos;
            self.wp_ns_pos = -self.wp_ew_pos;
            self.wp_ew_pos = tmp;
            self.rotate_waypoint_right(value - 90);
        }
    }

    fn move_ship(&mut self, value: i32) {
        self.ns_pos += self.wp_ns_pos * value;
        self.ew_pos += self.wp_ew_pos * value;
    }

    fn perform(&mut self, action: Action, value: i32) {
        match action {
            Action::N => self.move_waypoint(Direction::NORTH, value),
            Action::S => self.move_waypoint(Direction::SOUTH, value),
            Action::E => self.move_waypoint(Direction::EAST, value),
            Action::W => self.move_waypoint(Direction::WEST, value),
            Action::L => self.rotate_waypoint_right(360 - value),
            Action::R => self.rotate_waypoint_right(value),
            Action::F => self.move_ship(value)
        }
    }

    fn manhattan_distance(&self) -> i32 {
        self.ns_pos.abs() + self.ew_pos.abs()
    }
}

fn get_result_1(input: &Vec<String>) -> i32 {
    let instructions = build_instructions(input);
    let mut ship = Ship1::new();
    for instruction in instructions {
        ship.perform(instruction.action, instruction.value);
    }
    ship.manhattan_distance()
}

fn get_result_2(input: &Vec<String>) -> i32 {
    let instructions = build_instructions(input);
    let mut ship = Ship2::new();
    for instruction in instructions {
        ship.perform(instruction.action, instruction.value);
    }
    ship.manhattan_distance()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ship1_perform() {
        let mut ship = Ship1::new();
        ship.perform(Action::F, 10);
        ship.perform(Action::N, 3);
        ship.perform(Action::F, 7);
        ship.perform(Action::R, 90);
        ship.perform(Action::F, 11);
        assert_eq!(ship.manhattan_distance(), 25);
    }

    #[test]
    fn test_ship2_perform() {
        let mut ship = Ship2::new();
        ship.perform(Action::F, 10);
        ship.perform(Action::N, 3);
        ship.perform(Action::F, 7);
        ship.perform(Action::R, 90);
        ship.perform(Action::F, 11);
        assert_eq!(ship.manhattan_distance(), 286);
    }

}