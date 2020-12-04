use crate::puzzle::AbstractPuzzle;
use std::collections::HashMap;
use regex::Regex;

pub struct Puzzle04 {
    input: String,
}

impl AbstractPuzzle for Puzzle04 {
    fn get_day(&self) -> u8 {
        4
    }

    fn solve_part_1(&self) -> String {
        self.input.split("\n\n")
            .map(|passport| parse_passport(passport))
            .filter(|passport| has_required_fields(passport))
            .count()
            .to_string()
    }

    fn solve_part_2(&self) -> String {
        self.input.split("\n\n")
            .map(|passport| parse_passport(passport))
            .filter(|passport| has_required_fields(passport) && all_values_valid(passport))
            .count()
            .to_string()
    }
}

impl Puzzle04 {
    pub fn create(input: &str) -> Box<dyn AbstractPuzzle> {
        Box::new(Puzzle04 {
            input: input.to_string()
        })
    }
}

fn parse_passport(passport: &str) -> HashMap<String, String> {
    passport.split_whitespace()
        .filter_map(|entry| parse_passport_entry(entry))
        .collect::<HashMap<_, _>>()
}

fn parse_passport_entry(entry: &str) -> Option<(String, String)> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(byr|iyr|eyr|hgt|hcl|ecl|pid|cid):(\S+)$").unwrap();
    }
    RE.captures(entry).map(|cap| (cap[1].to_string(), cap[2].to_string()))
}

fn has_required_fields(passport: &HashMap<String, String>) -> bool {
    passport.len() == 8 || passport.len() == 7 && !passport.contains_key("cid")
}

fn all_values_valid(passport: &HashMap<String, String>) -> bool {
    passport.iter().all(|(key, value)| is_value_valid(key.as_str(), value.as_str()))
}

fn is_value_valid(key: &str, value: &str) -> bool {
    match key {
        "byr" => value.len() == 4 && in_range(value, 1920, 2002),
        "iyr" => value.len() == 4 && in_range(value, 2010, 2020),
        "eyr" => value.len() == 4 && in_range(value, 2020, 2030),
        "hgt" => {
            value.len() > 2 && match &value[(value.len() - 2)..] {
                "cm" => in_range(&value[0..(value.len() - 2)], 150, 193),
                "in" => in_range(&value[0..(value.len() - 2)], 59, 76),
                _ => false
            }
        },
        "hcl" => {
            lazy_static! {
                static ref RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
            }
            RE.is_match(value)
        },
        "ecl" => {
            lazy_static! {
                static ref RE: Regex = Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$").unwrap();
            }
            RE.is_match(value)
        },
        "pid" => {
            lazy_static! {
                static ref RE: Regex = Regex::new(r"^\d{9}$").unwrap();
            }
            RE.is_match(value)
        },
        "cid" => true,
        _ => false
    }
}

fn in_range(number: &str, start_inclusive: usize, end_inclusive: usize) -> bool {
    match number.parse::<usize>() {
        Ok(value) => value >= start_inclusive && value <= end_inclusive,
        Err(_) => false
    }
}

#[cfg(test)]
mod tests {
    use crate::puzzle04::{Puzzle04, is_value_valid};
    use std::path::PathBuf;
    use std::fs;

    #[test]
    fn test_part_1_example_1() {
        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\n\
                byr:1937 iyr:2017 cid:147 hgt:183cm\n\n\
                iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\n\
                hcl:#cfa07d byr:1929\n\n\
                hcl:#ae17e1 iyr:2013\n\
                eyr:2024\n\
                ecl:brn pid:760753108 byr:1931\n\
                hgt:179cm\n\n\
                hcl:#cfa07d eyr:2025 pid:166559648\n\
                iyr:2011 ecl:brn hgt:59in";
        let puzzle = Puzzle04::create(input);
        assert_eq!(puzzle.solve_part_1(), "2");
    }

    #[test]
    fn test_part_1_example_2() {
        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\n\
                byr:1937 iyr:2017 cid:147 hgt:183cm\n\n\
                iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\n\
                hcl:#cfa07d byr:1929\n\n\
                hcl:#ae17e1 iyr:2013\n\
                eyr:2024\n\
                ecl:brn pid:760753108 byr:1931\n\
                hgt:179cm\n\n\
                hcl:#cfa07d eyr:2025 pid:166559648\n\
                iyr:2011 ecl:brn hgt:59in";
        let puzzle = Puzzle04::create(input);
        assert_eq!(puzzle.solve_part_1(), "2");
    }

