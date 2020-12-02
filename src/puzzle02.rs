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

#[cfg(test)]
mod tests {
    use crate::puzzle02::Puzzle02;
    use std::path::PathBuf;
    use std::fs;

    #[test]
    fn test_part_1_example_1() {
        let input = vec![
            "1-3 a: abcde",
            "1-3 b: cdefg",
            "2-9 c: ccccccccc"
        ];
        let puzzle = Puzzle02::create(&input.join("\n"));
        assert_eq!(puzzle.solve_part_1(), "2");
    }

    #[test]
    fn test_part_1_example_2() {
        let input = vec![
            "1-3 a: abcde"
        ];
        let puzzle = Puzzle02::create(&input.join("\n"));
        assert_eq!(puzzle.solve_part_1(), "1");
    }

    #[test]
    fn test_part_1_example_3() {
        let input = vec![
            "1-3 b: cdefg"
        ];
        let puzzle = Puzzle02::create(&input.join("\n"));
        assert_eq!(puzzle.solve_part_1(), "0");
    }

    #[test]
    fn test_part_1_example_4() {
        let input = vec![
            "2-9 c: ccccccccc"
        ];
        let puzzle = Puzzle02::create(&input.join("\n"));
        assert_eq!(puzzle.solve_part_1(), "1");
    }

    #[test]
    fn test_solve_part_1() {
        let input = fs::read_to_string(PathBuf::from("resources/tests/2")).unwrap();
        let puzzle = Puzzle02::create(input.as_str());
        assert_eq!(puzzle.solve_part_1(), "434");
    }

    #[test]
    fn test_part_2_example_1() {
        let input = vec![
            "1-3 a: abcde",
            "1-3 b: cdefg",
            "2-9 c: ccccccccc"
        ];
        let puzzle = Puzzle02::create(&input.join("\n"));
        assert_eq!(puzzle.solve_part_2(), "1");
    }

    #[test]
    fn test_part_2_example_2() {
        let input = vec![
            "1-3 a: abcde"
        ];
        let puzzle = Puzzle02::create(&input.join("\n"));
        assert_eq!(puzzle.solve_part_2(), "1");
    }

    #[test]
    fn test_part_2_example_3() {
        let input = vec![
            "1-3 b: cdefg"
        ];
        let puzzle = Puzzle02::create(&input.join("\n"));
        assert_eq!(puzzle.solve_part_2(), "0");
    }

    #[test]
    fn test_part_2_example_4() {
        let input = vec![
            "2-9 c: ccccccccc"
        ];
        let puzzle = Puzzle02::create(&input.join("\n"));
        assert_eq!(puzzle.solve_part_2(), "0");
    }

    #[test]
    fn test_solve_part_2() {
        let input = fs::read_to_string(PathBuf::from("resources/tests/2")).unwrap();
        let puzzle = Puzzle02::create(input.as_str());
        assert_eq!(puzzle.solve_part_2(), "509");
    }
}
