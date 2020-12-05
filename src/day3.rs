use crate::day;
use crate::file;

pub struct Day3 {

}

struct Map<'a> {
    map: &'a Vec<String>,
    x: usize,
    y: usize
}

impl<'a> Map<'a> {
    fn new(the_map: &'a Vec<String>) -> Map {
        Map { map: the_map, x: 0, y: 0 }
    }

    fn moove(&mut self, x: usize, y: usize) {
        if self.y < self.map.len() {
            self.x += x;
            self.y += y;
            if self.y < self.map.len() && self.x >= self.map[self.y].len() {
                self.x = self.x - self.map[self.y].len();
            }
        }
    }

    fn is_end(&self) -> bool {
        return self.y >= self.map.len();
    }

    fn is_tree(&self) -> bool {
        if self.y < self.map.len() {
            return &self.map[self.y][self.x..self.x+1] == "#";
        }
        return false;
    }
}

impl day::Day for Day3 {

    fn puzzle1(&self) {
        println!("Day 3, puzzle 1");

        let hits = count_hits_1(&file::lines("res/day3_1.txt"));

        println!("{}", hits);
    }

    fn puzzle2(&self) {
        println!("Day 3, puzzle 2");

        let hits = count_hits_2(&file::lines("res/day3_1.txt"));

        println!("{}", hits);
    }

}

fn count_hits(l: &Vec<String>, x: usize, y: usize) -> u64 {
    let mut map = Map::new(l);

    let mut hits = 0;
    if map.is_tree() {
        hits += 1;
    }
    while !map.is_end() {
        map.moove(x, y);
        if map.is_tree() {
            hits += 1;
        }
    }
    return hits;
}

fn count_hits_1(l: &Vec<String>) -> u64 {
    return count_hits(l, 3, 1);
}

fn count_hits_2(l: &Vec<String>) -> u64 {
    return count_hits(l, 1, 1)
        * count_hits(l, 3, 1)
        * count_hits(l, 5, 1)
        * count_hits(l, 7, 1)
        * count_hits(l, 1, 2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_hits_1() {
        assert_eq!(
            count_hits_1(&vec![
                "..##.......".to_string(),
                "#...#...#..".to_string(),
                ".#....#..#.".to_string(),
                "..#.#...#.#".to_string(),
                ".#...##..#.".to_string(),
                "..#.##.....".to_string(),
                ".#.#.#....#".to_string(),
                ".#........#".to_string(),
                "#.##...#...".to_string(),
                "#...##....#".to_string(),
                ".#..#...#.#".to_string()]
            ),
            7
        );
    }

    #[test]
    fn test_count_hits_2() {
        assert_eq!(
            count_hits_2(&vec![
                "..##.......".to_string(),
                "#...#...#..".to_string(),
                ".#....#..#.".to_string(),
                "..#.#...#.#".to_string(),
                ".#...##..#.".to_string(),
                "..#.##.....".to_string(),
                ".#.#.#....#".to_string(),
                ".#........#".to_string(),
                "#.##...#...".to_string(),
                "#...##....#".to_string(),
                ".#..#...#.#".to_string()]
            ),
            336
        );
    }

}