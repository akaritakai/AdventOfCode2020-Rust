use crate::puzzle::AbstractPuzzle;
use regex::Regex;

pub struct Puzzle02 {
    input: String,
}

impl AbstractPuzzle for Puzzle02 {
    fn get_day(&self) -> u8 {
        return 2;
    }

    fn solve_part_1(&self) -> String {
        return self.input.lines().into_iter()
            .map(|line| parse(line))
            .filter(|(lower, upper, letter, password)| {
                let count = password.matches(*letter).count();
                return count >= *lower && count <= *upper;
            })
            .count()
            .to_string();
    }

    fn solve_part_2(&self) -> String {
        return self.input.lines().into_iter()
            .map(|line| parse(line))
            .filter(|(lower, upper, letter, password)|
                  (lower - 1 < password.len() && password.as_bytes()[lower - 1] as char == *letter)
                ^ (upper - 1 < password.len() && password.as_bytes()[upper - 1] as char == *letter))
            .count()
            .to_string();
    }
}

impl Puzzle02 {
    pub fn create(input: &str) -> Box<dyn AbstractPuzzle> {
        return Box::new(Puzzle02 {
            input: input.to_string()
        });
    }
}

fn parse(line: &str) -> (usize, usize, char, String) {
    lazy_static! {
            static ref RE: Regex = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();
    }
    let cap = RE.captures(line).unwrap();
    return (cap[1].parse::<usize>().unwrap(),
            cap[2].parse::<usize>().unwrap(),
            cap[3].parse::<char>().unwrap(),
            cap[4].to_string());
}
