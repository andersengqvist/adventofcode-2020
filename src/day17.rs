use crate::day;
use crate::file;
use std::collections::HashSet;
use std::cmp::{min, max};

pub struct Day17 {

}

impl day::Day for Day17 {

    fn puzzle1(&self) {
        println!("Day 17, puzzle 1");

        let result = get_result_1(&file::lines("res/day17_1.txt"));

        println!("{}", result);
    }

    fn puzzle2(&self) {
        println!("Day 17, puzzle 2");

        let result = get_result_2(&file::lines("res/day17_1.txt"));

        println!("{}", result);
    }

}

fn get_result_1(input: &[String]) -> usize {
    run_cycles_and_get_result_1(input, 6)
}

fn run_cycles_and_get_result_1(input: &[String], cycles: usize) -> usize {
    let mut initial_state = vec![];
    let mut y = 0;
    for s in input {
        let mut x = 0;
        for c in s.chars() {
            if c == '#' {
                initial_state.push((x,y,0));
            }
            x += 1;
        }
        y += 1;
    }
    run_cycles_1(&initial_state, cycles)
}

fn run_cycles_1(initial_state: &[(i32,i32,i32)], num_cycles: usize) -> usize {
    let mut pd = PocketDimension3D::new();

    for init in initial_state {
        pd.set_active(init);
    }

    //println!("{:?}", pd);

    for i in 1..num_cycles+1 {
        let mut pd_next = PocketDimension3D::new();
        if i == num_cycles {
            pd_next.do_inspect = false;
        }
        for insp in &pd.to_inspect {
            let is_active = pd.is_active(insp);
            let neighbours = pd.count_neighbours(insp);
            if is_active && (neighbours == 2 || neighbours == 3) {
                pd_next.set_active(insp);
            }
            else if !is_active && neighbours == 3 {
                pd_next.set_active(insp);
            }
        }
        pd.active = pd_next.active;
        pd.to_inspect = pd_next.to_inspect;
        pd.x_min = pd_next.x_min;
        pd.x_max = pd_next.x_max;
        pd.y_min = pd_next.y_min;
        pd.y_max = pd_next.y_max;
        pd.z_min = pd_next.z_min;
        pd.z_max = pd_next.z_max;
        //println!("{:?}", pd);
    }
    pd.count_active()
}

fn get_result_2(input: &Vec<String>) -> usize {
    run_cycles_and_get_result_2(input, 6)
}

fn run_cycles_and_get_result_2(input: &[String], cycles: usize) -> usize {
    let mut initial_state = vec![];
    let mut y = 0;
    for s in input {
        let mut x = 0;
        for c in s.chars() {
            if c == '#' {
                initial_state.push((x,y,0,0));
            }
            x += 1;
        }
        y += 1;
    }
    run_cycles_2(&initial_state, cycles)
}

fn run_cycles_2(initial_state: &[(i32,i32,i32,i32)], num_cycles: usize) -> usize {
    let mut pd = PocketDimension4D::new();

    for init in initial_state {
        pd.set_active(init);
    }

    //println!("{:?}", pd);

    for i in 1..num_cycles+1 {
        let mut pd_next = PocketDimension4D::new();
        if i == num_cycles {
            pd_next.do_inspect = false;
        }
        for insp in &pd.to_inspect {
            let is_active = pd.is_active(insp);
            let neighbours = pd.count_neighbours(insp);
            if is_active && (neighbours == 2 || neighbours == 3) {
                pd_next.set_active(insp);
            }
            else if !is_active && neighbours == 3 {
                pd_next.set_active(insp);
            }
        }
        pd.active = pd_next.active;
        pd.to_inspect = pd_next.to_inspect;
        pd.x_min = pd_next.x_min;
        pd.x_max = pd_next.x_max;
        pd.y_min = pd_next.y_min;
        pd.y_max = pd_next.y_max;
        pd.z_min = pd_next.z_min;
        pd.z_max = pd_next.z_max;
        pd.w_min = pd_next.w_min;
        pd.w_max = pd_next.w_max;
        //println!("{:?}", pd);
    }
    pd.count_active()
}

