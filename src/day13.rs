use crate::day;
use std::ops::{Add, Div, Rem};

pub struct Day13 {

}

impl day::Day for Day13 {

    fn puzzle1(&self) {
        println!("Day 13, puzzle 1");

        let result = get_result_1();

        println!("{}", result);
    }

    fn puzzle2(&self) {
        println!("Day 13, puzzle 2");

        let result = get_result_2();

        println!("{}", result);
    }

}

fn get_result_1() -> u64 {
    let earliest_depart: u64 = 1000067;
    let bus_ids: Vec<u64> = vec!(17,37,439,29,13,23,787,41,19);
    let (bus_id, depart_time) = get_earliest_bus(earliest_depart, &bus_ids);
    (depart_time - earliest_depart) * bus_id
}

// returns (bus_id, wait_time)
fn get_earliest_bus(earliest_depart: u64, bus_ids: &[u64]) -> (u64, u64) {
    bus_ids
        .iter()
        .map(|&id| (id, get_nearest_time(earliest_depart, id)))
        .min_by(|(_i1, w1), (_i2, w2)|  w1.cmp(w2))
        .unwrap()
}

fn get_nearest_time(earliest_depart: u64, bus_id: u64) -> u64 {
    let m = (earliest_depart / bus_id) * bus_id;
    if m < earliest_depart {
        m + bus_id
    } else {
        m
    }
}

/**
Solution taken from https://math.stackexchange.com/questions/2218763/how-to-find-lcm-of-two-numbers-when-one-starts-with-an-offset
Especially the combine_phased_rotations and extended_gcd methods are directly taken from the Python code by Eric Langlois and transformed to Rust.
I do not fully understand what is going on though.
*/
fn get_result_2() -> i128 {
    let bus_ids: Vec<i128> = vec!(17,0,0,0,0,0,0,0,0,0,0,37,0,0,0,0,0,439,0,29,0,0,0,0,0,0,0,0,0,0,13,0,0,0,0,0,0,0,0,0,23,0,0,0,0,0,0,0,787,0,0,0,0,0,0,0,0,0,41,0,0,0,0,0,0,0,0,19);
    earliest_timestamp(&bus_ids)
}

fn earliest_timestamp(input: &[i128]) -> i128 {
    let (period, phase) = input
        .iter()
        .enumerate()
        .filter_map(|(idx, &value)| {
            if value > 0 {
                Some((value, idx as i128))
            } else {
                None
            }
        })
        .fold(
            (1, 0),
            |(p1, ph1), (p2, ph2)| combine_phased_rotations(p1, ph1, p2, ph2)
        );
    period - phase
}

/**
Combine two phased rotations into a single phased rotation

    Returns: (combined_period, combined_phase)

    The combined rotation is at its reference point if and only if both a and b
    are at their reference points.
*/
fn combine_phased_rotations(a_period: i128, a_phase: i128, b_period: i128, b_phase: i128) -> (i128, i128) {
    let (gcd, s, _t) = extended_gcd(a_period, b_period);
    let phase_difference = a_phase - b_phase;
    let (pd_mult, pd_remainder) = divmod(phase_difference, gcd);
    if pd_remainder != 0 {
        panic!("Rotation reference points never synchronize.");
    }
    let combined_period = a_period / gcd * b_period;
    let combined_phase = modulus(a_phase - s * pd_mult * a_period, combined_period);
    (combined_period, combined_phase)
}

/**
Extended Greatest Common Divisor Algorithm

    Returns:
        gcd: The greatest common divisor of a and b.
        s, t: Coefficients such that s*a + t*b = gcd

    Reference:
        https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm#Pseudocode
*/
fn extended_gcd(a: i128, b: i128) -> (i128, i128, i128) {
    let mut old_r: i128 = a;
    let mut r: i128 = b;
    let mut old_s: i128 = 1;
    let mut s: i128 = 0;
    let mut old_t: i128 = 0;
    let mut t: i128 = 1;
    while r != 0 {
        let (quotient, remainder) = divmod(old_r, r);
        old_r = r;
        r = remainder;
        let old_old_s = old_s;
        old_s = s;
        s = old_old_s - quotient * s;
        let old_old_t = old_t;
        old_t = t;
        t = old_old_t - quotient * t;
    }
    (old_r, old_s, old_t)
}

fn divmod<T>(a: T, b: T) -> (T, T)
    where
        T: Add<Output = T>,
        T: Div<Output = T>,
        T: Rem<Output = T>,
        T: Copy
{
    (a/b, modulus(a, b))
}

// Took a while to understand that in Rust and Python the % operator does not work the same way
fn modulus<T>(a: T, b: T) -> T
    where
        T: Add<Output = T>,
        T: Rem<Output = T>,
        T: Copy
{
    ((a % b) + b) % b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_earliest_bus() {
        let bus_ids: Vec<u64> = vec!(7,13,59,31,19);
        assert_eq!(get_earliest_bus(939, &bus_ids), (59, 944));
    }


    #[test]
    fn test_combine_phased_rotations() {
        assert_eq!(combine_phased_rotations(1,0, 5, 0), (5, 0));
        assert_eq!(combine_phased_rotations(5,0, 1, 0), (5, 0));
        assert_eq!(combine_phased_rotations(3,0, 5, 0), (15, 0));
        assert_eq!(combine_phased_rotations(3, 0, 5, 1), (15, 6));
        assert_eq!(combine_phased_rotations(5, 1, 3, 0), (15, 6));
        assert_eq!(combine_phased_rotations(5, 0, 10, 0), (10, 0));
    }

    #[test]
    fn test_multiple_combine_phased_rotations_1() {
        let (p1, ph1) = combine_phased_rotations(1, 0, 67, 0);
        let (p2, ph2) = combine_phased_rotations(p1, ph1, 7, 1);
        let (p3, ph3) = combine_phased_rotations(p2, ph2, 59, 2);
        let (p4, ph4) = combine_phased_rotations(p3, ph3, 61, 3);
        assert_eq!(p4 - ph4, 754018);
    }

    #[test]
    fn test_earliest_timestamp() {
        assert_eq!(earliest_timestamp(&vec!(3,5,7)), 54);
        assert_eq!(earliest_timestamp(&vec!(7,13,0,0,59,0,31,19)), 1068781);
        assert_eq!(earliest_timestamp(&vec!(17,0,13,19)), 3417);
        assert_eq!(earliest_timestamp(&vec!(67,7,59,61)), 754018);
        assert_eq!(earliest_timestamp(&vec!(67,0,7,59,61)), 779210);
        assert_eq!(earliest_timestamp(&vec!(67,7,0,59,61)), 1261476);
        assert_eq!(earliest_timestamp(&vec!(1789,37,47,1889)), 1202161486);
    }

}