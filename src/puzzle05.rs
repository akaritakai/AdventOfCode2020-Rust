use crate::puzzle::AbstractPuzzle;

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
        let mut seat_ids: Vec<usize> = self.input.lines()
            .map(|line| seat_id(line))
            .collect();
        seat_ids.sort_unstable();
        for i in 0..seat_ids.len()-1 {
            let seat_id1 = seat_ids[i];
            let seat_id2 = seat_ids[i + 1];
            if seat_id1 + 1 != seat_id2 {
                return (seat_id1 + 1).to_string();
            }
        }
        unreachable!()
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
    let mut min_row = 0;
    let mut max_row = 127;
    let mut min_col = 0;
    let mut max_col = 7;
    boarding_pass.chars().for_each(|c| match c {
        'F' => max_row -= (max_row - min_row) / 2 + 1,
        'B' => min_row += (max_row - min_row) / 2 + 1,
        'L' => max_col -= (max_col - min_col) / 2 + 1,
        'R' => min_col += (max_col - min_col) / 2 + 1,
        _ => unreachable!()
    });
    min_row * 8 + min_col
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
