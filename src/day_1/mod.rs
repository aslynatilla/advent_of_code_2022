use std::fs::read_to_string;
use std::vec;

pub fn solution(path_to_file: &str) -> (u32, u32) {
    match read_to_string(path_to_file) {
        Ok(calories_input) => {
            let calories_list: Vec<Option<u32>> = calories_input
                .lines()
                .map(str::parse::<u32>)
                .map(Result::ok)
                .collect();

            let mut calories_sums = vec![0u32];
            for c in calories_list {
                match c {
                    Some(value) => {
                        if let Some(last_value) = calories_sums.last_mut() {
                            *last_value += value
                        }
                    }
                    None => calories_sums.push(0u32),
                }
            }

            calories_sums.sort_unstable();
            (
                *calories_sums.last().expect("Inappropriate input file"),
                calories_sums.iter().rev().take(3).sum::<u32>(),
            )
        }
        Err(_) => panic!("Could not open the file."),
    }
}
