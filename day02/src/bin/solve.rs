use day02::{part1, part2};
use input::get_puzzle_input;

fn main() {
    run_part1();
    run_part2();
}

pub fn run_part1() {
    let input_string = get_puzzle_input("02");
    let value = part1(input_string.as_str());
    println!("Result: {}", value);
}

pub fn run_part2() {
    let input_string = get_puzzle_input("02");
    let value = part2(input_string.as_str());

    println!("Result: {}", value);
}
