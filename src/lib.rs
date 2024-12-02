use std::path::Path;
use std::{fs, path::PathBuf};

pub mod day01;
pub mod day02;
pub mod day03;

pub fn get_puzzle_input(name: &str) -> String {
    let file = format!("day{}.txt", name);
    let filename: PathBuf = Path::new("data").join(file);
    let input_string = fs::read_to_string(filename).unwrap();
    input_string
}
