use rayon::prelude::*;
use std::collections::HashMap;

use nom::{
    character::complete::{self, space1},
    multi::separated_list1,
    IResult, Parser,
};

const BATCH: usize = 10;

pub fn part1(input: &str) -> i32 {
    solve_puzzle_cached(input, 25)
}

pub fn part2(input: &str) -> i32 {
    solve_puzzle_cached(input, 75)
}

fn solve_puzzle_cached(data: &str, blinks: usize) -> i32 {
    let orig_stones = parse_data(data);
    // Cache of stones blinked 5 times
    let mut cache: HashMap<i64, Vec<i64>> = HashMap::new();

    let remainder = blinks % BATCH;
    let batched_blinks = (blinks / BATCH) * BATCH;

    let mut initial_stones: Vec<i64> = Vec::new();
    if remainder > 0 {
        // Generate initial stone list by blinking a few times without caching
        for num in orig_stones.iter() {
            initial_stones.extend_from_slice(&blink_stone(*num, remainder));
        }
    } else {
        initial_stones = orig_stones;
    }

    println!("initial stones: {:?}", initial_stones);
    println!("blinks: {}", blinks);
    println!("remainder blinks: {}", remainder);
    println!("batched blinks: {}", batched_blinks);

    // Now, we will use caching
    let mut total: usize = 0;
    if batched_blinks > 0 {
        for num in initial_stones.iter() {
            println!("processing stone: {}", num);
            total += wink_stone(*num, batched_blinks, &mut cache);
        }
    } else {
        total = initial_stones.len();
    }
    total as i32
}

fn blink_stone(num: i64, blinks: usize) -> Vec<i64> {
    let mut digits: Vec<i64> = vec![num];
    for _ in 0..blinks {
        let mut current: Vec<i64> = Vec::new();
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

fn wink_stone(num: i64, blinks: usize, cache: &mut HashMap<i64, Vec<i64>>) -> usize {
    assert!(
        blinks % BATCH == 0,
        "Wink should have blinks divisible by BATCH"
    );
    if blinks == BATCH {
        return if let Some(entry) = cache.get(&num) {
            entry.len()
        } else {
            let stones = blink_stone(num, BATCH);
            let result = stones.len();
            cache.insert(num, stones);
            result
        };
    }

    let mut total: usize = 0;
    // Blink the stone 5 times
    let stones = blink_stone(num, BATCH);
    // Wink each stone minus BATCH
    for v in stones.iter() {
        total += wink_stone(*v, blinks - BATCH, cache);
    }
    total
}

fn parse_data(data: &str) -> Vec<i64> {
    if let Ok((_, items)) = parse_line(data) {
        return items;
    }
    vec![]
}

fn parse_line(data: &str) -> IResult<&str, Vec<i64>> {
    separated_list1(space1, complete::i64).parse(data)
}

fn get_num_digits(num: i64) -> Vec<i64> {
    if num == 0 {
        return vec![0];
    }

    let mut digits: Vec<i64> = Vec::new();
    let mut n = num;
    while n > 0 {
        digits.push(n % 10);
        n /= 10;
    }
    digits.reverse();
    digits
}

fn merge_digits(digits: &[i64]) -> i64 {
    let result: i64 = digits
        .iter()
        .enumerate()
        .rev()
        .map(|(k, v)| {
            let d = 10_i64.pow((digits.len() - k - 1) as u32);
            d * v
        })
        .sum();
    result
}

fn split_digits(digits: Vec<i64>) -> (i64, i64) {
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
