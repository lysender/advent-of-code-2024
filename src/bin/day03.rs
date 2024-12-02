use aoc2024::day03::{part1, part2};
use aoc2024::get_puzzle_input;

fn main() {
    run_part1();
    run_part2();
}

pub fn run_part1() {
    let input_string = get_puzzle_input("03");
    let value = part1(input_string.as_str());
    println!("Total safe reports: v1: {}", value);
}

pub fn run_part2() {
    let input_string = get_puzzle_input("03");
    let value = part2(input_string.as_str());

    println!("Total safe reports dampened: v2: {}", value);
}
