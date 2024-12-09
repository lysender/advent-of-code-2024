pub fn part1(input: &str) -> i32 {
    solve_puzzle(input)
}

pub fn part2(input: &str) -> i32 {
    solve_puzzle(input)
}

fn solve_puzzle(_input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use input::get_puzzle_input;

    use super::*;

    #[test]
    fn test_part1() {
        let input = get_puzzle_input("09-sample");
        let result = solve_puzzle(input.as_str());
        assert_eq!(result, 1928);
    }

    #[test]
    fn test_part2() {
        let input = get_puzzle_input("09-sample");
        let result = solve_puzzle(input.as_str());
        assert_eq!(result, 1928);
    }
}
