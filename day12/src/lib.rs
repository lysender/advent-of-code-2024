use std::{
    cmp::Ordering,
    collections::{HashSet, VecDeque},
};

use glam::IVec2;
use grid::{coord_greater, create_visited_grid};
use itertools::Itertools;

pub fn part1(input: &str) -> i32 {
    solve_puzzle(input)
}

pub fn part2(input: &str) -> i32 {
    solve_puzzle_discounted(input)
}

fn solve_puzzle(data: &str) -> i32 {
    let grid = parse_data(data);

    // Collect all regions
    let mut surveyed = create_visited_grid(grid.rows as usize, grid.cols as usize);
    let mut perimeter: i32 = 0;

    for x in 0..grid.rows {
        for y in 0..grid.cols {
            let pos = IVec2::new(x as i32, y as i32);
            if let Some(region) = survey_area(&grid, &pos, &mut surveyed) {
                perimeter += region.compute_fence_cost();
            }
        }
    }

    perimeter
}

fn solve_puzzle_discounted(data: &str) -> i32 {
    let grid = parse_data(data);

    // Collect all regions
    let mut surveyed = create_visited_grid(grid.rows as usize, grid.cols as usize);
    let mut cost: i32 = 0;
    let mut regions: Vec<Region> = Vec::new();

    for x in 0..grid.rows {
        for y in 0..grid.cols {
            let pos = IVec2::new(x as i32, y as i32);
            if let Some(region) = survey_area(&grid, &pos, &mut surveyed) {
                regions.push(region);
            }
        }
    }

    // Compute region fence cost using a discounted pattern
    // where 1 straight line is only counted as 1 side
    // simply count the number of turns
    for region in regions.iter() {
        let turns = compute_outer_turns(&grid, region);
        let region_cost = turns * region.coords.len() as i32;
        cost += region_cost;
    }

    cost
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

    // Use BFS to map all contagious plants that forms an area/polygon
    let mut visited: Vec<Vec<bool>> = create_visited_grid(grid.rows as usize, grid.cols as usize);
    let mut queue: VecDeque<IVec2> = VecDeque::new();
    let mut region = Region::new(plant.clone());

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

fn compute_outer_turns(grid: &Grid, region: &Region) -> i32 {
    let coords = region.sorted_coords();
    let mut turns: i32 = 0;

    // Start at the leftmost top edge and face upwards
    let orig_coord = coords[0];
    let orig_dir = Dir::Up;

    let mut curr = orig_coord.clone();
    let mut dir = orig_dir;

    // Breaker
    let mut count = 0;

    loop {
        match get_next_item(grid, region, &curr, dir) {
            Some(next) => {
                // Found an item, try the next item on the same direction
                curr = next;
            }
            None => {
                // Turn and try to get the next item
                dir = match dir {
                    Dir::Up => Dir::Right,
                    Dir::Right => Dir::Down,
                    Dir::Down => Dir::Left,
                    Dir::Left => Dir::Up,
                };

                turns += 1;
            }
        };

        if curr == orig_coord && dir == orig_dir {
            // Arrived at where we started
            break;
        }

        if count > 1000 {
            println!("Breaker...");
            break;
        }
        count += 1;
    }

    turns + 1
}

fn get_next_item(grid: &Grid, region: &Region, current: &IVec2, dir: Dir) -> Option<IVec2> {
    // Move forward based on the direction
    let pos = match dir {
        Dir::Up => current + IVec2::new(-1, 0),
        Dir::Right => current + IVec2::new(0, 1),
        Dir::Down => current + IVec2::new(1, 0),
        Dir::Left => current + IVec2::new(0, -1),
    };

    if !region.coords.contains(&pos) {
        return None;
    }
    let Some(item) = grid.find_item(&pos) else {
        return None;
    };
    if item != &region.plant {
        return None;
    }

    Some(pos)
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
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

    fn add_coord(&mut self, coord: IVec2) {
        self.coords.insert(coord);
    }

    fn compute_fence_cost(&self) -> i32 {
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

    fn sorted_coords(&self) -> Vec<IVec2> {
        let mut coords: Vec<IVec2> = self.coords.clone().into_iter().collect_vec();
        // Sort coords
        coords.sort_by(|a, b| {
            if coord_greater(a, b) {
                Ordering::Greater
            } else if a == b {
                Ordering::Equal
            } else {
                Ordering::Less
            }
        });
        coords
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
        let result = solve_puzzle_discounted(input.as_str());
        assert_eq!(result, 80);
    }

    #[test]
    fn test_part2_sample2() {
        let input = get_puzzle_input("12-sample2");
        let result = solve_puzzle_discounted(input.as_str());
        assert_eq!(result, 436);
    }

    #[test]
    fn test_part2_sample3() {
        let input = get_puzzle_input("12-sample3");
        let result = solve_puzzle_discounted(input.as_str());
        assert_eq!(result, 1206);
    }

    #[test]
    fn test_part2_sample4() {
        let input = get_puzzle_input("12-sample4");
        let result = solve_puzzle_discounted(input.as_str());
        assert_eq!(result, 236);
    }

    #[test]
    fn test_part2_sample5() {
        let input = get_puzzle_input("12-sample5");
        let result = solve_puzzle_discounted(input.as_str());
        assert_eq!(result, 368);
    }
}
