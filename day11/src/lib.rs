use std::collections::HashMap;

use nom::{
    character::complete::{self, space1},
    multi::separated_list1,
    IResult, Parser,
};

pub fn part1(input: &str) -> usize {
    solve_puzzle_cached(input, 25)
}

pub fn part2(input: &str) -> usize {
    solve_puzzle_cached(input, 75)
}

fn solve_puzzle_cached(data: &str, blinks: usize) -> usize {
    let stones = parse_data(data);
    let mut cache: HashMap<(u64, usize), u64> = HashMap::new();

    // Now, we will use caching
    let mut total: usize = 0;
    for num in stones.iter() {
        total += wink_stone(*num, blinks, &mut cache);
    }
    total
}

fn blink_stone(num: u64, blinks: usize) -> Vec<u64> {
    let mut digits: Vec<u64> = vec![num];
    for _ in 0..blinks {
        let mut current: Vec<u64> = Vec::new();
        for num in digits.iter() {
            if *num == 0 {
                current.push(1);
            } else {
                let digits = get_num_digits(*num);
                if digits.len() % 2 == 0 {
                    let (left, right) = split_digits(digits);
                    current.push(left);
                    current.push(right);
                } else {
                    current.push(*num * 2024);
                }
            }
        }
        digits = current;
    }
    digits
}

fn wink_stone(num: u64, blinks: usize, cache: &mut HashMap<(u64, usize), u64>) -> usize {
    if blinks == 0 {
        return 1;
    }

    if let Some(entry) = cache.get(&(num, blinks)) {
        return *entry as usize;
    }

    let mut total: usize = 0;
    let stones = blink_stone(num, 1);
    for v in stones.iter() {
        total += wink_stone(*v, blinks - 1, cache);
    }

    cache.insert((num, blinks), total as u64);
    total
}

fn parse_data(data: &str) -> Vec<u64> {
    if let Ok((_, items)) = parse_line(data) {
        return items;
    }
    vec![]
}

fn parse_line(data: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(space1, complete::u64).parse(data)
}

fn get_num_digits(num: u64) -> Vec<u64> {
    if num == 0 {
        return vec![0];
    }

    let mut digits: Vec<u64> = Vec::new();
    let mut n = num;
    while n > 0 {
        digits.push(n % 10);
        n /= 10;
    }
    digits.reverse();
    digits
}

fn merge_digits(digits: &[u64]) -> u64 {
    let result: u64 = digits
        .iter()
        .enumerate()
        .rev()
        .map(|(k, v)| {
            let d = 10_u64.pow((digits.len() - k - 1) as u32);
            d * v
        })
        .sum();
    result
}

fn split_digits(digits: Vec<u64>) -> (u64, u64) {
    assert!(digits.len() % 2 == 0, "Number of digits must be even");
    let (left, right) = digits.split_at(digits.len() / 2);
    (merge_digits(left), merge_digits(right))
}

#[cfg(test)]
mod tests {
    use input::get_puzzle_input;

    use super::*;

    #[test]
    fn test_is_event_digits() {
        assert_eq!(get_num_digits(253000), vec![2, 5, 3, 0, 0, 0]);
        assert_eq!(get_num_digits(10), vec![1, 0]);
        assert_eq!(get_num_digits(1), vec![1]);
        assert_eq!(get_num_digits(253), vec![2, 5, 3]);
    }

    #[test]
    fn test_merge_digits() {
        assert_eq!(merge_digits(&vec![2, 5, 3, 0, 0, 0]), 253000);
        assert_eq!(merge_digits(&vec![1, 0]), 10);
    }

    #[test]
    fn test_split_digits() {
        assert_eq!(split_digits(vec![2, 5, 3, 0, 0, 0]), (253, 0));
        assert_eq!(split_digits(vec![1, 0]), (1, 0));
    }

    #[test]
    fn test_part1() {
        let input = get_puzzle_input("11-sample");
        let result = solve_puzzle_cached(input.as_str(), 6);
        assert_eq!(result, 22);
    }

    #[test]
    fn test_part2() {
        let input = get_puzzle_input("11-sample");
        let result = solve_puzzle_cached(input.as_str(), 25);
        assert_eq!(result, 55312);
    }
}
