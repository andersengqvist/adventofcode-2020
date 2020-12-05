use crate::day;
use crate::file;

pub struct Day5 {

}

impl day::Day for Day5 {

    fn puzzle1(&self) {
        println!("Day 5, puzzle 1");

        let highest_seat_id = find_highest_seat_id(&file::lines("res/day5_1.txt"));

        println!("{}", highest_seat_id);
    }

    fn puzzle2(&self) {
        println!("Day 5, puzzle 2");
// 593 is too high
        let my_seat_id = find_my_seat_id(&file::lines("res/day5_1.txt"));

        println!("{}", my_seat_id);
    }

}

fn find_highest_seat_id(l: &Vec<String>) -> u16 {
    l.iter()
        .map(|s| find_seat(s))
        .map( |t| calc_seat_id(t))
        .max()
        .unwrap()
}

fn find_my_seat_id(l: &Vec<String>) -> u16 {
    let mut v: Vec<u16> =
        l
            .iter()
            .map(|s| find_seat(s))
            .map( |t| calc_seat_id(t))
            .collect();
    v.sort();

    let mut expect_next = v[0];
    for id in v {
        if id != expect_next {
            return expect_next;
        }
        expect_next = id + 1;
    }
    0
}

fn find_seat(str: &str) -> (u8, u8) {

    let (row, seat) = str.split_at(7);

    let mut low: u8 = 0;
    let mut high: u8 = 127;
    let mut seat_row: u8 = 0;
    let mut seat_col: u8 = 0;

    for c in row.chars() {
        if c == 'F' {
            high = (low + high) / 2;
            seat_row = high;
        } else if c == 'B' {
            low = (low + high + 1) / 2;
            seat_row = low;
        } else {
            panic!("Unknown letter {}", c);
        }
    }

    low = 0;
    high = 7;

    for c in seat.chars() {
        if c == 'L' {
            high = (low + high) / 2;
            seat_col = high;
        } else if c == 'R' {
            low = (low + high + 1) / 2;
            seat_col = low;
        } else {
            panic!("Unknown letter {}", c);
        }
    }

    (seat_row, seat_col)
}

fn calc_seat_id(seat: (u8, u8)) -> u16 {
    let (row, col) = seat;
    (row as u16) * 8 + (col as u16)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_seat() {
        assert_eq!(find_seat("FBFBBFFRLR"), (44, 5));
        assert_eq!(find_seat("BFFFBBFRRR"), (70, 7));
        assert_eq!(find_seat("FFFBBBFRRR"), (14, 7));
        assert_eq!(find_seat("BBFFBBFRLL"), (102, 4));
    }

    #[test]
    fn test_calc_seat_id() {
        assert_eq!(calc_seat_id((44, 5)), 357);
        assert_eq!(calc_seat_id((70, 7)), 567);
        assert_eq!(calc_seat_id((14, 7)), 119);
        assert_eq!(calc_seat_id((102, 4)), 820);
    }

}