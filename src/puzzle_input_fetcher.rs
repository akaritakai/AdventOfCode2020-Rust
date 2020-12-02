use std::{fmt, fs};
use std::ops::Not;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

use reqwest::StatusCode;

pub struct PuzzleInputFetcher {
    is_input_set: Vec<Arc<RwLock<bool>>>,
    inputs: Vec<String>,
    is_session_token_set: Arc<RwLock<bool>>,
    session_token: String,
}

impl PuzzleInputFetcher {
    pub fn create() -> PuzzleInputFetcher {
        let mut is_input_set = Vec::with_capacity(26);
        (0..26).for_each(|_| is_input_set.push(Arc::new(RwLock::new(false))));
        return PuzzleInputFetcher {
            is_input_set,
            inputs: vec![String::new(); 26],
            is_session_token_set: Arc::new(RwLock::new(false)),
            session_token: String::new(),
        };
    }

    pub fn get_puzzle_input(&mut self, day: u8) -> Result<&str> {
        if self.is_input_set[day as usize].read().unwrap().not() {
            let mut is_input_set = self.is_input_set[day as usize].write().unwrap();
            if is_input_set.not() {
                let local_input = self.fetch_local_puzzle_input(day);
                return if local_input.is_ok() {
                    self.inputs[day as usize].push_str(local_input.unwrap().as_str());
                    *is_input_set = true;
                    Ok(self.inputs[day as usize].as_str())
                } else {
                    if self.is_session_token_set.read().unwrap().not() {
                        let mut is_session_token_set = self.is_session_token_set.write().unwrap();
                        if is_session_token_set.not() {
                            let session_token = fs::read_to_string(PathBuf::from("cookie.txt"))
                                .map_err(|e| Error::new(format!(
                                    "Failed to fetch session token: {}", e)))?;
                            self.session_token.push_str(session_token.as_str());
                            *is_session_token_set = true;
                        }
                    }
                    let session_token = self.session_token.as_str();
                    let remote_input = self.fetch_remote_puzzle_input(day, session_token)?;
                    self.store_puzzle_input_locally(day, remote_input.as_str());
                    self.inputs[day as usize].push_str(remote_input.as_str());
                    *is_input_set = true;
                    Ok(self.inputs[day as usize].as_str())
                };
            }
        }
        return Ok(self.inputs[day as usize].as_str());
    }

    fn fetch_local_puzzle_input(&self, day: u8) -> Result<String> {
        let path = PathBuf::from("puzzle").join(day.to_string());
        return fs::read_to_string(path).map_err(|e| Error::new(format!(
                "Failed to fetch local puzzle for day {}: {}", day, e)));
    }

    fn store_puzzle_input_locally(&self, day: u8, input: &str) {
        let path = PathBuf::from("puzzle");
        let _ = fs::create_dir_all(path.as_path());
        let path = path.join(day.to_string());
        let _ = fs::write(path, input);
    }

    fn fetch_remote_puzzle_input(&self, day: u8, session_token: &str) -> Result<String> {
        let client = reqwest::blocking::Client::new();
        let path = format!("https://adventofcode.com/2020/day/{}/input", day);
        let response = client.get(&path)
            .header("Cookie", format!("session={}", session_token))
            .send()
            .map_err(|e| Error::new(format!(
                "Failed to fetch remote puzzle input for day {}: {}", day, e)))?;
        return if response.status() != StatusCode::OK {
            Err(Error::new(format!(
                "Failed to fetch remote puzzle input for day {}: Got status code = {}",
                day, response.status())))
        } else {
            response.text().map_err(|e| Error::new(format!(
                "Failed to fetch remote puzzle input for day {}: Failed to read body as text: {}",
                day, e)))
        };
    }
}

type Result<T> = std::result::Result<T, PuzzleInputFetcherError>;
type Error = PuzzleInputFetcherError;

#[derive(Debug, Clone)]
pub struct PuzzleInputFetcherError {
    description: String
}

impl PuzzleInputFetcherError {
    fn new(description: String) -> PuzzleInputFetcherError {
        return PuzzleInputFetcherError { description };
    }
}

impl fmt::Display for PuzzleInputFetcherError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}", self.description);
    }
}
