use crate::puzzle::AbstractPuzzle;
use std::collections::HashSet;

pub struct Puzzle01 {
    input: String,
}

impl AbstractPuzzle for Puzzle01 {
    fn get_day(&self) -> u8 {
        return 1;
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
        numbers.sort();
        for i in 0..numbers.len() - 2 {
            let n1 = numbers[i];
            let mut j = i + 1;
            let mut k = numbers.len() - 1;
            while j < k {
                let n2 = numbers[j];
                let n3 = numbers[k];
                let sum = n1 + n2 + n3;
                if sum < 2020 {
                    j += 1;
                } else if sum > 2020 {
                    k -= 1;
                } else {
                    return (n1 * n2 * n3).to_string();
                }
            }
        }
        panic!("Unable to find the solution");
    }
}

impl Puzzle01 {
    pub fn create(input: &str) -> Box<dyn AbstractPuzzle> {
        return Box::new(Puzzle01 {
            input: input.to_string()
        });
    }
}
