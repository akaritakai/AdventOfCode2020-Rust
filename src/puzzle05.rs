use crate::puzzle::AbstractPuzzle;
use std::collections::BTreeSet;

pub struct Puzzle05 {
    input: String,
}

impl AbstractPuzzle for Puzzle05 {
    fn get_day(&self) -> u8 {
        5
    }

    fn solve_part_1(&self) -> String {
        self.input.lines()
            .map(|line| seat_id(line))
            .max()
            .unwrap()
            .to_string()
    }

    fn solve_part_2(&self) -> String {
        let seat_ids = self.input.lines()
            .map(|line| seat_id(line))
            .collect::<BTreeSet<_>>();
        let min = *seat_ids.iter().next().unwrap();
        let max = *seat_ids.iter().last().unwrap();
        (min..max).find(|seat_id| !seat_ids.contains(seat_id)).unwrap().to_string()
    }
}

impl Puzzle05 {
    pub fn create(input: &str) -> Box<dyn AbstractPuzzle> {
        Box::new(Puzzle05 {
            input: input.to_string()
        })
    }
}

fn seat_id(boarding_pass: &str) -> usize {
    boarding_pass.chars().fold(0, |n, c| {
        (n << 1) | match c {
            'B' | 'R' => 1,
            _ => 0
        }
    })
}

#[cfg(test)]
mod tests {
    use crate::puzzle05::Puzzle05;
    use std::path::PathBuf;
    use std::fs;

    #[test]
    fn test_part_1_example_1() {
        let puzzle = Puzzle05::create("FBFBBFFRLR");
        assert_eq!(puzzle.solve_part_1(), "357");
    }

    #[test]
    fn test_part_1_example_2() {
        let puzzle = Puzzle05::create("BFFFBBFRRR");
        assert_eq!(puzzle.solve_part_1(), "567");
    }

    #[test]
    fn test_part_1_example_3() {
        let puzzle = Puzzle05::create("FFFBBBFRRR");
        assert_eq!(puzzle.solve_part_1(), "119");
    }

    #[test]
    fn test_part_1_example_4() {
        let puzzle = Puzzle05::create("BBFFBBFRLL");
        assert_eq!(puzzle.solve_part_1(), "820");
    }

    #[test]
    fn test_part_1_example_5() {
        let puzzle = Puzzle05::create("\
            FBFBBFFRLR\n\
            BFFFBBFRRR\n\
            BBFFBBFRLL\n\
            FFFBBBFRRR");
        assert_eq!(puzzle.solve_part_1(), "820");
    }

    #[test]
    fn test_solve_part_1() {
        let input = fs::read_to_string(PathBuf::from("resources/tests/5")).unwrap();
        let puzzle = Puzzle05::create(input.as_str());
        assert_eq!(puzzle.solve_part_1(), "906");
    }

    #[test]
    fn test_solve_part_2() {
        let input = fs::read_to_string(PathBuf::from("resources/tests/5")).unwrap();
        let puzzle = Puzzle05::create(input.as_str());
        assert_eq!(puzzle.solve_part_2(), "519");
    }
}
