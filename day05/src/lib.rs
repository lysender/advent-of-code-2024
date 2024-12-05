use std::collections::HashMap;

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
    solve_puzzle2(data)
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

fn solve_puzzle2(data: &str) -> i32 {
    let (rules, pages) = parse_rules(data);
    if rules.len() > 0 && pages.len() > 0 {
        let invalid_pages = collect_invalid_pages(&rules, &pages);
        // println!("Invalid list: {:?}", invalid_pages);
        let mut result: i32 = 0;

        for i in 0..invalid_pages.len() {
            let cur_pages = &invalid_pages[i];
            let fixed = fix_invalid_pages(&rules, cur_pages);
            // println!("Fixed: {:?}", fixed);
            result += find_middle_val(&fixed);
        }
        return result;
    }
    0
}

fn collect_invalid_pages(rules: &Vec<Rule>, pages: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();
    for i in 0..pages.len() {
        let cur_pages = &pages[i];
        if !valid_pages(rules, cur_pages) {
            result.push(cur_pages.clone());
        }
    }

    result
}

fn fix_invalid_pages(rules: &Vec<Rule>, pages: &Vec<i32>) -> Vec<i32> {
    let mut result = pages.clone();
    let map = make_map(pages);

    let mut left_valid = false;
    let mut breaker = 0;
    while !left_valid {
        // println!("Reset from top");
        let mut valid_count = 0;
        for i in 0..pages.len() {
            // Find the broken element
            if let Err(pos) = valid_left_rules(i, rules, pages, &map) {
                // println!("LEFT: Broken page: {} at {}", pages[pos], pos);
                // Move broken value to the right until it becomes valid or at the end
                let mut is_broken = true;
                let mut cur_pos = pos;
                while is_broken && cur_pos < pages.len() - 1 {
                    // println!("{:?}", result);
                    // println!("swap values to the right");
                    let tmp = result[cur_pos];
                    result[cur_pos] = result[cur_pos + 1];
                    result[cur_pos + 1] = tmp;
                    cur_pos += 1;

                    is_broken = valid_left_rules(i, rules, pages, &map).is_err();
                }
            } else {
                valid_count += 1;
            }
        }
        // println!("valid_count: {}", valid_count);
        left_valid = valid_count == pages.len();

        breaker += 1;
        if breaker > 100 {
            // println!("breaker...");
            break;
        }
    }

    for i in 1..pages.len() {
        // Find broken element
        if let Err(pos) = valid_right_rules(i, rules, pages, &map) {
            // println!("RIGHT: Broken page: {} at {}", pages[pos], pos);
        }
    }

    result
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

fn make_map(pages: &Vec<i32>) -> HashMap<i32, usize> {
    let mut map = HashMap::new();
    for (k, v) in pages.iter().enumerate() {
        map.insert(*v, k);
    }
    map
}

fn valid_pages(rules: &Vec<Rule>, pages: &Vec<i32>) -> bool {
    let map = make_map(pages);
    for i in 0..pages.len() {
        if !valid_page(i, rules, pages, &map) {
            return false;
        }
    }
    true
}

fn valid_page(
    index: usize,
    rules: &Vec<Rule>,
    pages: &Vec<i32>,
    map: &HashMap<i32, usize>,
) -> bool {
    valid_left_rules(index, rules, pages, map).is_ok()
        && valid_right_rules(index, rules, pages, map).is_ok()
}

fn valid_left_rules(
    index: usize,
    rules: &Vec<Rule>,
    pages: &Vec<i32>,
    map: &HashMap<i32, usize>,
) -> Result<(), usize> {
    if index <= 0 {
        // Nothing else to check beyond this point
        return Ok(());
    }

    let val = pages[index];

    // When val is the left of the rule, we should not find any right values to the left
    let right_vals: Vec<i32> = rules
        .iter()
        .filter(|rule| rule.left == val)
        .map(|rule| rule.right)
        .collect();

    // Ensure that page doesn't violate anything to the left
    for r in right_vals.iter() {
        if let Some(pos) = map.get(r) {
            if pos < &index {
                return Err(*pos);
            }
        }
    }

    Ok(())
}

fn valid_right_rules(
    index: usize,
    rules: &Vec<Rule>,
    pages: &Vec<i32>,
    map: &HashMap<i32, usize>,
) -> Result<(), usize> {
    if index >= pages.len() - 1 {
        // Nothing else to check beyond this point
        return Ok(());
    }

    let val = pages[index];

    // When val is the right of the rule, we should not find any left values to the right
    let left_vals: Vec<i32> = rules
        .iter()
        .filter(|rule| rule.right == val)
        .map(|rule| rule.left)
        .collect();

    // Ensure that page doesn't violate anything to the right
    for l in left_vals.iter() {
        if let Some(pos) = map.get(l) {
            if &index < pos {
                return Err(*pos);
            }
        }
    }

    Ok(())
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
        let result = solve_puzzle2(input.as_str());
        assert_eq!(result, 123);
    }
}