    #[test]
    fn test_part_1_example_3() {
        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\n\
                byr:1937 iyr:2017 cid:147 hgt:183cm\n\n\
                iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\n\
                hcl:#cfa07d byr:1929\n\n\
                hcl:#ae17e1 iyr:2013\n\
                eyr:2024\n\
                ecl:brn pid:760753108 byr:1931\n\
                hgt:179cm\n\n\
                hcl:#cfa07d eyr:2025 pid:166559648\n\
                iyr:2011 ecl:brn hgt:59in";
        let puzzle = Puzzle04::create(input);
        assert_eq!(puzzle.solve_part_1(), "2");
    }

    #[test]
    fn test_part_1_example_4() {
        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\n\
                byr:1937 iyr:2017 cid:147 hgt:183cm\n\n\
                iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\n\
                hcl:#cfa07d byr:1929\n\n\
                hcl:#ae17e1 iyr:2013\n\
                eyr:2024\n\
                ecl:brn pid:760753108 byr:1931\n\
                hgt:179cm\n\n\
                hcl:#cfa07d eyr:2025 pid:166559648\n\
                iyr:2011 ecl:brn hgt:59in";
        let puzzle = Puzzle04::create(input);
        assert_eq!(puzzle.solve_part_1(), "2");
    }

    #[test]
    fn test_part_1_example_5() {
        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\n\
                byr:1937 iyr:2017 cid:147 hgt:183cm\n\n\
                iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\n\
                hcl:#cfa07d byr:1929\n\n\
                hcl:#ae17e1 iyr:2013\n\
                eyr:2024\n\
                ecl:brn pid:760753108 byr:1931\n\
                hgt:179cm\n\n\
                hcl:#cfa07d eyr:2025 pid:166559648\n\
                iyr:2011 ecl:brn hgt:59in";
        let puzzle = Puzzle04::create(input);
        assert_eq!(puzzle.solve_part_1(), "2");
    }

    #[test]
    fn test_solve_part_1() {
        let input = fs::read_to_string(PathBuf::from("resources/tests/4")).unwrap();
        let puzzle = Puzzle04::create(input.as_str());
        assert_eq!(puzzle.solve_part_1(), "250");
    }

    #[test]
    fn test_part_2_example_1() {
        assert!(is_value_valid("byr", "2002"));
        assert!(!is_value_valid("bry", "2003"));
        assert!(is_value_valid("hgt", "60in"));
        assert!(is_value_valid("hgt", "190cm"));
        assert!(!is_value_valid("hgt", "190in"));
        assert!(!is_value_valid("hgt", "190"));
        assert!(is_value_valid("hcl", "#123abc"));
        assert!(!is_value_valid("hcl", "#123abz"));
        assert!(!is_value_valid("hcl", "123abc"));
        assert!(is_value_valid("ecl", "brn"));
        assert!(!is_value_valid("ecl", "wat"));
        assert!(is_value_valid("pid", "000000001"));
        assert!(!is_value_valid("pid", "0123456789"));
    }

    #[test]
    fn test_part_2_example_2() {
        let input = "eyr:1972 cid:100\n\
                hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926\n\n\
                iyr:2019\n\
                hcl:#602927 eyr:1967 hgt:170cm\n\
                ecl:grn pid:012533040 byr:1946\n\n\
                hcl:dab227 iyr:2012\n\
                ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277\n\n\
                hgt:59cm ecl:zzz\n\
                eyr:2038 hcl:74454a iyr:2023\n\
                pid:3556412378 byr:2007";
        let puzzle = Puzzle04::create(input);
        assert_eq!(puzzle.solve_part_2(), "0");
    }

    #[test]
    fn test_part_2_example_3() {
        let input = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980\n\
                hcl:#623a2f\n\n\
                eyr:2029 ecl:blu cid:129 byr:1989\n\
                iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm\n\n\
                hcl:#888785\n\
                hgt:164cm byr:2001 iyr:2015 cid:88\n\
                pid:545766238 ecl:hzl\n\
                eyr:2022\n\n\
                iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
        let puzzle = Puzzle04::create(input);
        assert_eq!(puzzle.solve_part_2(), "4");
    }

    #[test]
    fn test_solve_part_2() {
        let input = fs::read_to_string(PathBuf::from("resources/tests/4")).unwrap();
        let puzzle = Puzzle04::create(input.as_str());
        assert_eq!(puzzle.solve_part_2(), "158");
    }
}
