use glam::IVec2;

pub fn part1(input: &str) -> i32 {
    solve_puzzle(input)
}

pub fn part2(input: &str) -> i32 {
    solve_puzzle(input)
}

fn solve_puzzle(data: &str) -> i32 {
    let plots = parse_data(data);
    println!("{:?}", plots);
    0
}

fn parse_data(data: &str) -> Grid {
    let result: Vec<Vec<char>> = data
        .lines()
        .map(|line| {
            let row: Vec<char> = line.chars().collect();
            row
        })
        .collect();

    Grid::new(result)
}

#[derive(Debug)]
struct Grid {
    rows: Vec<Vec<char>>,
    max_x: i32,
    max_y: i32,
}

impl Grid {
    fn new(rows: Vec<Vec<char>>) -> Self {
        let max_x = (rows.len() as i32) - 1;
        assert!(max_x > 0, "Grid rows must be greater than 0");
        let max_y = (rows[0].len() as i32) - 1;
        assert!(max_y > 0, "Grid columns must be greater than 0");

        Self { rows, max_x, max_y }
    }

    fn find_item(&self, x: i32, y: i32) -> Option<&char> {
        if x >= 0 && y >= 0 && x <= self.max_x && y <= self.max_y {
            return Some(&self.rows[x as usize][y as usize]);
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use input::get_puzzle_input;

    use super::*;

    #[test]
    fn test_part1() {
        let input = get_puzzle_input("12-sample");
        let result = solve_puzzle(input.as_str());
        assert_eq!(result, 140);
    }

    #[test]
    fn test_part1_sample2() {
        let input = get_puzzle_input("12-sample2");
        let result = solve_puzzle(input.as_str());
        assert_eq!(result, 172);
    }

    #[test]
    fn test_part1_sample3() {
        let input = get_puzzle_input("12-sample3");
        let result = solve_puzzle(input.as_str());
        assert_eq!(result, 1930);
    }

    #[test]
    fn test_part2() {
        let input = get_puzzle_input("12-sample");
        let result = solve_puzzle(input.as_str());
        assert_eq!(result, 0);
    }
}
