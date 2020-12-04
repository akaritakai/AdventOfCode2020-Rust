use crate::puzzle::AbstractPuzzle;

pub struct Puzzle03 {
    input: String,
}

impl AbstractPuzzle for Puzzle03 {
    fn get_day(&self) -> u8 {
        3
    }

    fn solve_part_1(&self) -> String {
        self.trees_on_slope(3, 1).to_string()
    }

    fn solve_part_2(&self) -> String {
        let mut count = 1;
        count *= self.trees_on_slope(1, 1);
        count *= self.trees_on_slope(3, 1);
        count *= self.trees_on_slope(5, 1);
        count *= self.trees_on_slope(7, 1);
        count *= self.trees_on_slope(1, 2);
        count.to_string()
    }
}

impl Puzzle03 {
    pub fn create(input: &str) -> Box<dyn AbstractPuzzle> {
        Box::new(Puzzle03 {
            input: input.to_string()
        })
    }

    fn trees_on_slope(&self, x: usize, y: usize) -> usize {
        self.input.lines()
            .step_by(y)
            .enumerate()
            .filter(|(i, line)| line.chars().cycle().nth(i * x) == Some('#'))
            .count()
    }
}

#[cfg(test)]
mod tests {
    use crate::puzzle03::Puzzle03;
    use std::path::PathBuf;
    use std::fs;

    #[test]
    fn test_part_1_example_1() {
        let input = "..##.......\n\
                           #...#...#..\n\
                           .#....#..#.\n\
                           ..#.#...#.#\n\
                           .#...##..#.\n\
                           ..#.##.....\n\
                           .#.#.#....#\n\
                           .#........#\n\
                           #.##...#...\n\
                           #...##....#\n\
                           .#..#...#.#\n";
        let puzzle = Puzzle03::create(input);
        assert_eq!(puzzle.solve_part_1(), "7");
    }

    #[test]
    fn test_solve_part_1() {
        let input = fs::read_to_string(PathBuf::from("resources/tests/3")).unwrap();
        let puzzle = Puzzle03::create(input.as_str());
        assert_eq!(puzzle.solve_part_1(), "284");
    }

    #[test]
    fn test_part_2_example_1() {
        let input = "..##.......\n\
                           #...#...#..\n\
                           .#....#..#.\n\
                           ..#.#...#.#\n\
                           .#...##..#.\n\
                           ..#.##.....\n\
                           .#.#.#....#\n\
                           .#........#\n\
                           #.##...#...\n\
                           #...##....#\n\
                           .#..#...#.#\n";
        let puzzle = Puzzle03::create(input);
        assert_eq!(puzzle.solve_part_2(), "336");
    }

    #[test]
    fn test_solve_part_2() {
        let input = fs::read_to_string(PathBuf::from("resources/tests/3")).unwrap();
        let puzzle = Puzzle03::create(input.as_str());
        assert_eq!(puzzle.solve_part_2(), "3510149120");
    }
}
