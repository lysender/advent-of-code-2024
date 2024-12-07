use day07::{part1, part2};
use input::get_puzzle_input;

fn main() {
    divan::main();
}

#[divan::bench]
fn part1_bench() {
    let input_string = get_puzzle_input("07");
    part1(divan::black_box(input_string.as_str()));
}

#[divan::bench]
fn part2_bench() {
    let input_string = get_puzzle_input("07");
    part2(divan::black_box(input_string.as_str()));
}
