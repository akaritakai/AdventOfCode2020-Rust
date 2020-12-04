use crate::puzzle::AbstractPuzzle;
use std::collections::HashSet;
use std::cmp::Ordering;

pub struct Puzzle01 {
    input: String,
}

impl AbstractPuzzle for Puzzle01 {
    fn get_day(&self) -> u8 {
        1
    }

    fn solve_part_1(&self) -> String {
        let mut numbers : HashSet<u32> = HashSet::new();
        for line in self.input.lines() {
            let n1 = line.parse::<u32>().unwrap();
            let n2 = 2020 - n1;
            if numbers.contains(&n2) {
                return (n1 * n2).to_string();
            }
            numbers.insert(n1);
        }
        panic!("Unable to find the solution");
    }

    fn solve_part_2(&self) -> String {
        let mut numbers: Vec<u32> = self.input.lines()
            .map(|line| line.parse::<u32>().unwrap())
            .collect();
        numbers.sort_unstable();
        for i in 0..numbers.len() - 2 {
            let n1 = numbers[i];
            let mut j = i + 1;
            let mut k = numbers.len() - 1;
            while j < k {
                let n2 = numbers[j];
                let n3 = numbers[k];
                let sum = n1 + n2 + n3;
                match sum.cmp(&2020) {
                    Ordering::Less => j += 1,
                    Ordering::Greater => k -= 1,
                    Ordering::Equal => return (n1 * n2 * n3).to_string()
                }
            }
        }
        panic!("Unable to find the solution");
    }
}

