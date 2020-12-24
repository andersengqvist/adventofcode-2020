use crate::day;
use crate::file;
use std::collections::HashMap;

pub struct Day19 {

}

impl day::Day for Day19 {

    fn puzzle1(&self) {
        println!("Day 19, puzzle 1");

        let result = get_result(&file::lines("res/day19_1.txt"));

        println!("{}", result);
    }

    fn puzzle2(&self) {
        println!("Day 19, puzzle 2");

        let result = get_result(&file::lines("res/day19_2.txt"));

        println!("{}", result);
    }

}

fn get_result(input: &[String]) -> usize {
    let (rs, messages) = split_input(input);
    let rules = build_rules(rs);
    let rule_zero = rules.get(&0).unwrap();
    messages
        .iter()
        .map(|s|
                 if rule_zero.matches_complete(&s.chars().collect::<Vec<char>>(), &rules) {
                     1
                 } else {
                     0
                 }
        )
        .sum()

}

fn split_input(input: &[String]) -> (&[String], &[String]) {
    for i in 0..input.len() {
        if input[i].is_empty() {
            return (&input[0..i], &input[i+1..input.len()]);
        }
    }
    panic!("Empty line not found");
}

trait Rule {
    /// Check this rule, and any subrules, for a partial match of the rule.
    /// Returns list of index where the rule ends if a match
    fn matches_part(&self, input: &[char], idx: usize, rules: &HashMap<u32, Box<dyn Rule>>) -> Vec<usize>;

    /// Check this rule, and any subrules, for a complete match of the rule.
    /// Returns the true if a complete match, false if no match
    fn matches_complete(&self, input: &[char], rules: &HashMap<u32, Box<dyn Rule>>) -> bool;
}

struct CharRule {
    c: char
}

impl CharRule {
    fn new(c: char) -> CharRule {
        CharRule { c }
    }
}

impl Rule for CharRule {
    fn matches_part(&self, input: &[char], idx: usize, _rules: &HashMap<u32, Box<dyn Rule>>) -> Vec<usize> {
        if idx < input.len() && input[idx] == self.c {
            vec!(idx + 1)
        } else {
            vec!()
        }
    }

    fn matches_complete(&self, input: &[char], _rules: &HashMap<u32, Box<dyn Rule>>) -> bool {
        if input.len() == 1 && input[0] == self.c {
            true
        } else {
            false
        }
    }
}

struct ComplexRule {
    rules: Vec<Vec<u32>>
}

impl ComplexRule {
    fn new(rules: Vec<Vec<u32>>) -> ComplexRule {
        ComplexRule { rules }
    }
}

impl Rule for ComplexRule {
    fn matches_part(&self, input: &[char], idx: usize, rules: &HashMap<u32, Box<dyn Rule>>) -> Vec<usize> {
        let mut result: Vec<usize> = vec!();
        for rs in &self.rules {
            let mut indexes = rs.iter()
                .fold(
                    vec!(idx),
                    |idxs, s| {
                        rules
                            .get(s)
                            .map(|rule| {
                                idxs
                                    .iter()
                                    .flat_map(|n_idx| rule.matches_part(input, *n_idx, rules))
                                    .collect()
                            })
                            .unwrap_or(vec!())
                    }

                );
            result.append(indexes.as_mut());
        }

        result
    }

    fn matches_complete(&self, input: &[char], rules: &HashMap<u32, Box<dyn Rule>>) -> bool {
        self.matches_part(input, 0, rules)
            .iter()
            .any(|n_idx| *n_idx == input.len())

    }
}

