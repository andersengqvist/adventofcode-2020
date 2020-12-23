use crate::day;
use crate::file;

pub struct Day18 {

}

impl day::Day for Day18 {

    fn puzzle1(&self) {
        println!("Day 18, puzzle 1");

        let result = get_result_1(&file::lines("res/day18_1.txt"));

        println!("{}", result);
    }

    fn puzzle2(&self) {
        println!("Day 18, puzzle 2");

        let result = get_result_2(&file::lines("res/day18_1.txt"));

        println!("{}", result);
    }

}

fn get_result_1(input: &Vec<String>) -> i64 {
    input
        .iter()
        .map(|s| evaluate_expression_str_1(s))
        .sum()
}

fn get_result_2(input: &Vec<String>) -> i64 {
    input
        .iter()
        .map(|s| evaluate_expression_str_2(s))
        .sum()
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
enum State {
    OPERATOR,
    RIGHT,
    END
}

fn evaluate_expression_str_1(exp: &str) -> i64 {
    evaluate_expression_1(&exp.chars().collect::<Vec<char>>())
}

fn evaluate_expression_1(exp: &[char]) -> i64 {
    //println!("Expression: {:?}", exp);
    let mut result: i64;
    let mut idx: usize = 0;

    // first find the first term
    while exp[idx] == ' ' {
        idx += 1;
    }
    if exp[idx] == '(' {
        let c_idx = find_closing_parenthesis(exp, idx);
        result = evaluate_expression_1(&exp[idx+1..c_idx]);
        idx = c_idx + 1;
    } else {
        let (l, new_idx) = parse_number(exp, idx);
        result = l;
        idx = new_idx;
    }

    // Then check the rest of the terms
    let mut state = State::OPERATOR;
    let mut operator: char = ' ';
    let mut right: i64 = 0;
    while idx < exp.len() {
        if state == State::OPERATOR {
            if exp[idx] == ' ' {
                idx += 1;
            } else {
                operator = exp[idx];
                idx += 1;
                state = State::RIGHT;
            }
        }
        if state == State::RIGHT {
            if exp[idx] == ' ' {
                idx += 1;
            } else if exp[idx] == '(' {
                let cp_idx = find_closing_parenthesis(exp, idx);
                right = evaluate_expression_1(&exp[idx+1..cp_idx]);
                idx = cp_idx + 1;
                state = State::END;
            } else {
                let (l, new_idx) = parse_number(exp, idx);
                right = l;
                idx = new_idx;
                state = State::END;
            }
        }
        if state == State::END {
            //println!("Evaluating: {} {} {}", result, operator, right);
            match operator {
                '*' => result *= right,
                '+' => result += right,
                _ => panic!("Unknown operator {}", operator)
            }
            state = State::OPERATOR;
        }
    }

    result
}

fn evaluate_expression_str_2(exp: &str) -> i64 {
    evaluate_expression_2(&exp.chars().collect::<Vec<char>>())
}

fn evaluate_expression_2(exp: &[char]) -> i64 {
    //println!("Expression: {:?}", exp);
    let mut result: i64;
    let mut idx: usize = 0;

    // first find the first term
    while exp[idx] == ' ' {
        idx += 1;
    }
    if exp[idx] == '(' {
        let c_idx = find_closing_parenthesis(exp, idx);
        result = evaluate_expression_2(&exp[idx+1..c_idx]);
        idx = c_idx + 1;
    } else {
        let (l, new_idx) = parse_number(exp, idx);
        result = l;
        idx = new_idx;
    }

    // Then check the rest of the terms
    let mut state = State::OPERATOR;
    let mut operator: char = ' ';
    let mut right: i64 = 0;
    while idx < exp.len() {
        if state == State::OPERATOR {
            if exp[idx] == ' ' {
                idx += 1;
            } else {
                operator = exp[idx];
                idx += 1;
                state = State::RIGHT;
            }
        }
        if state == State::RIGHT {
            if operator == '*' {
                right = evaluate_expression_2(&exp[idx..exp.len()]);
                idx = exp.len();
                state = State::END;
            } else if exp[idx] == ' ' {
                idx += 1;
            } else if exp[idx] == '(' {
                let cp_idx = find_closing_parenthesis(exp, idx);
                right = evaluate_expression_2(&exp[idx+1..cp_idx]);
                idx = cp_idx + 1;
                state = State::END;
            } else {
                let (l, new_idx) = parse_number(exp, idx);
                right = l;
                idx = new_idx;
                state = State::END;
            }
        }
        if state == State::END {
            //println!("Evaluating: {} {} {}", result, operator, right);
            match operator {
                '*' => result *= right,
                '+' => result += right,
                _ => panic!("{} is not a valid operator", operator)
            }
            state = State::OPERATOR;
        }
    }

    result
}

// Finds the closing parenthesis, expecting char at at_idx to be an opening parenthesis
// If no matching is found, index out of bound is returned
fn find_closing_parenthesis(exp: &[char], at_idx: usize) -> usize {
    let mut idx = at_idx + 1;
    let mut num_open = 1;
    while idx < exp.len() && num_open > 0 {
        if exp[idx] == '(' {
            num_open += 1;
        } else if exp[idx] == ')' {
            num_open -= 1;
        }
        if num_open > 0 {
            idx += 1;
        }
    }
    idx
}

fn parse_number(exp: &[char], at_idx: usize) -> (i64, usize) {
    let mut idx = at_idx;
    let mut negative = false;
    if exp[idx] == '-' {
        negative = true;
        idx += 1;
    }

    if !is_number(exp[idx]) {
        panic!("{} is not a number", exp[idx]);
    }

    let mut num = get_number(exp[idx]);
    idx += 1;
    while idx < exp.len() && is_number(exp[idx]) {
        num *= 10;
        num += get_number(exp[idx]);
        idx += 1;
    }

    if negative {
        num = -num;
    }
    (num, idx)
}

fn is_number(c: char) -> bool {
    let n = c as u32;
    n >=48 && n <= 57
}

fn get_number(c: char) -> i64 {
    c as i64 - 48
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_number() {
        assert_eq!(parse_number(&vec!['2'],0), (2, 1));
        assert_eq!(parse_number(&vec!['-','2'],0), (-2, 2));
        assert_eq!(parse_number(&vec!['h','e','l','l','o','4','2','4','2','w','o','r','l','d'],5), (4242, 9));
        assert_eq!(parse_number(&vec!['h','3','1','l','o','4','2','h','2','w','0','-','1','d'],1), (31, 3));
        assert_eq!(parse_number(&vec!['h','3','1','l','o','4','2','h','2','w','0','-','1','d'],5), (42, 7));
        assert_eq!(parse_number(&vec!['h','3','1','l','o','4','2','h','2','w','0','-','1','d'],8), (2, 9));
        assert_eq!(parse_number(&vec!['h','3','1','l','o','4','2','h','2','w','0','-','1','d'],10), (0, 11));
        assert_eq!(parse_number(&vec!['h','3','1','l','o','4','2','h','2','w','0','-','1','d'],11), (-1, 13));
    }

    #[test]
    fn test_find_closing_parenthesis() {
        assert_eq!(find_closing_parenthesis(&vec!['(','h','e','l','l','o',')','(','w','o','r','l','d',')'],0), 6);
        assert_eq!(find_closing_parenthesis(&vec!['(','h','e','l','l','o',')','(','w','o','r','l','d',')'],7), 13);
        assert_eq!(find_closing_parenthesis(&vec!['(','(','e',')','(',')',')','(','w','(','r',')','d',')'],0), 6);
        assert_eq!(find_closing_parenthesis(&vec!['(','(','e',')','(',')',')','(','w','(','r',')','d',')'],1), 3);
        assert_eq!(find_closing_parenthesis(&vec!['(','(','e',')','(',')',')','(','w','(','r',')','d',')'],4), 5);
        assert_eq!(find_closing_parenthesis(&vec!['(','(','e',')','(',')',')','(','w','(','r',')','d',')'],4), 5);
        assert_eq!(find_closing_parenthesis(&vec!['(','(','e',')','(',')',')','(','w','(','r',')','d',')'],7), 13);
        assert_eq!(find_closing_parenthesis(&vec!['(','(','e',')','(',')',')','(','w','(','r',')','d',')'],9), 11);
        assert_eq!(find_closing_parenthesis(&vec!['(','(','e',')','(','o',')','(','w','(','r',')','d',')'],0), 14);
    }

    #[test]
    fn test_evaluate_expression_str_1() {
        assert_eq!(evaluate_expression_str_1(&"1 + 2 * 3 + 4 * 5 + 6".to_string()), 71);
        assert_eq!(evaluate_expression_str_1(&"1 + (2 * 3) + (4 * (5 + 6))".to_string()), 51);
        assert_eq!(evaluate_expression_str_1(&"2 * 3 + (4 * 5)".to_string()), 26);
        assert_eq!(evaluate_expression_str_1(&"5 + (8 * 3 + 9 + 3 * 4 * 3)".to_string()), 437);
        assert_eq!(evaluate_expression_str_1(&"5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".to_string()), 12240);
        assert_eq!(evaluate_expression_str_1(&"((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".to_string()), 13632);
    }

    #[test]
    fn test_evaluate_expression_str_2() {
        assert_eq!(evaluate_expression_str_2(&"1 + 2 * 3 + 4 * 5 + 6".to_string()), 231);
        assert_eq!(evaluate_expression_str_2(&"1 + (2 * 3) + (4 * (5 + 6))".to_string()), 51);
        assert_eq!(evaluate_expression_str_2(&"2 * 3 + (4 * 5)".to_string()), 46);
        assert_eq!(evaluate_expression_str_2(&"5 + (8 * 3 + 9 + 3 * 4 * 3)".to_string()), 1445);
        assert_eq!(evaluate_expression_str_2(&"5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".to_string()), 669060);
        assert_eq!(evaluate_expression_str_2(&"((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".to_string()), 23340);
    }
}
