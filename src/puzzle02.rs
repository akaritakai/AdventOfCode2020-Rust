use crate::puzzle::AbstractPuzzle;
use regex::{Regex};

pub struct Puzzle02 {
    input: String,
}

impl AbstractPuzzle for Puzzle02 {
    fn get_day(&self) -> u8 {
        return 2;
    }

    fn solve_part_1(&self) -> String {
        let mut count = 0;
        for line in self.input.lines() {
            let data = self.parse(line);
            let mut n = 0;
            for c in data.password.chars() {
                if c == data.letter {
                    n = n + 1;
                }
            }
            if n >= data.lower && n <= data.upper {
                count = count + 1;
            }
        }
        return count.to_string();
    }

    fn solve_part_2(&self) -> String {
        let mut count = 0;
        for line in self.input.lines() {
            let data = self.parse(line);
            let lower =  data.lower - 1 < data.password.len()
                && data.password.chars().nth(data.lower - 1).unwrap() == data.letter;
            let upper =  data.upper - 1 < data.password.len()
                && data.password.chars().nth(data.upper - 1).unwrap() == data.letter;
            if lower ^ upper {
                count = count + 1;
            }
        }
        return count.to_string();
    }
}

struct PasswordAndPolicy {
    lower: usize,
    upper: usize,
    letter: char,
    password: String
}

impl Puzzle02 {
    pub fn create(input: &str) -> Box<dyn AbstractPuzzle> {
        return Box::new(Puzzle02 {
            input: input.to_string()
        });
    }

    fn parse(&self, line: &str) -> PasswordAndPolicy {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();
        }
        let cap = RE.captures(line).unwrap();
        return PasswordAndPolicy {
            lower: cap[1].parse::<usize>().unwrap(),
            upper: cap[2].parse::<usize>().unwrap(),
            letter: cap[3].parse::<char>().unwrap(),
            password: cap[4].to_string()
        }
    }
}