struct PocketDimension3D {
    active: HashSet<(i32, i32, i32)>,
    to_inspect: HashSet<(i32, i32, i32)>,
    do_inspect: bool,
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
    z_min: i32,
    z_max: i32,
}

impl std::fmt::Debug for PocketDimension3D {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {

        match writeln!(f, "x=[{},{}] y=[{},{}] z=[{},{}]", self.x_min, self.x_max, self.y_min, self.y_max, self.z_min, self.z_max) {
            Result::Ok(_) => (),
            err => return err
        }
        for z in self.z_min..self.z_max+1 {
            match writeln!(f, "z={}", z) {
                Result::Ok(_) => (),
                err => return err
            }
            for y in self.y_min..self.y_max+1 {
                for x in self.x_min..self.x_max+1 {
                    if self.is_active(&(x,y,z)) {
                        match write!(f, "#")  {
                            Result::Ok(_) => (),
                            err => return err
                        }
                    } else {
                        match write!(f, ".") {
                            Result::Ok(_) => (),
                            err => return err
                        }
                    }
                }
                match writeln!(f, "")  {
                    Result::Ok(_) => (),
                    err => return err
                }
            }
        }
        writeln!(f, "")
    }
}

impl PocketDimension3D {

    fn new() -> PocketDimension3D {
        PocketDimension3D {
            active: HashSet::new(),
            to_inspect: HashSet::new(),
            do_inspect: true,
            x_min: 2147483647,
            x_max: -2147483648,
            y_min: 2147483647,
            y_max: -2147483648,
            z_min: 2147483647,
            z_max: -2147483648,
        }
    }

    fn set_active(&mut self, p: &(i32,i32,i32)) {
        let pc = p.clone();
        self.active.insert(pc);
        self.x_min = min(self.x_min, pc.0);
        self.x_max = max(self.x_max, pc.0);
        self.y_min = min(self.y_min, pc.1);
        self.y_max = max(self.y_max, pc.1);
        self.z_min = min(self.z_min, pc.2);
        self.z_max = max(self.z_max, pc.2);
        if self.do_inspect {
            for x in pc.0 - 1..pc.0 + 2 {
                for y in pc.1 - 1..pc.1 + 2 {
                    for z in pc.2 - 1..pc.2 + 2 {
                        self.to_inspect.insert((x, y, z));
                    }
                }
            }
        }
    }

    fn is_active(&self, p: &(i32,i32,i32)) -> bool {
        self.active.contains(p)
    }

    fn count_neighbours(&self, p: &(i32,i32,i32)) -> usize {
        let mut result = 0;
        for x in p.0-1..p.0+2 {
            for y in p.1-1..p.1+2 {
                for z in p.2-1..p.2+2 {
                    if x == p.0 && y == p.1 && z == p.2 {
                        // Do not count
                    }
                    else if self.active.contains(&(x,y,z)) {
                        result += 1;
                    }
                }
            }
        }
        result
    }

    fn count_active(&self) -> usize {
        self.active.len()
    }
}

struct PocketDimension4D {
    active: HashSet<(i32, i32, i32, i32)>,
    to_inspect: HashSet<(i32, i32, i32, i32)>,
    do_inspect: bool,
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
    z_min: i32,
    z_max: i32,
    w_min: i32,
    w_max: i32,
}

impl std::fmt::Debug for PocketDimension4D {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {

