use crate::day;
use crate::file;
use std::collections::HashSet;

pub struct Day6 {

}

impl day::Day for Day6 {

    fn puzzle1(&self) {
        println!("Day 6, puzzle 1");

        let groups = build_groups(&file::lines("res/day6_1.txt"));
        let yes = count_yes_1(&groups);

        println!("{}", yes);
    }

    fn puzzle2(&self) {
        println!("Day 6, puzzle 2");

        let groups = build_groups(&file::lines("res/day6_1.txt"));
        let yes = count_yes_2(&groups);

        println!("{}", yes);
    }

}

pub struct Group {
    answers: Vec<String>
}

impl Group {

    fn new() -> Group {
        Group { answers: Vec::new() }
    }

    fn add_answer(&mut self, answer: &str) {
        self.answers.push(answer.to_string())
    }

    fn count_yes_1(&self) -> usize {
        let a: HashSet<char> = self.answers
            .iter()
            .flat_map(|answer| answer.chars())
            .collect();

        a.len()
    }

    fn count_yes_2(&self) -> usize {
        let a: Option<HashSet<char>> = self.answers
            .iter()
            .map(|answer| answer.chars().collect::<HashSet<char>>())
            .fold(
                Option::None,
                |h1, h2| {
                    if h1.is_none() {
                        Option::Some(h2)
                    } else {
                        Option::Some(h1.unwrap().intersection(&h2).map(|&c| c).collect::<HashSet<char>>())
                    }
                }
            );

        a.unwrap().len()
    }

}

fn build_groups(l: &Vec<String>) -> Vec<Group> {
    let mut groups: Vec<Group> = Vec::new();
    groups.push(Group::new());

    for str in l {
        if str.trim().is_empty() {
            groups.push(Group::new());
        } else {
            let g: &mut Group = groups.last_mut().unwrap();
            g.add_answer(str);
        }
    }

    groups
}

fn count_yes_1(g: &Vec<Group>) -> usize {
    g.iter().map(|g| g.count_yes_1()).sum()
}

fn count_yes_2(g: &Vec<Group>) -> usize {
    g.iter().map(|g| g.count_yes_2()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_yes_1() {
        let groups = build_groups(
            &vec![
                "abc".to_string(),
                "".to_string(),
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
                "".to_string(),
                "ab".to_string(),
                "ac".to_string(),
                "".to_string(),
                "a".to_string(),
                "a".to_string(),
                "a".to_string(),
                "a".to_string(),
                "".to_string(),
                "b".to_string()
        ]);
        assert_eq!(count_yes_1(&groups), 11);
    }

    #[test]
    fn test_count_yes_2() {
        let groups = build_groups(
            &vec![
                "abc".to_string(),
                "".to_string(),
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
                "".to_string(),
                "ab".to_string(),
                "ac".to_string(),
                "".to_string(),
                "a".to_string(),
                "a".to_string(),
                "a".to_string(),
                "a".to_string(),
                "".to_string(),
                "b".to_string()
            ]);
        assert_eq!(count_yes_2(&groups), 6);
    }

}