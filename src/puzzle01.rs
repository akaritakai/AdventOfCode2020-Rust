use crate::puzzle::AbstractPuzzle;
use std::collections::HashMap;

pub struct Puzzle01 {
    input: String,
}

impl AbstractPuzzle for Puzzle01 {
    fn get_day(&self) -> u8 {
        return 1;
    }

    fn solve_part_1(&self) -> String {
        let mut numbers : HashMap<u32, usize> = HashMap::new();
        for n in self.numbers().iter() {
            match numbers.get_mut(n) {
                Some(count) => *count += 1,
                None => {
                    numbers.insert(*n, 1);
                    ()
                }
            }
        }
        for n in numbers.iter() {
            let n1 = *(n.0);
            let n2 = 2020 - n1;
            let count = numbers.get(&n2);
            if count.is_some() && (n1 != n2 || *count.unwrap() >= 2) {
                return format!("{}", n1 * n2);
            }
        }
        panic!("Unable to find the solution");
    }

    fn solve_part_2(&self) -> String {
        let mut numbers = self.numbers();
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
                    return format!("{}", n1 * n2 * n3);
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

    fn numbers(&self) -> Vec<u32> {
        let mut numbers: Vec<u32> = vec![];
        let lines = self.input.lines();
        for line in lines {
            let number = line.parse::<u32>().unwrap();
            numbers.push(number);
        }
        return numbers;
    }
}
