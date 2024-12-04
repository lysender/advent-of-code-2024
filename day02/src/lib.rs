use nom::{
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    IResult, Parser,
};

pub fn part1(input: &str) -> i32 {
    compute_safe_reports(input)
}

pub fn part2(input: &str) -> i32 {
    compute_safe_reports_with_dampener(input)
}

fn compute_safe_reports(input: &str) -> i32 {
    if let Ok((_, reports)) = parse_report(input) {
        let total: i32 = reports.iter().map(|report| is_safe_count(report)).sum();
        return total;
    }
    0
}

fn compute_safe_reports_with_dampener(input: &str) -> i32 {
    if let Ok((_, reports)) = parse_report(input) {
        let total: i32 = reports
            .iter()
            .map(|report| is_safe_count_dampened(report))
            .sum();
        return total;
    }
    0
}

fn is_safe_count(levels: &Vec<u32>) -> i32 {
    match is_safe(levels) {
        true => 1,
        false => 0,
    }
}

fn is_safe_count_dampened(levels: &Vec<u32>) -> i32 {
    if is_safe(levels) {
        return 1;
    }
    if is_safe_dampened(levels) {
        return 1;
    }
    0
}

fn is_safe(levels: &Vec<u32>) -> bool {
    let mut asc: Option<bool> = None;
    for x in levels.windows(2) {
        let diff: i32 = x[0] as i32 - x[1] as i32;
        if diff == 0 {
            // Not safe
            return false;
        } else if diff > 0 {
            if diff > 3 {
                return false;
            }
            if let Some(asc_val) = asc {
                if !asc_val {
                    return false;
                }
            } else {
                asc = Some(true);
            }
        } else if diff < 0 {
            if diff < -3 {
                return false;
            }
            if let Some(asc_val) = asc {
                if asc_val {
                    return false;
                }
            } else {
                asc = Some(false);
            }
        }
    }
    true
}

fn is_safe_dampened(levels: &Vec<u32>) -> bool {
    let source = levels.clone();

    // Assumes levels is already unsafe
    // Brute force it
    for i in 0..levels.len() {
        let mut dampened = source.clone();
        dampened.remove(i);
        if is_safe(&dampened) {
            return true;
        }
    }
    false
}

fn parse_report(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    separated_list1(line_ending, row_parser).parse(input)
}

fn row_parser(line: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(space1, complete::u32).parse(line)
}

#[cfg(test)]
mod tests {
    use input::get_puzzle_input;

    use super::*;

    #[test]
    fn test_part1() {
        let input = get_puzzle_input("02-sample");
        let total = compute_safe_reports(input.as_str());
        assert_eq!(total, 2);
    }

    #[test]
    fn test_part2() {
        let input = get_puzzle_input("02-sample");
        let total = compute_safe_reports_with_dampener(input.as_str());
        assert_eq!(total, 4);
    }
}
