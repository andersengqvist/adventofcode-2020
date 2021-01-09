use crate::day;

pub struct Day25 {

}

impl day::Day for Day25 {

    fn puzzle1(&self) {
        println!("Day 25, puzzle 1");

        let result = get_result_1(12092626, 4707356);

        println!("{}", result);
    }

    fn puzzle2(&self) {
        println!("Day 25, puzzle 2");

        println!("Wohoo!!!");
    }

}

fn get_result_1(pub_key1: u64, pub_key2: u64) -> u64 {
    let loop_size1 =  guess_loop_size(7, pub_key1);
    let loop_size2 =  guess_loop_size(7, pub_key2);

    let priv_key1 = transform_subject_number(pub_key1, loop_size2);
    let priv_key2 = transform_subject_number(pub_key2, loop_size1);

    assert_eq!(priv_key1, priv_key2);

    priv_key1
}

fn transform_subject_number(subject_number: u64, loop_size: usize) -> u64 {
    let mut value = 1;
    for _l in 0..loop_size {
        value = transform_step(value, subject_number);
    }
    value
}

// Runs one step, returns the new value
fn transform_step(value: u64, subject_number: u64) -> u64 {
    let v = value * subject_number;
    v % 20201227
}

fn guess_loop_size(subject_number: u64, public_key: u64) -> usize {
    let mut value: u64 = 1;
    for l in 0..100000000 {
        if value == public_key {
            return l;
        }
        value = transform_step(value, subject_number);
    }
    panic!("Loop size not found");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_result_1() {
        assert_eq!(get_result_1(5764801, 17807724), 14897079);
    }

    #[test]
    fn test_transform_subject_number() {
        assert_eq!(transform_subject_number(7, 8), 5764801);
        assert_eq!(transform_subject_number(7, 11), 17807724);
        assert_eq!(transform_subject_number(17807724, 8), 14897079);
        assert_eq!(transform_subject_number(5764801, 11), 14897079);
    }

    #[test]
    fn test_guess_loop_size() {
        assert_eq!(guess_loop_size(7, 5764801), 8);
        assert_eq!(guess_loop_size(7, 17807724), 11);
    }
}