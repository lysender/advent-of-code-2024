use nom::{
    character::complete::{self, char, line_ending, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};

const PLUS: char = '+';
const TIMES: char = '*';
const CAT: char = '|';

const OPS: [char; 2] = ['+', '*'];
const OPS2: [char; 3] = ['+', '*', '|'];

#[derive(Debug)]
struct Equation {
    result: i64,
    numbers: Vec<i64>,
}

pub fn part1(input: &str) -> i64 {
    solve_puzzle(input)
}

pub fn part2(input: &str) -> i64 {
    solve_puzzle2(input)
}

fn solve_puzzle(data: &str) -> i64 {
    let items = parse_data(data);
    items
        .iter()
        .map(|item| {
            let ops = get_valid_ops(item, &OPS);
            if ops.len() > 0 {
                return item.result;
            }
            return 0;
        })
        .sum()
}

fn solve_puzzle2(data: &str) -> i64 {
    let items = parse_data(data);
    items
        .iter()
        .map(|item| {
            let ops = get_valid_ops(item, &OPS2);
            if ops.len() > 0 {
                return item.result;
            }
            return 0;
        })
        .sum()
}

fn get_valid_ops(eq: &Equation, ops: &[char]) -> Vec<char> {
    let all_ops = generate_ops(eq.numbers.len() - 1, ops);
    for v in all_ops.iter() {
        let result = solve_eq(&eq.numbers, v);
        if result == eq.result {
            return ops.to_vec();
        }
    }
    vec![]
}

fn generate_ops(size: usize, ops: &[char]) -> Vec<Vec<char>> {
    let mut all_ops: Vec<Vec<char>> = Vec::new();
    let mut current: Vec<char> = vec![PLUS; size];
    generate_ops_inner(size, ops, &mut current, &mut all_ops);
    all_ops
}

fn generate_ops_inner(
    size: usize,
    ops: &[char],
    current: &mut Vec<char>,
    all_ops: &mut Vec<Vec<char>>,
) {
    if size == 0 {
        // A full list of operators just got completed
        all_ops.push(current.clone());
        return;
    }

    for ch in ops.iter() {
        current[size - 1] = *ch;
        generate_ops_inner(size - 1, ops, current, all_ops);
    }
}

fn solve_eq(numbers: &Vec<i64>, operators: &Vec<char>) -> i64 {
    assert!(numbers.len() > 0, "Numbers size should be > 0");
    assert_eq!(
        numbers.len(),
        operators.len() + 1,
        "Operators size must be less than 1 compared to the numbers size"
    );
    let mut answer = numbers[0];
    for (k, op) in operators.iter().enumerate() {
        let right = numbers[k + 1];
        if op == &PLUS {
            answer += right;
        } else if op == &TIMES {
            answer *= right;
        } else if op == &CAT {
            let str_num = format!("{}{}", answer, right);
            answer = str_num.parse().unwrap();
        }
    }
    answer
}

fn parse_data(data: &str) -> Vec<Equation> {
    if let Ok((_, items)) = separated_list1(line_ending, eq_line_parser).parse(data) {
        let result: Vec<Equation> = items
            .into_iter()
            .map(|(ans, nums)| Equation {
                result: ans,
                numbers: nums,
            })
            .collect();
        return result;
    }
    vec![]
}

fn eq_line_parser(data: &str) -> IResult<&str, (i64, Vec<i64>)> {
    separated_pair(complete::i64, char(':'), numbers_parser).parse(data)
}

fn numbers_parser(data: &str) -> IResult<&str, Vec<i64>> {
    separated_list1(space1, complete::i64).parse(data.trim())
}

#[cfg(test)]
mod tests {
    use input::get_puzzle_input;

    use super::*;

    #[test]
    fn test_solve_eq() {
        let numbers: Vec<i64> = vec![10, 20, 30, 40];
        let ops: Vec<char> = vec![PLUS, PLUS, PLUS];
        assert_eq!(100, solve_eq(&numbers, &ops));
    }

    #[test]
    fn test_solve_eq_mixed() {
        let numbers: Vec<i64> = vec![10, 20, 30, 40];
        let ops: Vec<char> = vec![TIMES, PLUS, PLUS];
        assert_eq!(270, solve_eq(&numbers, &ops));
    }

    #[test]
    fn test_gen_ops() {
        let ops = generate_ops(4, &OPS);
        assert_eq!(16, ops.len());
    }

    #[test]
    fn test_gen_ops2() {
        let ops = generate_ops(4, &OPS2);
        assert_eq!(81, ops.len());
    }

    #[test]
    fn test_part1() {
        let input = get_puzzle_input("07-sample");
        let result = solve_puzzle(input.as_str());
        assert_eq!(result, 3749);
    }

    #[test]
    fn test_part2() {
        let input = get_puzzle_input("07-sample");
        let result = solve_puzzle2(input.as_str());
        assert_eq!(result, 11387);
    }
}
