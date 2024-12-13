use std::collections::{HashSet, VecDeque};

use glam::IVec2;
use grid::create_visited_grid;

pub fn part1(input: &str) -> i32 {
    solve_puzzle(input)
}

pub fn part2(input: &str) -> i32 {
    solve_puzzle(input)
}

fn solve_puzzle(data: &str) -> i32 {
    let grid = parse_data(data);

    // Collect all regions
    let mut regions: Vec<Region> = Vec::new();
    let mut surveyed = create_visited_grid(grid.rows as usize, grid.cols as usize);

    for x in 0..grid.rows {
        for y in 0..grid.cols {
            let pos = IVec2::new(x as i32, y as i32);
            if let Some(region) = survey_area(&grid, &pos, &mut surveyed) {
                regions.push(region);
            }
        }
    }

    // Calculate all perimeters for each region
    let perimeter: i32 = regions.iter().map(|r| r.compute_perimeter()).sum();
    perimeter
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

fn survey_area(grid: &Grid, pos: &IVec2, surveyed: &mut Vec<Vec<bool>>) -> Option<Region> {
    let Some(plant) = grid.find_item(pos) else {
        // Off the grid
        return None;
    };

    if surveyed[pos.x as usize][pos.y as usize] {
        // Already surveyed
        return None;
    }

    let mut visited: Vec<Vec<bool>> = create_visited_grid(grid.rows as usize, grid.cols as usize);
    let mut queue: VecDeque<IVec2> = VecDeque::new();
    let mut region = Region::new();

    visited[pos.x as usize][pos.y as usize] = true;
    queue.push_back(*pos);

    let around: Vec<IVec2> = vec![
        IVec2::new(0, 1),
        IVec2::new(1, 0),
        IVec2::new(0, -1),
        IVec2::new(-1, 0),
    ];

    while let Some(current) = queue.pop_front() {
        // Mark as surveyed so we don't survey it again next time
        surveyed[current.x as usize][current.y as usize] = true;

        region.add_coord(current);

        // Find neighbors
        for npos in around.iter() {
            let next = npos + current;
            // Find sorrounding plants of the same species
            if let Some(next_item) = grid.find_item(&next) {
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
    matrix: Vec<Vec<char>>,
    rows: i32,
    cols: i32,
}

impl Grid {
    fn new(matrix: Vec<Vec<char>>) -> Self {
        let rows = matrix.len() as i32;
        assert!(rows > 0, "Grid rows must be greater than 0");
        let cols = matrix[0].len() as i32;
        assert!(cols > 0, "Grid columns must be greater than 0");

        Self { matrix, rows, cols }
    }

    fn find_item(&self, pos: &IVec2) -> Option<&char> {
        if pos.x >= 0 && pos.y >= 0 && pos.x < self.rows && pos.y < self.cols {
            return Some(&self.matrix[pos.x as usize][pos.y as usize]);
        }
        None
    }
}

#[derive(Debug)]
struct Region {
    coords: HashSet<IVec2>,
}

impl Region {
    fn new() -> Self {
        Self {
            coords: HashSet::new(),
        }
    }

    fn add_coord(&mut self, coord: IVec2) {
        self.coords.insert(coord);
    }

    fn compute_perimeter(&self) -> i32 {
        let mut perimeter: i32 = 0;
        let around: Vec<IVec2> = vec![
            IVec2::new(0, 1),
            IVec2::new(1, 0),
            IVec2::new(0, -1),
            IVec2::new(-1, 0),
        ];
        for pos in self.coords.iter() {
            // Assume perimeter as 4
            let mut current: i32 = 4;
            for side in around.iter() {
                let next = side + pos;
                // For every neighbor on a side, deduct 1 perimeter
                if self.coords.contains(&next) {
                    current -= 1;
                }
            }
            perimeter += current;
        }
        perimeter * self.coords.len() as i32
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
        assert_eq!(result, 772);
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
        assert_eq!(result, 140);
    }
}