        match writeln!(f, "x=[{},{}] y=[{},{}] z=[{},{}] w=[{},{}]", self.x_min, self.x_max, self.y_min, self.y_max, self.z_min, self.z_max, self.w_min, self.w_max) {
            Result::Ok(_) => (),
            err => return err
        }
        for w in self.w_min..self.w_max+1 {
            for z in self.z_min..self.z_max + 1 {
                match writeln!(f, "z={}, w={}", z, w) {
                    Result::Ok(_) => (),
                    err => return err
                }
                for y in self.y_min..self.y_max + 1 {
                    for x in self.x_min..self.x_max + 1 {
                        if self.is_active(&(x, y, z, w)) {
                            match write!(f, "#") {
                                Result::Ok(_) => (),
                                err => return err
                            }
                        } else {
                            match write!(f, ".") {
                                Result::Ok(_) => (),
                                err => return err
                            }
                        }
                    }
                    match writeln!(f, "") {
                        Result::Ok(_) => (),
                        err => return err
                    }
                }
            }
        }
        writeln!(f, "")
    }
}

impl PocketDimension4D {

    fn new() -> PocketDimension4D {
        PocketDimension4D {
            active: HashSet::new(),
            to_inspect: HashSet::new(),
            do_inspect: true,
            x_min: 2147483647,
            x_max: -2147483648,
            y_min: 2147483647,
            y_max: -2147483648,
            z_min: 2147483647,
            z_max: -2147483648,
            w_min: 2147483647,
            w_max: -2147483648,
        }
    }

    fn set_active(&mut self, p: &(i32,i32,i32,i32)) {
        let pc = p.clone();
        self.active.insert(pc);
        self.x_min = min(self.x_min, pc.0);
        self.x_max = max(self.x_max, pc.0);
        self.y_min = min(self.y_min, pc.1);
        self.y_max = max(self.y_max, pc.1);
        self.z_min = min(self.z_min, pc.2);
        self.z_max = max(self.z_max, pc.2);
        self.w_min = min(self.w_min, pc.3);
        self.w_max = max(self.w_max, pc.3);
        if self.do_inspect {
            for x in pc.0 - 1..pc.0 + 2 {
                for y in pc.1 - 1..pc.1 + 2 {
                    for z in pc.2 - 1..pc.2 + 2 {
                        for w in pc.3 - 1..pc.3 + 2 {
                            self.to_inspect.insert((x, y, z, w));
                        }
                    }
                }
            }
        }
    }

    fn is_active(&self, p: &(i32,i32,i32,i32)) -> bool {
        self.active.contains(p)
    }

    fn count_neighbours(&self, p: &(i32,i32,i32,i32)) -> usize {
        let mut result = 0;
        for x in p.0-1..p.0+2 {
            for y in p.1-1..p.1+2 {
                for z in p.2-1..p.2+2 {
                    for w in p.3-1..p.3+2 {
                        if x == p.0 && y == p.1 && z == p.2 && w == p.3 {
                            // Do not count
                        } else if self.active.contains(&(x, y, z, w)) {
                            result += 1;
                        }
                    }
                }
            }
        }
        result
    }

    fn count_active(&self) -> usize {
        self.active.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_cycles_1() {
        let init_state = vec![
            (1,0,0),
            (2,1,0),
            (0,2,0),
            (1,2,0),
            (2,2,0)
        ];
        assert_eq!(run_cycles_1(&init_state, 6), 112);
    }

    #[test]
    fn test_run_cycles_and_get_result_1() {
        let init_state = vec![
            ".#.".to_string(),
            "..#".to_string(),
            "###".to_string()
        ];
        assert_eq!(run_cycles_and_get_result_1(&init_state, 6), 112);
    }

    #[test]
    fn test_run_cycles_2() {
        let init_state = vec![
            (1,0,0,0),
            (2,1,0,0),
            (0,2,0,0),
            (1,2,0,0),
            (2,2,0,0)
        ];
        assert_eq!(run_cycles_2(&init_state, 6), 848);
    }

    #[test]
    fn test_run_cycles_and_get_result_2() {
        let init_state = vec![
            ".#.".to_string(),
            "..#".to_string(),
            "###".to_string()
        ];
        assert_eq!(run_cycles_and_get_result_2(&init_state, 6), 848);
    }
}