use std::time::Instant;

use day08::{part1, part2};
use input::{format_duration, get_puzzle_input};

fn main() {
    run_part1();
    run_part2();
}

pub fn run_part1() {
    let ts = Instant::now();
    let input_string = get_puzzle_input("08");
    let result = part1(input_string.as_str());
    let duration = ts.elapsed();
    println!(
        "Result: {}, duration: {}",
        result,
        format_duration(duration)
    );
}

pub fn run_part2() {
    let ts = Instant::now();
    let input_string = get_puzzle_input("08");
    let result = part2(input_string.as_str());
    let duration = ts.elapsed();
    println!(
        "Result: {}, duration: {}",
        result,
        format_duration(duration)
    );
}
