use aoc2024::day02::{part1, part2};
use aoc2024::get_puzzle_input;

fn main() {
    divan::main();
}

#[divan::bench]
fn part1_bench() {
    let input_string = get_puzzle_input("02");
    part1(divan::black_box(input_string.as_str()));
}

#[divan::bench]
fn part2_bench() {
    let input_string = get_puzzle_input("02");
    part2(divan::black_box(input_string.as_str()));
}
