use day09::{part1, part2};
use input::get_puzzle_input;

fn main() {
    divan::main();
}

#[divan::bench]
fn part1_bench() {
    let input_string = get_puzzle_input("09");
    part1(divan::black_box(input_string.as_str()));
}

#[divan::bench]
fn part2_bench() {
    let input_string = get_puzzle_input("09");
    part2(divan::black_box(input_string.as_str()));
}
