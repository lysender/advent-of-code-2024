use std::path::Path;
use std::{fs, path::PathBuf};

pub fn get_puzzle_input(name: &str) -> String {
    let file = format!("day{}.txt", name);
    let filename: PathBuf = Path::new("..").join("data").join(file);
    fs::read_to_string(filename).unwrap()
}
