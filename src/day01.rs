use itertools::Itertools;
use nom::{
    character::complete::{self, line_ending, space0},
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};

pub fn part1(input: &str) -> i32 {
    find_total_distances(input)
}

pub fn part2(input: &str) -> i32 {
    find_similarity_score(input)
}

fn find_total_distances(input: &str) -> i32 {
    if let Ok((_, items)) = parse_table(input) {
        let length = items.len();
        let mut left: Vec<u32> = Vec::with_capacity(length);
        let mut right: Vec<u32> = Vec::with_capacity(length);

        for (l, r) in items.into_iter() {
            left.push(l);
            right.push(r);
        }

        left.sort();
        right.sort();

        let total: i32 = std::iter::zip(left, right)
            .map(|(l, r)| (l as i32 - r as i32).abs())
            .sum();

        return total;
    }
    0
}

fn find_similarity_score(input: &str) -> i32 {
    if let Ok((_, items)) = parse_table(input) {
        let length = items.len();
        let mut left: Vec<u32> = Vec::with_capacity(length);
        let mut right: Vec<u32> = Vec::with_capacity(length);

        for (l, r) in items.into_iter() {
            left.push(l);
            right.push(r);
        }

        let counts = right.iter().counts();

        let total: i32 = left
            .iter()
            .map(|x| {
                if let Some(count) = counts.get(x) {
                    return *count as i32 * *x as i32;
                }
                0
            })
            .sum();

        return total;
    }

    0
}

fn parse_table(lines: &str) -> IResult<&str, Vec<(u32, u32)>> {
    separated_list1(line_ending, pair_parser).parse(lines)
}

fn pair_parser(line: &str) -> IResult<&str, (u32, u32)> {
    separated_pair(complete::u32, space0, complete::u32).parse(line)
}

#[cfg(test)]
mod tests {
    use crate::get_puzzle_input;

    use super::*;

    #[test]
    fn test_part1() {
        let input = get_puzzle_input("01-sample");
        let total = find_total_distances(input.as_str());
        assert_eq!(total, 11);
    }

    #[test]
    fn test_part2() {
        let input = get_puzzle_input("01-sample");
        let total = find_similarity_score(input.as_str());
        assert_eq!(total, 31);
    }
}