fn build_rules(input: &[String]) -> HashMap<u32, Box<dyn Rule>> {
    let mut rules: HashMap<u32, Box<dyn Rule>> = HashMap::new();

    for s in input {
        let c_idx = s.find(":").unwrap();
        let rule_num = s[0..c_idx].parse::<u32>().unwrap();

        if s.contains("\"") {
            let f_idx = s.find("\"").unwrap();
            let s_idx = s[f_idx+1..s.len()].find("\"").unwrap();
            let c = s[f_idx+1..s_idx+f_idx+1].chars().next().unwrap();
            rules.insert(rule_num, Box::new(CharRule::new(c)));
        } else {
            let mut ov: Vec<Vec<u32>> = vec![];
            let or_split = s[c_idx+1..s.len()].split('|');
            for or in or_split {
                let mut av: Vec<u32> = vec![];

                let and_split = or.split(' ');
                for and in and_split {
                    let trimmed = and.trim();
                    if trimmed.len() > 0 {
                        let r = trimmed.parse::<u32>().unwrap();
                        av.push(r);
                    }
                }
                ov.push(av);

            }
            rules.insert(rule_num, Box::new(ComplexRule::new(ov)));
        }
    }

    rules
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_rules() {
        let mut rules: HashMap<u32, Box<dyn Rule>> = HashMap::new();
        rules.insert(0, Box::new(ComplexRule::new(vec!(vec!(4,1,5)))));
        rules.insert(1, Box::new(ComplexRule::new(vec!(vec!(2,3),vec!(3,2)))));
        rules.insert(2, Box::new(ComplexRule::new(vec!(vec!(4,4),vec!(5,5)))));
        rules.insert(3, Box::new(ComplexRule::new(vec!(vec!(4,5),vec!(5,4)))));
        rules.insert(4, Box::new(CharRule::new('a')));
        rules.insert(5, Box::new(CharRule::new('b')));

        let rule_zero = rules.get(&0).unwrap();

        assert_eq!(rule_zero.matches_complete(&"ababbb".chars().collect::<Vec<char>>(), &rules), true);
        assert_eq!(rule_zero.matches_complete(&"abbbab".chars().collect::<Vec<char>>(), &rules), true);
        assert_eq!(rule_zero.matches_complete(&"bababa".chars().collect::<Vec<char>>(), &rules), false);
        assert_eq!(rule_zero.matches_complete(&"aaabbb".chars().collect::<Vec<char>>(), &rules), false);
        assert_eq!(rule_zero.matches_complete(&"aaaabbb".chars().collect::<Vec<char>>(), &rules), false);
        assert_eq!(rule_zero.matches_complete(&"ababb".chars().collect::<Vec<char>>(), &rules), false);
    }

    #[test]
    fn test_build_and_run_rules() {
        let rules = build_rules(&vec![
            "0: 4 1 5".to_string(),
            "1: 2 3 | 3 2".to_string(),
            "2: 4 4 | 5 5".to_string(),
            "3: 4 5 | 5 4".to_string(),
            "4: \"a\"".to_string(),
            "5: \"b\"".to_string()
        ]);

        let rule_zero = rules.get(&0).unwrap();

        assert_eq!(rule_zero.matches_complete(&"ababbb".chars().collect::<Vec<char>>(), &rules), true);
        assert_eq!(rule_zero.matches_complete(&"abbbab".chars().collect::<Vec<char>>(), &rules), true);
        assert_eq!(rule_zero.matches_complete(&"bababa".chars().collect::<Vec<char>>(), &rules), false);
        assert_eq!(rule_zero.matches_complete(&"aaabbb".chars().collect::<Vec<char>>(), &rules), false);
        assert_eq!(rule_zero.matches_complete(&"aaaabbb".chars().collect::<Vec<char>>(), &rules), false);
        assert_eq!(rule_zero.matches_complete(&"ababb".chars().collect::<Vec<char>>(), &rules), false);
    }

    #[test]
    fn test_get_result_1() {
        let input = vec![
            "0: 4 1 5".to_string(),
            "1: 2 3 | 3 2".to_string(),
            "2: 4 4 | 5 5".to_string(),
            "3: 4 5 | 5 4".to_string(),
            "4: \"a\"".to_string(),
            "5: \"b\"".to_string(),
            "".to_string(),
            "ababbb".to_string(),
            "bababa".to_string(),
            "abbbab".to_string(),
            "aaabbb".to_string(),
            "aaaabbb".to_string()
        ];
        assert_eq!(get_result(&input), 2);
    }

    #[test]
    fn test_get_result_2() {
        let input = &vec![
            "42: 9 14 | 10 1".to_string(),
            "9: 14 27 | 1 26".to_string(),
            "10: 23 14 | 28 1".to_string(),
            "1: \"a\"".to_string(),
            "11: 42 31".to_string(),
            "5: 1 14 | 15 1".to_string(),
            "19: 14 1 | 14 14".to_string(),
            "12: 24 14 | 19 1".to_string(),
            "16: 15 1 | 14 14".to_string(),
            "31: 14 17 | 1 13".to_string(),
            "6: 14 14 | 1 14".to_string(),
            "2: 1 24 | 14 4".to_string(),
            "0: 8 11".to_string(),
            "13: 14 3 | 1 12".to_string(),
            "15: 1 | 14".to_string(),
            "17: 14 2 | 1 7".to_string(),
            "23: 25 1 | 22 14".to_string(),
            "28: 16 1".to_string(),
            "4: 1 1".to_string(),
            "20: 14 14 | 1 15".to_string(),
            "3: 5 14 | 16 1".to_string(),
            "27: 1 6 | 14 18".to_string(),
            "14: \"b\"".to_string(),
            "21: 14 1 | 1 14".to_string(),
            "25: 1 1 | 1 14".to_string(),
            "22: 14 14".to_string(),
            "8: 42".to_string(),
            "26: 14 22 | 1 20".to_string(),
            "18: 15 15".to_string(),
            "7: 14 5 | 1 21".to_string(),
            "24: 14 1".to_string(),
            "".to_string(),
            "abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa".to_string(),
            "bbabbbbaabaabba".to_string(),
            "babbbbaabbbbbabbbbbbaabaaabaaa".to_string(),
            "aaabbbbbbaaaabaababaabababbabaaabbababababaaa".to_string(),
            "bbbbbbbaaaabbbbaaabbabaaa".to_string(),
            "bbbababbbbaaaaaaaabbababaaababaabab".to_string(),
            "ababaaaaaabaaab".to_string(),
            "ababaaaaabbbaba".to_string(),
            "baabbaaaabbaaaababbaababb".to_string(),
            "abbbbabbbbaaaababbbbbbaaaababb".to_string(),
            "aaaaabbaabaaaaababaa".to_string(),
            "aaaabbaaaabbaaa".to_string(),
            "aaaabbaabbaaaaaaabbbabbbaaabbaabaaa".to_string(),
            "babaaabbbaaabaababbaabababaaab".to_string(),
            "aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba".to_string(),
        ];
        assert_eq!(get_result(&input), 3);
    }

    #[test]
    fn test_get_result_3() {
        let input = &vec![
            "42: 9 14 | 10 1".to_string(),
            "9: 14 27 | 1 26".to_string(),
            "10: 23 14 | 28 1".to_string(),
            "1: \"a\"".to_string(),
            "11: 42 31 | 42 11 31".to_string(),
            "5: 1 14 | 15 1".to_string(),
            "19: 14 1 | 14 14".to_string(),
            "12: 24 14 | 19 1".to_string(),
            "16: 15 1 | 14 14".to_string(),
            "31: 14 17 | 1 13".to_string(),
            "6: 14 14 | 1 14".to_string(),
            "2: 1 24 | 14 4".to_string(),
            "0: 8 11".to_string(),
            "13: 14 3 | 1 12".to_string(),
            "15: 1 | 14".to_string(),
            "17: 14 2 | 1 7".to_string(),
            "23: 25 1 | 22 14".to_string(),
            "28: 16 1".to_string(),
            "4: 1 1".to_string(),
            "20: 14 14 | 1 15".to_string(),
            "3: 5 14 | 16 1".to_string(),
            "27: 1 6 | 14 18".to_string(),
            "14: \"b\"".to_string(),
            "21: 14 1 | 1 14".to_string(),
            "25: 1 1 | 1 14".to_string(),
            "22: 14 14".to_string(),
            "8: 42 | 42 8".to_string(),
            "26: 14 22 | 1 20".to_string(),
            "18: 15 15".to_string(),
            "7: 14 5 | 1 21".to_string(),
            "24: 14 1".to_string(),
            "".to_string(),
            "abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa".to_string(),
            "bbabbbbaabaabba".to_string(),
            "babbbbaabbbbbabbbbbbaabaaabaaa".to_string(),
            "aaabbbbbbaaaabaababaabababbabaaabbababababaaa".to_string(),
            "bbbbbbbaaaabbbbaaabbabaaa".to_string(),
            "bbbababbbbaaaaaaaabbababaaababaabab".to_string(),
            "ababaaaaaabaaab".to_string(),
            "ababaaaaabbbaba".to_string(),
            "baabbaaaabbaaaababbaababb".to_string(),
            "abbbbabbbbaaaababbbbbbaaaababb".to_string(),
            "aaaaabbaabaaaaababaa".to_string(),
            "aaaabbaaaabbaaa".to_string(),
            "aaaabbaabbaaaaaaabbbabbbaaabbaabaaa".to_string(),
            "babaaabbbaaabaababbaabababaaab".to_string(),
            "aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba".to_string(),
        ];
        assert_eq!(get_result(&input), 12);
    }

    #[test]
    fn test_get_result_4() {
        let input = vec![
            "0: 1 2".to_string(),
            "1: 3 | 3 1".to_string(),
            "2: 4 | 4 2".to_string(),
            "3: \"a\"".to_string(),
            "4: \"b\"".to_string(),
            "".to_string(),
            "ab".to_string(),
            "aab".to_string(),
            "abb".to_string(),
            "aaaaaaaaaaabbbbbbbbbbb".to_string(),
            "aabbaaaaaa".to_string(),
            "ba".to_string(),
            "aaaaaaaabbbbbbbaaaaaaabbbbb".to_string()
        ];
        assert_eq!(get_result(&input), 4);
    }

}