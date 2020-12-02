use crate::puzzle_input_fetcher::PuzzleInputFetcher;
use crate::puzzle01::Puzzle01;
use crate::puzzle::AbstractPuzzle;

mod puzzle_input_fetcher;
mod puzzle;
mod puzzle01;

fn main() {
    let fetcher = PuzzleInputFetcher::create();
    let puzzles : Vec<Box<dyn AbstractPuzzle>> = vec![
        Puzzle01::create(fetcher.get_puzzle_input(1).unwrap().as_str())
    ];
    for puzzle in puzzles.iter() {
        let day = format!("{:02}", puzzle.get_day());
        println!("Day {} Part 1: {}", day, puzzle.solve_part_1());
        println!("Day {} Part 2: {}", day, puzzle.solve_part_2());
    }
}