impl Puzzle01 {
    pub fn create(input: &str) -> Box<dyn AbstractPuzzle> {
        Box::new(Puzzle01 {
            input: input.to_string()
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::puzzle01::Puzzle01;
    use std::path::PathBuf;
    use std::fs;

    #[test]
    fn test_part_1_example_1() {
        let input = vec!["1721", "979", "366", "299", "675", "1456"];
        let puzzle = Puzzle01::create(&input.join("\n"));
        assert_eq!(puzzle.solve_part_1(), "514579");
    }

    #[test]
    fn test_part_1_example_2() {
        // Ensure duplicate entries are handled
        let input = vec!["1000", "1010", "1010"];
        let puzzle = Puzzle01::create(&input.join("\n"));
        assert_eq!(puzzle.solve_part_1(), "1020100");
    }

    #[test]
    fn test_part_1_example_3() {
        // Ensure duplicate entries are handled (alternate order)
        let input = vec!["1010", "1000", "1010"];
        let puzzle = Puzzle01::create(&input.join("\n"));
        assert_eq!(puzzle.solve_part_1(), "1020100");
    }

    #[test]
    fn test_part_1_example_4() {
        // Ensure a size of 2 is handled
        let input = vec!["1009", "1011"];
        let puzzle = Puzzle01::create(&input.join("\n"));
        assert_eq!(puzzle.solve_part_1(), "1020099");
    }

    #[test]
    #[should_panic]
    fn test_part_1_example_5() {
        // Ensure a size of 1 fails
        let input = vec!["2020"];
        let puzzle = Puzzle01::create(&input.join("\n"));
        puzzle.solve_part_1();
    }

    #[test]
    #[should_panic]
    fn test_part_1_example_6() {
        // Ensure an input that does not satisfy part 1 fails
        let input = vec!["1010", "1011"];
        let puzzle = Puzzle01::create(&input.join("\n"));
        puzzle.solve_part_1();
    }

    #[test]
    #[should_panic]
    fn test_part_1_example_7() {
        // Ensure an input that does not satisfy part 1 fails (alternate)
        let input = vec!["1721", "979", "366", "675", "1456"];
        let puzzle = Puzzle01::create(&input.join("\n"));
        puzzle.solve_part_1();
    }

    #[test]
    fn test_part_1_example_8() {
        // Ensure a size of 2 with duplicates are handled
        let input = vec!["1010", "1010"];
        let puzzle = Puzzle01::create(&input.join("\n"));
        assert_eq!(puzzle.solve_part_1(), "1020100");
    }

    #[test]
    fn test_part_1_example_9() {
        // Ensure a pass if the subset is on the extremes of the array
        let input = vec!["1721", "979", "366", "675", "1456", "299"];
        let puzzle = Puzzle01::create(&input.join("\n"));
        assert_eq!(puzzle.solve_part_1(), "514579");
    }

    #[test]
    #[should_panic]
    fn test_part_1_example_10() {
        // Ensure an empty input fails
        let puzzle = Puzzle01::create("");
        puzzle.solve_part_1();
    }

    #[test]
    fn test_solve_part_1() {
        let input = fs::read_to_string(PathBuf::from("resources/tests/1")).unwrap();
        let puzzle = Puzzle01::create(input.as_str());
        assert_eq!(puzzle.solve_part_1(), "440979");
    }

    #[test]
    fn test_part_2_example_1() {
        let input = vec!["1721", "979", "366", "299", "675", "1456"];
        let puzzle = Puzzle01::create(&input.join("\n"));
        assert_eq!(puzzle.solve_part_2(), "241861950");
    }

    #[test]
    fn test_part_2_example_2() {
        // Ensure duplicate entries are handled
        let input = vec!["672", "672", "676"];
        let puzzle = Puzzle01::create(&input.join("\n"));
        assert_eq!(puzzle.solve_part_2(), "305270784");
    }

    #[test]
    fn test_part_2_example_3() {
        // Ensure duplicate entries are handled (alternate order)
        let input = vec!["672", "676", "672"];
        let puzzle = Puzzle01::create(&input.join("\n"));
        assert_eq!(puzzle.solve_part_2(), "305270784");
    }

    #[test]
    #[should_panic]
    fn test_part_2_example_4() {
        // Ensure a size of 2 fails
        let input = vec!["1009", "1011"];
        let puzzle = Puzzle01::create(&input.join("\n"));
        puzzle.solve_part_2();
    }

    #[test]
    #[should_panic]
    fn test_part_2_example_5() {
        // Ensure a size of 1 fails
        let input = vec!["2020"];
        let puzzle = Puzzle01::create(&input.join("\n"));
        puzzle.solve_part_2();
    }

    #[test]
    #[should_panic]
    fn test_part_2_example_6() {
        // Ensure an input that does not satisfy part 2 fails
        let input = vec!["500", "501", "1020"];
        let puzzle = Puzzle01::create(&input.join("\n"));
        puzzle.solve_part_2();
    }

    #[test]
    #[should_panic]
    fn test_part_2_example_7() {
        // Ensure an input that does not satisfy part 2 fails (alternate)
        let input = vec!["1721", "979", "366", "299", "1456"];
        let puzzle = Puzzle01::create(&input.join("\n"));
        puzzle.solve_part_2();
    }

    #[test]
    fn test_part_2_example_8() {
        // Ensure a size of 4 with duplicates are handled
        let input = vec!["672", "500", "672", "676"];
        let puzzle = Puzzle01::create(&input.join("\n"));
        assert_eq!(puzzle.solve_part_2(), "305270784");
    }

    #[test]
    fn test_part_2_example_9() {
        // Ensure a pass if the subset is on the extremes of the array
        let input = vec!["979", "1721", "1456", "366", "299", "675"];
        let puzzle = Puzzle01::create(&input.join("\n"));
        assert_eq!(puzzle.solve_part_2(), "241861950");
    }

    #[test]
    #[should_panic]
    fn test_part_2_example_10() {
        // Ensure an empty input fails
        let puzzle = Puzzle01::create("");
        puzzle.solve_part_2();
    }

    #[test]
    fn test_solve_part_2() {
        let input = fs::read_to_string(PathBuf::from("resources/tests/1")).unwrap();
        let puzzle = Puzzle01::create(input.as_str());
        assert_eq!(puzzle.solve_part_2(), "82498112");
    }
}
