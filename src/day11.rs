use crate::day;
use crate::file;
use std::cmp::max;

pub struct Day11 {

}

impl day::Day for Day11 {

    fn puzzle1(&self) {
        println!("Day 11, puzzle 1");

        let result = get_result_1(&file::lines("res/day11_1.txt"));

        println!("{}", result);
    }

    fn puzzle2(&self) {
        println!("Day 11, puzzle 2");

        let result = get_result_2(&file::lines("res/day11_1.txt"));

        println!("{}", result);
    }

}

fn get_result_1(input: &[String]) -> usize {
    let mut wa = build_waiting_area(input);
    let x_size = wa.get_x();
    let y_size = wa.get_y();
    loop {
        let mut wa_next = WaitingArea::new(x_size, y_size);
        let mut changed = false;
        for y in 0..y_size {
            for x in 0..x_size {
                let seating = wa.get_seating(x, y);
                if seating != Seating::FLOOR {
                    let num_neighbours = wa.count_neighbours(x, y);
                    if seating == Seating::EMPTY && num_neighbours == 0 {
                        wa_next.set_seating(x, y, Seating::OCCUPIED);
                        changed = true;
                    } else if seating == Seating::OCCUPIED && num_neighbours >= 4 {
                        wa_next.set_seating(x, y, Seating::EMPTY);
                        changed = true;
                    } else {
                        wa_next.set_seating(x, y, seating);
                    }
                }
            }
        }
        if !changed {
            return wa.count_occupied_seats();
        }
        wa.layout = wa_next.layout;
    }
}

fn get_result_2(input: &[String]) -> usize {
    let mut wa = build_waiting_area(input);
    let x_size = wa.get_x();
    let y_size = wa.get_y();
    loop {
        let mut wa_next = WaitingArea::new(x_size, y_size);
        let mut changed = false;
        for y in 0..y_size {
            for x in 0..x_size {
                let seating = wa.get_seating(x, y);
                if seating != Seating::FLOOR {
                    let num_neighbours = wa.count_visible_neighbours(x, y);
                    if seating == Seating::EMPTY && num_neighbours == 0 {
                        wa_next.set_seating(x, y, Seating::OCCUPIED);
                        changed = true;
                    } else if seating == Seating::OCCUPIED && num_neighbours >= 5 {
                        wa_next.set_seating(x, y, Seating::EMPTY);
                        changed = true;
                    } else {
                        wa_next.set_seating(x, y, seating);
                    }
                }
            }
        }
        if !changed {
            return wa.count_occupied_seats();
        }
        wa.layout = wa_next.layout;
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Seating {
    FLOOR,
    EMPTY,
    OCCUPIED
}

struct WaitingArea {
    layout: Vec<Vec<Seating>>,
}

impl WaitingArea {

    fn new(x: usize, y: usize) -> WaitingArea {
        WaitingArea { layout: vec![vec![Seating::FLOOR; x]; y] }
    }

    fn get_x(&self) -> usize {
        self.layout[0].len()
    }

    fn get_y(&self) -> usize {
        self.layout.len()
    }

    fn set_seating(&mut self, x: usize, y: usize, seating: Seating) {
        if y < self.layout.len() && x < self.layout[0].len() {
            self.layout[y][x] = seating;
        } else {
            panic!("Not inside the waiting area: [{}, {}]", x, y);
        }
    }

    fn get_seating(&self, x: usize, y: usize) -> Seating {
        if y < self.layout.len() && x <self.layout[0].len() {
            self.layout[y][x]
        } else {
            Seating::FLOOR
        }
    }

    fn count_occupied_seats(&self) -> usize {
        self.layout
            .iter()
            .flat_map(|row| row.iter())
            .map(|&seating| {
                if seating == Seating::OCCUPIED {
                    1
                } else {
                    0
                }
            })
            .sum()
    }

    fn count_neighbours(&self, x: usize, y: usize) -> usize {
        let mut result = 0;

        for ys in max(y,1)-1..y+2 {
            for xs in max(x,1)-1..x+2 {
                if !(y == ys && x == xs) {
                    if self.get_seating(xs, ys) == Seating::OCCUPIED {
                        result += 1;
                    }
                }
            }
        }
        result
    }

    fn count_visible_neighbours(&self, x: usize, y: usize) -> usize {
        self.count_visible_neighbour(x, y, -1, -1)
            + self.count_visible_neighbour(x, y, -1, 0)
            + self.count_visible_neighbour(x, y, -1, 1)
            + self.count_visible_neighbour(x, y, 0, -1)
            + self.count_visible_neighbour(x, y, 0, 1)
            + self.count_visible_neighbour(x, y, 1, -1)
            + self.count_visible_neighbour(x, y, 1, 0)
            + self.count_visible_neighbour(x, y, 1, 1)
    }

    fn count_visible_neighbour(&self, x: usize, y: usize, dx: isize, dy: isize) -> usize {
        let seating = self.get_visible_neighbour(x, y, dx, dy);
        if seating == Seating::OCCUPIED {
            1
        } else {
            0
        }
    }

    fn get_visible_neighbour(&self, x: usize, y: usize, dx: isize, dy: isize) -> Seating {
        let mut xs = dx + x as isize;
        let mut ys = dy + y as isize;
        while ys >= 0 && (ys as usize) < self.get_y() && xs >= 0 && (xs as usize) < self.get_x() {
            let seating = self.get_seating(xs as usize, ys as usize);
            if seating != Seating::FLOOR {
                return seating;
            }
            xs = dx + xs;
            ys = dy + ys;
        }
        Seating::FLOOR
    }

}

fn build_waiting_area(input: &[String]) -> WaitingArea {
    let y_size = input.len();
    let x_size = input[0].len();

    let mut wa = WaitingArea::new(x_size, y_size);
    for y in 0..y_size {
        let cs = input[y].chars().collect::<Vec<char>>();
        for x in 0..x_size {
            let c = cs[x];
            if c == 'L' {
                wa.set_seating(x, y, Seating::EMPTY);
            } else if c == '#' {
                wa.set_seating(x, y, Seating::OCCUPIED);
            }
        }
    }
    wa
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_result_1() {
        let input = vec![
            "L.LL.LL.LL".to_string(),
            "LLLLLLL.LL".to_string(),
            "L.L.L..L..".to_string(),
            "LLLL.LL.LL".to_string(),
            "L.LL.LL.LL".to_string(),
            "L.LLLLL.LL".to_string(),
            "..L.L.....".to_string(),
            "LLLLLLLLLL".to_string(),
            "L.LLLLLL.L".to_string(),
            "L.LLLLL.LL".to_string()
        ];
        assert_eq!(get_result_1(&input), 37);
    }

    #[test]
    fn test_get_result_2() {
        let input = vec![
            "L.LL.LL.LL".to_string(),
            "LLLLLLL.LL".to_string(),
            "L.L.L..L..".to_string(),
            "LLLL.LL.LL".to_string(),
            "L.LL.LL.LL".to_string(),
            "L.LLLLL.LL".to_string(),
            "..L.L.....".to_string(),
            "LLLLLLLLLL".to_string(),
            "L.LLLLLL.L".to_string(),
            "L.LLLLL.LL".to_string()
        ];
        assert_eq!(get_result_2(&input), 26);
    }

}