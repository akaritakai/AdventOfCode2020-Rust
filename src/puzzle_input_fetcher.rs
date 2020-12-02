use std::fs;
use std::path::PathBuf;
use std::sync::{RwLock, Arc};
use std::collections::HashMap;
use std::error::Error;

pub struct PuzzleInputFetcher {
    puzzle_path: PathBuf,
    session_token_path: PathBuf,
    session_token: Arc<RwLock<Option<String>>>,
    cache: Arc<RwLock<HashMap<u8, String>>>
}

impl PuzzleInputFetcher {
    pub fn create() -> PuzzleInputFetcher {
        return PuzzleInputFetcher {
            puzzle_path: PathBuf::from("puzzle"),
            session_token_path: PathBuf::from("cookie.txt"),
            session_token: Arc::new(RwLock::new(None)),
            cache: Arc::new(RwLock::new(HashMap::new()))
        }
    }

    pub fn get_puzzle_input(&self, day: u8) -> Result<String, Box<dyn Error>> {
        let cache = self.cache.read().unwrap();
        if !cache.contains_key(&day) {
            drop(cache);
            let local_input = self.fetch_local_puzzle_input(day);
            if local_input.is_ok() {
                let mut cache = self.cache.write().unwrap();
                if !cache.contains_key(&day) {
                    cache.insert(day, local_input.as_ref().unwrap().clone());
                }
                return Ok(cache.get(&day).unwrap().to_owned());
            }
            let remote_input = self.fetch_remote_puzzle_input(day)?;
            let mut cache = self.cache.write().unwrap();
            if !cache.contains_key(&day) {
                cache.insert(day, remote_input.clone());
                self.store_puzzle_input_locally(day, remote_input.clone().as_str());
            }
            return Ok(cache.get(&day).unwrap().to_owned());
        }
        return Ok(cache.get(&day).unwrap().to_owned());
    }

    fn fetch_local_puzzle_input(&self, day: u8) -> Result<String, Box<dyn Error>> {
        let path = self.puzzle_path.join(day.to_string());
        return fs::read_to_string(&path).map_err(|e| e.into());
    }

    fn store_puzzle_input_locally(&self, day: u8, input: &str) {
        let _ = fs::create_dir_all(self.puzzle_path.as_path());
        let path = self.puzzle_path.as_path().join(day.to_string());
        let _ = fs::write(path, input);
    }

    fn fetch_remote_puzzle_input(&self, day: u8) -> Result<String, Box<dyn Error>> {
        let client = reqwest::blocking::Client::new();
        let path = format!("https://adventofcode.com/2020/day/{}/input", day);
        let token = self.get_session_token()?;
        return client.get(&path)
            .header("Cookie", format!("session={}", token))
            .send()?
            .text()
            .map_err(|e| e.into());
    }

    fn get_session_token(&self) -> Result<String, Box<dyn Error>> {
        let token = self.session_token.read().unwrap();
        if token.is_none() {
            drop(token);
            let mut token = self.session_token.write().unwrap();
            let path = self.session_token_path.to_owned();
            let value = fs::read_to_string(path)?;
            *token = Some(value);
            return Ok(token.as_ref().unwrap().clone());
        }
        return Ok(token.as_ref().unwrap().clone());
    }
}
