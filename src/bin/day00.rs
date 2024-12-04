use aoc2024::day00::{part1, part2};
use aoc2024::get_puzzle_input;

fn main() {
    run_part1();
    run_part2();
}

pub fn run_part1() {
    let input_string = get_puzzle_input("00");
    let result = part1(input_string.as_str());
    println!("Result: {}", result);
}

pub fn run_part2() {
    let input_string = get_puzzle_input("00");
    let result = part2(input_string.as_str());
    println!("Result: {}", result);
}
