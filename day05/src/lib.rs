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

#[derive(Debug)]
struct PageRuleMap {
    map: HashMap<i32, Vec<i32>>,
    rules: Vec<Rule>,
}

impl PageRuleMap {
    fn new(rules: Vec<Rule>) -> Self {
        PageRuleMap {
            map: HashMap::new(),
            rules,
        }
    }

    fn register_rules(&mut self, page: i32) {
        if self.map.get(&page).is_some() {
            return;
        }

        let rights: Vec<i32> = self
            .rules
            .iter()
            .filter_map(|r| {
                if r.left == page {
                    return Some(r.right);
                }
                None
            })
            .collect();

        self.map.insert(page, rights);
    }

    fn valid_pages(&mut self, pages: &Vec<i32>) -> bool {
        for i in 0..pages.len() {
            if !self.valid_page(i, pages) {
                return false;
            }
        }
        true
    }

    fn valid_page(&mut self, index: usize, pages: &Vec<i32>) -> bool {
        self.valid_lefts(index, pages).is_ok()
    }

    fn valid_lefts(&mut self, index: usize, pages: &Vec<i32>) -> Result<(), usize> {
        if index <= 0 {
            // Nothing else to check beyond this point
            return Ok(());
        }

        let val = pages[index];
        self.register_rules(val);

        // All values to the left must not be listed in the right rules
        if let Some(rights) = self.map.get(&val) {
            let buffer = &pages[..index];
            for (k, v) in buffer.iter().enumerate() {
                if rights.contains(v) {
                    return Err(k);
                }
            }
        }

        Ok(())
    }
}

fn solve_puzzle(data: &str) -> i32 {
    let (rules, pages) = parse_rules(data);
    if rules.len() > 0 && pages.len() > 0 {
        let mut worker = PageRuleMap::new(rules);
        let mut result: i32 = 0;

        for i in 0..pages.len() {
            let cur_pages = &pages[i];
            if worker.valid_pages(cur_pages) {
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
        let mut worker = PageRuleMap::new(rules);
        let mut result: i32 = 0;

        for i in 0..pages.len() {
            let cur_pages = &pages[i];
            if !worker.valid_pages(cur_pages) {
                // Fix invalid page
                let fixed = fix_invalid_pages(&mut worker, cur_pages);
                // Get middle value and add result
                result += find_middle_val(&fixed);
            }
        }

        return result;
    }
    0
}

fn fix_invalid_pages(worker: &mut PageRuleMap, pages: &Vec<i32>) -> Vec<i32> {
    let mut result = pages.clone();

    let mut left_valid = false;
    let mut breaker = 0;
    while !left_valid {
        let mut valid_count = 0;
        for i in 0..pages.len() {
            // Find the broken element
            match worker.valid_lefts(i, &result) {
                Ok(_) => {
                    valid_count += 1;
                }
                Err(pos) => {
                    // Move the invalid element to the right side
                    let broken = result[pos];
                    result.remove(pos);
                    result.insert(i, broken);

                    // Keep moving it to the right until end of line or when all is valid
                    let mut inner_valid = false;
                    let mut ipos = i;

                    while ipos < pages.len() - 1 && !inner_valid {
                        inner_valid = worker.valid_lefts(i, &result).is_ok();
                        if !inner_valid {
                            // Swap value to the right
                            let tmp = result[ipos + 1];
                            result[ipos + 1] = result[ipos];
                            result[ipos] = tmp;
                            ipos += 1;
                        }
                    }
                }
            }
        }

        left_valid = valid_count == pages.len();

        breaker += 1;
        if breaker > 100 {
            println!("left breaker...");
            break;
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
