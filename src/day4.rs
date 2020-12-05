use crate::day;
use crate::file;
use std::collections::HashMap;
use lazy_static::lazy_static;
use regex::Regex;

pub struct Day4 {

}

struct StringCollector {
    vec: Vec<String>,
    str: String
}

impl StringCollector {
    fn new() -> StringCollector {
        StringCollector { vec: Vec::new(), str: String::new() }
    }

    fn add(&mut self, s: &str) {
        if s.trim().is_empty() {
            self.vec.push(self.str.clone());
            self.str.clear();
        } else {
            if self.str.trim().is_empty() {
                self.str.push_str(s);
            } else {
                self.str.push_str(&format!(" {}", s));
            }
        }
    }

    fn finalize(&mut self) -> Vec<String> {
        if !self.str.trim().is_empty() {
            self.vec.push(self.str.clone());
            self.str.clear();
        }
        self.vec.clone()
    }

}

fn group_passports(vec: &Vec<String>) -> Vec<String> {
    let mut string_collector = StringCollector::new();
    for str in vec {
        string_collector.add(str);
    }
    string_collector.finalize()
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Passport {
    byr: Option<String>, // (Birth Year)
    iyr: Option<String>, // (Issue Year)
    eyr: Option<String>, // (Expiration Year)
    hgt: Option<String>, // (Height)
    hcl: Option<String>, // (Hair Color)
    ecl: Option<String>, // (Eye Color)
    pid: Option<String>, // (Passport ID)
    cid: Option<String>, // (Country ID)
}

impl Passport {

    fn from_str(passport_str: &str) -> Passport {

        let map: HashMap<&str, &str> =
            passport_str
                .split_whitespace()
                .filter_map(|l| match l.split(":").collect::<Vec<&str>>()[..] {
                    [first, second] => Option::Some((first, second)),
                    _ => Option::None,
                })
                .collect();

        Passport {
            byr: map.get("byr").map(|s| s.to_string()), // (Birth Year)
            iyr: map.get("iyr").map(|s| s.to_string()), // (Issue Year)
            eyr: map.get("eyr").map(|s| s.to_string()), // (Expiration Year)
            hgt: map.get("hgt").map(|s| s.to_string()), // (Height)
            hcl: map.get("hcl").map(|s| s.to_string()), // (Hair Color)
            ecl: map.get("ecl").map(|s| s.to_string()), // (Eye Color)
            pid: map.get("pid").map(|s| s.to_string()), // (Passport ID)
            cid: map.get("cid").map(|s| s.to_string()), // (Country ID)
        }
    }

    fn is_valid_1(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    fn is_valid_2(&self) -> bool {
        is_valid_byr(&self.byr)
            && is_valid_iyr(&self.iyr)
            && is_valid_eyr(&self.eyr)
            && is_valid_hgt(&self.hgt)
            && is_valid_hcl(&self.hcl)
            && is_valid_ecl(&self.ecl)
            && is_valid_pid(&self.pid)
    }
}

// byr (Birth Year) - four digits; at least 1920 and at most 2002.
fn is_valid_byr(byr: &Option<String>) ->  bool {
    byr
        .as_ref()
        .and_then(|s| s.parse::<u16>().ok().filter(|&n| n >= 1920 && n <= 2002))
        .is_some()
}

// iyr (Issue Year) - four digits; at least 2010 and at most 2020.
fn is_valid_iyr(iyr: &Option<String>) ->  bool {
    iyr
        .as_ref()
        .and_then(|s| s.parse::<u16>().ok().filter(|&n| n >= 2010 && n <= 2020))
        .is_some()
}

// eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
fn is_valid_eyr(eyr: &Option<String>) ->  bool {
    eyr
        .as_ref()
        .and_then(|s| s.parse::<u16>().ok().filter(|&n| n >= 2020 && n <= 2030))
        .is_some()
}

// hgt (Height) - a number followed by either cm or in:
// If cm, the number must be at least 150 and at most 193.
// If in, the number must be at least 59 and at most 76.
fn is_valid_hgt(hgt: &Option<String>) ->  bool {
    hgt
        .as_ref()
        .filter(|s| -> bool {
            if s.ends_with("cm") {
                s[0..s.len()-2].parse::<u16>().ok().filter(|&n| n >= 150 && n <= 193).is_some()
            } else if s.ends_with("in") {
                s[0..s.len()-2].parse::<u16>().ok().filter(|&n| n >= 59 && n <= 76).is_some()
            } else {
                false
            }
        })
        .is_some()
}

lazy_static! {
    static ref RE_HCL: Regex = Regex::new("#[0-9a-f]{6}$").unwrap();
}

// hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
fn is_valid_hcl(hcl: &Option<String>) -> bool {
    hcl
        .as_ref()
        .filter(|&s| -> bool {
            RE_HCL.is_match(s)
        })
        .is_some()
}

// ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
fn is_valid_ecl(ecl: &Option<String>) ->  bool {
    ecl
        .as_ref()
        .filter(|&s|
            s == &"amb".to_string()
                || s == &"blu".to_string()
                || s == &"brn".to_string()
                || s == &"gry".to_string()
                || s == &"grn".to_string()
                || s == &"hzl".to_string()
                || s == &"oth".to_string()
        )
        .is_some()
}

lazy_static! {
    static ref RE_PID: Regex = Regex::new("^\\d{9}$").unwrap();
}

// pid (Passport ID) - a nine-digit number, including leading zeroes.
fn is_valid_pid(pid: &Option<String>) -> bool {
    pid
        .as_ref()
        .filter(|&s| -> bool {
            RE_PID.is_match(s)
        })
        .is_some()
}


fn build_passports(vec: &Vec<String>) -> Vec<Passport> {
    vec
        .iter()
        .map( |s| Passport::from_str(s))
        .collect()
}

fn count_valid_passports_1(l: &Vec<String>) -> usize {
    let l2 = group_passports(l);
    let passports = build_passports(&l2);
    passports
        .iter()
        .filter(|p| p.is_valid_1())
        .count()
}

fn count_valid_passports_2(l: &Vec<String>) -> usize {
    let l2 = group_passports(l);
    let passports = build_passports(&l2);
    passports
        .iter()
        .filter(|p| p.is_valid_2())
        .count()
}

impl day::Day for Day4 {

    fn puzzle1(&self) {
        println!("Day 4, puzzle 1");

        let valid = count_valid_passports_1(&file::lines("res/day4_1.txt"));

        println!("{}", valid);
    }

    fn puzzle2(&self) {
        println!("Day 4, puzzle 2");

        let valid = count_valid_passports_2(&file::lines("res/day4_1.txt"));

        println!("{}", valid);
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_string() {
        let mut string_collector = StringCollector::new();
        string_collector.add(&"hello");
        string_collector.add(&"world");
        let v = string_collector.finalize();

        assert_eq!(
            vec!["hello world"],
            v
        );
    }

    #[test]
    fn test_group_passports() {
        let v = vec![
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd".to_string(),
            "byr:1937 iyr:2017 cid:147 hgt:183cm".to_string(),
            "".to_string(),
            "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884".to_string(),
            "hcl:#cfa07d byr:1929".to_string(),
            "".to_string(),
            "hcl:#ae17e1 iyr:2013".to_string(),
            "eyr:2024".to_string(),
            "ecl:brn pid:760753108 byr:1931".to_string(),
            "hgt:179cm".to_string(),
            "".to_string(),
            "hcl:#cfa07d eyr:2025 pid:166559648".to_string(),
            "iyr:2011 ecl:brn hgt:59in".to_string()
        ];

        let v2 = group_passports(&v);

        assert_eq!(
            vec![
                "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm".to_string(),
                "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884 hcl:#cfa07d byr:1929".to_string(),
                "hcl:#ae17e1 iyr:2013 eyr:2024 ecl:brn pid:760753108 byr:1931 hgt:179cm".to_string(),
                "hcl:#cfa07d eyr:2025 pid:166559648 iyr:2011 ecl:brn hgt:59in".to_string()
            ],
            v2
        );
    }

    #[test]
    fn test_create_passports() {
        let p = Passport::from_str("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm");
        assert_eq!(
            p,
            Passport {
                byr: Option::Some("1937".to_string()), // (Birth Year)
                iyr: Option::Some("2017".to_string()), // (Issue Year)
                eyr: Option::Some("2020".to_string()), // (Expiration Year)
                hgt: Option::Some("183cm".to_string()), // (Height)
                hcl: Option::Some("#fffffd".to_string()), // (Hair Color)
                ecl: Option::Some("gry".to_string()), // (Eye Color)
                pid: Option::Some("860033327".to_string()), // (Passport ID)
                cid: Option::Some("147".to_string()), // (Country ID)
            }
        );
    }

    #[test]
    fn test_build_passports() {
        assert_eq!(
            build_passports(
            &vec![
                    "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm".to_string(),
                    "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884 hcl:#cfa07d byr:1929".to_string(),
                    "hcl:#ae17e1 iyr:2013 eyr:2024 ecl:brn pid:760753108 byr:1931 hgt:179cm".to_string(),
                    "hcl:#cfa07d eyr:2025 pid:166559648 iyr:2011 ecl:brn hgt:59in".to_string()
                ]
            ),
            vec![
                Passport {
                    byr: Option::Some("1937".to_string()), // (Birth Year)
                    iyr: Option::Some("2017".to_string()), // (Issue Year)
                    eyr: Option::Some("2020".to_string()), // (Expiration Year)
                    hgt: Option::Some("183cm".to_string()), // (Height)
                    hcl: Option::Some("#fffffd".to_string()), // (Hair Color)
                    ecl: Option::Some("gry".to_string()), // (Eye Color)
                    pid: Option::Some("860033327".to_string()), // (Passport ID)
                    cid: Option::Some("147".to_string()), // (Country ID)
                },
                Passport {
                    byr: Option::Some("1929".to_string()), // (Birth Year)
                    iyr: Option::Some("2013".to_string()), // (Issue Year)
                    eyr: Option::Some("2023".to_string()), // (Expiration Year)
                    hgt: Option::None, // (Height)
                    hcl: Option::Some("#cfa07d".to_string()), // (Hair Color)
                    ecl: Option::Some("amb".to_string()), // (Eye Color)
                    pid: Option::Some("028048884".to_string()), // (Passport ID)
                    cid: Option::Some("350".to_string()), // (Country ID)
                },
                Passport {
                    byr: Option::Some("1931".to_string()), // (Birth Year)
                    iyr: Option::Some("2013".to_string()), // (Issue Year)
                    eyr: Option::Some("2024".to_string()), // (Expiration Year)
                    hgt: Option::Some("179cm".to_string()), // (Height)
                    hcl: Option::Some("#ae17e1".to_string()), // (Hair Color)
                    ecl: Option::Some("brn".to_string()), // (Eye Color)
                    pid: Option::Some("760753108".to_string()), // (Passport ID)
                    cid: Option::None, // (Country ID)
                },
                Passport {
                    byr: Option::None, // (Birth Year)
                    iyr: Option::Some("2011".to_string()), // (Issue Year)
                    eyr: Option::Some("2025".to_string()), // (Expiration Year)
                    hgt: Option::Some("59in".to_string()), // (Height)
                    hcl: Option::Some("#cfa07d".to_string()), // (Hair Color)
                    ecl: Option::Some("brn".to_string()), // (Eye Color)
                    pid: Option::Some("166559648".to_string()), // (Passport ID)
                    cid: Option::None, // (Country ID)
                }
            ]
        );
    }

    #[test]
    fn test_is_passport_valid_1() {
        assert_eq!(
            Passport {
                byr: Option::Some("1937".to_string()), // (Birth Year)
                iyr: Option::Some("2017".to_string()), // (Issue Year)
                eyr: Option::Some("2020".to_string()), // (Expiration Year)
                hgt: Option::Some("183cm".to_string()), // (Height)
                hcl: Option::Some("#fffffd".to_string()), // (Hair Color)
                ecl: Option::Some("gry".to_string()), // (Eye Color)
                pid: Option::Some("860033327".to_string()), // (Passport ID)
                cid: Option::Some("147".to_string()), // (Country ID)
            }.is_valid_1(),
            true
        );
        assert_eq!(
            Passport {
                byr: Option::Some("1929".to_string()), // (Birth Year)
                iyr: Option::Some("2013".to_string()), // (Issue Year)
                eyr: Option::Some("2023".to_string()), // (Expiration Year)
                hgt: Option::None, // (Height)
                hcl: Option::Some("#cfa07d".to_string()), // (Hair Color)
                ecl: Option::Some("amb".to_string()), // (Eye Color)
                pid: Option::Some("028048884".to_string()), // (Passport ID)
                cid: Option::Some("350".to_string()), // (Country ID)
            }.is_valid_1(),
            false
        );
        assert_eq!(
            Passport {
                byr: Option::Some("1931".to_string()), // (Birth Year)
                iyr: Option::Some("2013".to_string()), // (Issue Year)
                eyr: Option::Some("2024".to_string()), // (Expiration Year)
                hgt: Option::Some("179cm".to_string()), // (Height)
                hcl: Option::Some("#ae17e1".to_string()), // (Hair Color)
                ecl: Option::Some("brn".to_string()), // (Eye Color)
                pid: Option::Some("760753108".to_string()), // (Passport ID)
                cid: Option::None, // (Country ID)
            }.is_valid_1(),
            true
        );
        assert_eq!(
            Passport {
                byr: Option::None, // (Birth Year)
                iyr: Option::Some("2011".to_string()), // (Issue Year)
                eyr: Option::Some("2025".to_string()), // (Expiration Year)
                hgt: Option::Some("59in".to_string()), // (Height)
                hcl: Option::Some("#cfa07d".to_string()), // (Hair Color)
                ecl: Option::Some("brn".to_string()), // (Eye Color)
                pid: Option::Some("166559648".to_string()), // (Passport ID)
                cid: Option::None, // (Country ID)
            }.is_valid_1(),
            false
        );
    }

    #[test]
    fn test_is_fields_valid() {
        assert_eq!(
            is_valid_byr(&Option::Some("2002".to_string())),
            true
        );
        assert_eq!(
            is_valid_byr(&Option::Some("2003".to_string())),
            false
        );

        assert_eq!(
            is_valid_hgt(&Option::Some("60in".to_string())),
            true
        );
        assert_eq!(
            is_valid_hgt(&Option::Some("190cm".to_string())),
            true
        );
        assert_eq!(
            is_valid_hgt(&Option::Some("190in".to_string())),
            false
        );
        assert_eq!(
            is_valid_hgt(&Option::Some("190".to_string())),
            false
        );

        assert_eq!(
            is_valid_hcl(&Option::Some("#123abc".to_string())),
            true
        );
        assert_eq!(
            is_valid_hcl(&Option::Some("#123abz".to_string())),
            false
        );
        assert_eq!(
            is_valid_hcl(&Option::Some("123abc".to_string())),
            false
        );

        assert_eq!(
            is_valid_ecl(&Option::Some("brn".to_string())),
            true
        );
        assert_eq!(
            is_valid_ecl(&Option::Some("wat".to_string())),
            false
        );
        assert_eq!(
            is_valid_pid(&Option::Some("000000001".to_string())),
            true
        );
        assert_eq!(
            is_valid_pid(&Option::Some("0123456789".to_string())),
            false
        );
    }

    #[test]
    fn test_is_passport_valid_2() {
        assert_eq!(
            Passport::from_str(" pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030  byr:1980 hcl:#623a2f ").is_valid_2(),
            true
        );
        assert_eq!(
            Passport::from_str("eyr:2029 ecl:blu  cid:129 byr:1989 iyr:2014 pid:896056539 hcl:#a97842  hgt:165cm").is_valid_2(),
            true
        );
        assert_eq!(
            Passport::from_str("hcl:#888785 hgt:164cm byr:2001 iyr:2015 cid:88 pid:545766238 ecl:hzl  eyr:2022 ").is_valid_2(),
            true
        );
        assert_eq!(
            Passport::from_str("iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719").is_valid_2(),
            true
        );

        assert_eq!(
            Passport::from_str("eyr:1972 cid:100 hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926").is_valid_2(),
            false
        );
        assert_eq!(
            Passport::from_str("iyr:2019 hcl:#602927 eyr:1967 hgt:170cm ecl:grn pid:012533040 byr:1946").is_valid_2(),
            false
        );
        assert_eq!(
            Passport::from_str("hcl:dab227 iyr:2012 ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277").is_valid_2(),
            false
        );
        assert_eq!(
            Passport::from_str("hgt:59cm ecl:zzz eyr:2038 hcl:74454a iyr:2023 pid:3556412378 byr:2007").is_valid_2(),
            false
        );
    }

}