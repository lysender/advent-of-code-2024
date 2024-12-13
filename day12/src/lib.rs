use std::collections::{HashSet, VecDeque};

use glam::IVec2;

pub fn part1(input: &str) -> i32 {
    solve_puzzle(input)
}

pub fn part2(input: &str) -> i32 {
    solve_puzzle(input)
}

fn solve_puzzle(data: &str) -> i32 {
    let grid = parse_data(data);
    println!("{:?}", grid);

    let mut regions: Vec<Region> = Vec::new();
    let mut surveyed = 

    for x in 0..=grid.max_x {
        for y in 0..=grid.max_y {
            let pos = IVec2::new(x, y);
            if let Some(region) = survey_area(&grid, &pos) {
                println!("{:?}", region);
            }
        }
    }
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

fn survey_area(grid: &Grid, pos: &IVec2) -> Option<Region> {
    let Some(plant) = grid.find_item(pos.x, pos.y) else {
        return None;
    };

    let row: Vec<bool> = vec![false; grid.max_y as usize + 1];
    let mut visited: Vec<Vec<bool>> = vec![row; grid.max_x as usize + 1];
    let mut queue: VecDeque<IVec2> = VecDeque::new();
    let mut region = Region::new(*plant);

    visited[pos.x as usize][pos.y as usize] = true;
    queue.push_back(*pos);

    while let Some(current) = queue.pop_front() {
        region.add_coord(current);

        // Find neighbors
        let around: Vec<IVec2> = vec![
            IVec2::new(0, 1),
            IVec2::new(1, 0),
            IVec2::new(0, -1),
            IVec2::new(-1, 0),
        ];

        for npos in around.iter() {
            let next = npos + current;
            // Find sorrounding plants of the same species
            if let Some(next_item) = grid.find_item(next.x, next.y) {
                if next_item == plant && !visited[next.x as usize][next.y as usize] {
                    visited[next.x as usize][next.y as usize] = true;
                    queue.push_back(next);
                }
            }
        }
    }

    Some(region)
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

#[derive(Debug)]
struct Region {
    plant: char,
    coords: HashSet<IVec2>,
}

impl Region {
    fn new(plant: char) -> Self {
        Self {
            plant,
            coords: HashSet::new(),
        }
    }

    fn set_coords(&mut self, coords: HashSet<IVec2>) {
        self.coords = coords;
    }

    fn add_coord(&mut self, coord: IVec2) {
        self.coords.insert(coord);
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

    //#[test]
    //fn test_part1_sample2() {
    //    let input = get_puzzle_input("12-sample2");
    //    let result = solve_puzzle(input.as_str());
    //    assert_eq!(result, 172);
    //}
    //
    //#[test]
    //fn test_part1_sample3() {
    //    let input = get_puzzle_input("12-sample3");
    //    let result = solve_puzzle(input.as_str());
    //    assert_eq!(result, 1930);
    //}

    #[test]
    fn test_part2() {
        let input = get_puzzle_input("12-sample");
        let result = solve_puzzle(input.as_str());
        assert_eq!(result, 0);
    }
}
