use nom::{
    character::complete::{self, char, line_ending},
    multi::separated_list1,
    sequence::separated_pair,
    FindSubstring, IResult, Parser,
};

pub fn part1(data: &str) -> i32 {
    solve_puzzle(data)
}

pub fn part2(data: &str) -> i32 {
    solve_puzzle(data)
}

#[derive(Debug)]
struct Rule {
    left: i32,
    right: i32,
}

fn solve_puzzle(data: &str) -> i32 {
    let (rules, pages) = parse_rules(data);
    if rules.len() > 0 && pages.len() > 0 {
        let mut result: i32 = 0;

        for i in 0..pages.len() {
            let cur_pages = &pages[i];
            if valid_pages(&rules, cur_pages) {
                result += find_middle_val(cur_pages);
            }
        }
        return result;
    }
    0
}

fn parse_rules(data: &str) -> (Vec<Rule>, Vec<Vec<i32>>) {
    // Split sections
    let split_pos = data.find_substring("\n\n");
    if let Some(pos) = split_pos {
        let rules_str = &data[0..pos];
        let pages_str = &data[(pos + 2)..];

        let rules = rules_parser(rules_str);
        let pages = pages_parser(pages_str);
        return (rules, pages);
    }
    (vec![], vec![])
}

fn rules_parser(data: &str) -> Vec<Rule> {
    if let Ok((_, items)) = separated_list1(line_ending, rule_parser).parse(data) {
        let result: Vec<Rule> = items
            .iter()
            .map(|item| Rule {
                left: item.0,
                right: item.1,
            })
            .collect();
        return result;
    }
    vec![]
}

fn rule_parser(data: &str) -> IResult<&str, (i32, i32)> {
    separated_pair(complete::i32, char('|'), complete::i32).parse(data)
}

fn pages_parser(data: &str) -> Vec<Vec<i32>> {
    if let Ok((_, items)) = separated_list1(line_ending, page_parser).parse(data) {
        return items;
    }
    vec![]
}

fn page_parser(data: &str) -> IResult<&str, Vec<i32>> {
    separated_list1(char(','), complete::i32).parse(data)
}

fn find_middle_val(pages: &Vec<i32>) -> i32 {
    assert!(pages.len() % 2 != 0, "Pages must be odd");
    let index: usize = pages.len() / 2;
    pages[index]
}

fn valid_pages(rules: &Vec<Rule>, pages: &Vec<i32>) -> bool {
    for i in 0..pages.len() {
        if !valid_page(i, rules, pages) {
            return false;
        }
    }
    true
}

fn valid_page(index: usize, rules: &Vec<Rule>, pages: &Vec<i32>) -> bool {
    valid_left_rules(index, rules, pages) && valid_right_rules(index, rules, pages)
}

fn valid_left_rules(index: usize, rules: &Vec<Rule>, pages: &Vec<i32>) -> bool {
    if index <= 0 {
        // Nothing else to check beyond this point
        return true;
    }

    let val = pages[index];

    // When val is the left of the rule, we should not find any right values to the left
    let right_vals: Vec<i32> = rules
        .iter()
        .filter(|rule| rule.left == val)
        .map(|rule| rule.right)
        .collect();

    // Ensure that page doesn't violate anything to the left
    let map = &pages[..index];
    for r in right_vals.iter() {
        if let Some(_) = map.iter().find(|x| *x == r) {
            return false;
        }
    }

    true
}

fn valid_right_rules(index: usize, rules: &Vec<Rule>, pages: &Vec<i32>) -> bool {
    if index >= pages.len() - 1 {
        // Nothing else to check beyond this point
        return true;
    }

    let val = pages[index];

    // When val is the right of the rule, we should not find any left values to the right
    let left_vals: Vec<i32> = rules
        .iter()
        .filter(|rule| rule.right == val)
        .map(|rule| rule.left)
        .collect();

    // Ensure that page doesn't violate anything to the right
    let map = &pages[index..];
    for r in left_vals.iter() {
        if let Some(_) = map.iter().find(|x| *x == r) {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use input::get_puzzle_input;

    use super::*;

    #[test]
    fn test_middle_val() {
        let arr: Vec<i32> = vec![1, 2, 3, 4, 5];
        let mid = find_middle_val(&arr);
        assert_eq!(mid, 3);
    }

    #[test]
    fn test_part1() {
        let input = get_puzzle_input("05-sample");
        let result = solve_puzzle(input.as_str());
        assert_eq!(result, 143);
    }

    #[test]
    fn test_part2() {
        let input = get_puzzle_input("05-sample");
        let result = solve_puzzle(input.as_str());
        assert_eq!(result, 143);
    }
}
