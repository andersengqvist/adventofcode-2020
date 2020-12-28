use crate::day;
use crate::file;
use std::collections::HashSet;

pub struct Day24 {

}

impl day::Day for Day24 {

    fn puzzle1(&self) {
        println!("Day 24, puzzle 1");

        let result = get_result_1(&file::lines("res/day24_1.txt"));

        println!("{}", result);
    }

    fn puzzle2(&self) {
        println!("Day 24, puzzle 2");

        let result = get_result_2(&file::lines("res/day24_1.txt"), 100);

        println!("{}", result);
    }

}

fn get_result_1(input: &[String]) -> usize {
    let mut tileset = HashSet::new();

    for p in input {
        let c = get_coordinates(p);
        if tileset.contains(&c) {
             tileset.remove(&c);
        } else {
            tileset.insert(c);
        }
    }

    tileset.len()
}

fn get_result_2(input: &[String], iterations: usize) -> usize {
    let mut tileset = HashSet::new();
    let mut to_visit = HashSet::new();

    for p in input {
        let c = get_coordinates(p);
        if tileset.contains(&c) {
            tileset.remove(&c);
        } else {
            tileset.insert(c);
            to_visit.insert((c.0, c.1));
            to_visit.insert((c.0, c.1-1));
            to_visit.insert((c.0, c.1+1));
            to_visit.insert((c.0-1, c.1));
            to_visit.insert((c.0+1, c.1));
            to_visit.insert((c.0-1, c.1+1));
            to_visit.insert((c.0+1, c.1-1));
        }
    }

    for _i in 1..iterations+1 {
        let mut t_next = HashSet::new();
        let mut v_next = HashSet::new();

        for tile in &to_visit {
            let neighbours = count_neighbours(&tileset, tile.0, tile.1);
            let is_black = tileset.contains(tile);
            let mut should_be_black = false;
            if is_black && (neighbours == 1 || neighbours == 2) {
                should_be_black = true;
            } else if !is_black && neighbours == 2 {
                should_be_black = true;
            }
            if should_be_black {
                t_next.insert((tile.0, tile.1));
                v_next.insert((tile.0, tile.1));
                v_next.insert((tile.0, tile.1-1));
                v_next.insert((tile.0, tile.1+1));
                v_next.insert((tile.0-1, tile.1));
                v_next.insert((tile.0+1, tile.1));
                v_next.insert((tile.0-1, tile.1+1));
                v_next.insert((tile.0+1, tile.1-1));
            }
        }
        tileset = t_next;
        to_visit = v_next;
    }

    tileset.len()
}

/// returns the axial coordinates (q, r) from the string
/// Coordinate system found here: https://www.redblobgames.com/grids/hexagons/
fn get_coordinates(input: &str) -> (i32, i32) {
    let chars: Vec<char> = input.chars().collect();

    let mut q = 0;
    let mut r = 0;
    let mut idx = 0;
    while idx < chars.len() {
        if chars[idx] == 'e' {
            q += 1;
            idx += 1;
        } else if chars[idx] == 'w' {
            q -= 1;
            idx += 1;
        } else if chars[idx] == 'n' {
            if chars[idx+1] == 'e' {
                q += 1;
                r -= 1;
            } else if chars[idx+1] == 'w' {
                r -= 1;
            } else {
                panic!("Unknown char {}", chars[idx+1])
            }
            idx += 2;
        } else if chars[idx] == 's' {
            if chars[idx+1] == 'e' {
                r += 1;
            } else if chars[idx+1] == 'w' {
                q -= 1;
                r += 1;
            } else {
                panic!("Unknown char {}", chars[idx+1])
            }
            idx += 2;
        } else {
            panic!("Unknown char {}", chars[idx])
        }
    }

    (q, r)
}

fn count_neighbours(tiles: &HashSet<(i32, i32)>, q: i32, r: i32) -> usize {
    count_black(tiles, q, r-1)
        + count_black(tiles, q, r+1)
        + count_black(tiles, q-1, r)
        + count_black(tiles, q+1, r)
        + count_black(tiles, q-1, r+1)
        + count_black(tiles, q+1, r-1)
}

fn count_black(tiles: &HashSet<(i32, i32)>, q: i32, r: i32) -> usize {
    if tiles.contains(&(q, r)) {
        1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_result_1() {
        let input = vec![
            "sesenwnenenewseeswwswswwnenewsewsw".to_string(),
            "neeenesenwnwwswnenewnwwsewnenwseswesw".to_string(),
            "seswneswswsenwwnwse".to_string(),
            "nwnwneseeswswnenewneswwnewseswneseene".to_string(),
            "swweswneswnenwsewnwneneseenw".to_string(),
            "eesenwseswswnenwswnwnwsewwnwsene".to_string(),
            "sewnenenenesenwsewnenwwwse".to_string(),
            "wenwwweseeeweswwwnwwe".to_string(),
            "wsweesenenewnwwnwsenewsenwwsesesenwne".to_string(),
            "neeswseenwwswnwswswnw".to_string(),
            "nenwswwsewswnenenewsenwsenwnesesenew".to_string(),
            "enewnwewneswsewnwswenweswnenwsenwsw".to_string(),
            "sweneswneswneneenwnewenewwneswswnese".to_string(),
            "swwesenesewenwneswnwwneseswwne".to_string(),
            "enesenwswwswneneswsenwnewswseenwsese".to_string(),
            "wnwnesenesenenwwnenwsewesewsesesew".to_string(),
            "nenewswnwewswnenesenwnesewesw".to_string(),
            "eneswnwswnwsenenwnwnwwseeswneewsenese".to_string(),
            "neswnwewnwnwseenwseesewsenwsweewe".to_string(),
            "wseweeenwnesenwwwswnew".to_string()
        ];
        assert_eq!(get_result_1(&input), 10);
    }

    #[test]
    fn test_get_result_2() {
        let input = vec![
            "sesenwnenenewseeswwswswwnenewsewsw".to_string(),
            "neeenesenwnwwswnenewnwwsewnenwseswesw".to_string(),
            "seswneswswsenwwnwse".to_string(),
            "nwnwneseeswswnenewneswwnewseswneseene".to_string(),
            "swweswneswnenwsewnwneneseenw".to_string(),
            "eesenwseswswnenwswnwnwsewwnwsene".to_string(),
            "sewnenenenesenwsewnenwwwse".to_string(),
            "wenwwweseeeweswwwnwwe".to_string(),
            "wsweesenenewnwwnwsenewsenwwsesesenwne".to_string(),
            "neeswseenwwswnwswswnw".to_string(),
            "nenwswwsewswnenenewsenwsenwnesesenew".to_string(),
            "enewnwewneswsewnwswenweswnenwsenwsw".to_string(),
            "sweneswneswneneenwnewenewwneswswnese".to_string(),
            "swwesenesewenwneswnwwneseswwne".to_string(),
            "enesenwswwswneneswsenwnewswseenwsese".to_string(),
            "wnwnesenesenenwwnenwsewesewsesesew".to_string(),
            "nenewswnwewswnenesenwnesewesw".to_string(),
            "eneswnwswnwsenenwnwnwwseeswneewsenese".to_string(),
            "neswnwewnwnwseenwseesewsenwsweewe".to_string(),
            "wseweeenwnesenwwwswnew".to_string()
        ];
        assert_eq!(get_result_2(&input, 100), 2208);
    }

    #[test]
    fn test_get_coordinates() {
        assert_eq!(get_coordinates("esew"), (0, 1));
        assert_eq!(get_coordinates("nwwswee"), (0, 0));
    }

}