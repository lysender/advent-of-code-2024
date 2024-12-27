use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet, VecDeque},
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
    let mut next_id: usize = 1;

    for x in 0..grid.rows {
        for y in 0..grid.cols {
            let pos = IVec2::new(x as i32, y as i32);
            if let Some(region) = survey_area(&grid, &pos, &mut surveyed, next_id) {
                perimeter += region.compute_cost();
                next_id += 1;
            }
        }
    }

    perimeter
}

fn solve_puzzle_discounted(data: &str) -> i32 {
    let grid = parse_data(data);

    // Collect all regions
    let mut surveyed = create_visited_grid(grid.rows as usize, grid.cols as usize);
    // Each region coord mapped to a region ID
    let mut plant_map: HashMap<IVec2, usize> = HashMap::new();
    let mut region_map: HashMap<usize, Region> = HashMap::new();

    let mut cost: usize = 0;
    let mut next_id: usize = 1;

    for x in 0..grid.rows {
        for y in 0..grid.cols {
            let pos = IVec2::new(x as i32, y as i32);
            if let Some(mut region) = survey_area(&grid, &pos, &mut surveyed, next_id) {
                // Map each coord into a region ID
                for coord in region.coords.iter() {
                    plant_map.insert(*coord, region.id);
                }

                // Compute edges early on
                region.compute_edges(&grid);
                region_map.insert(region.id, region);
                next_id += 1;
            }
        }
    }

    // Compute region fence cost using a discounted pattern
    // where 1 straight line is only counted as 1 side
    // simply count the number of turns
    for (_, region) in region_map.iter() {
        // Compute for the outer perimeter
        let outer_cost = region.sides * region.coords.len();
        let mut inner_cost = 0;

        println!("outer region: {:?}", region);

        // Try all other regions if they are inside,
        // then simply compute via sides
        for (k, inner_region) in region_map.iter() {
            if k != &region.id {
                if region_within(&grid, region, inner_region) {
                    println!("{} is within {}", inner_region.plant, region.plant);
                    inner_cost += inner_region.sides * inner_region.coords.len();
                }
            }
        }

        cost += inner_cost + outer_cost;
    }

    cost as i32
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

fn survey_area(
    grid: &Grid,
    pos: &IVec2,
    surveyed: &mut Vec<Vec<bool>>,
    next_id: usize,
) -> Option<Region> {
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
    let mut region = Region::new(next_id, *plant);

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

fn region_within(grid: &Grid, current_region: &Region, region: &Region) -> bool {
    // Get the first item of the region and use ray tracing
    // to know if region is inside current region
    let mut current = Some(IVec2::new(region.min.x, 0));
    let mut passes: usize = 0;
    let mut hit: bool = false;

    while let Some(pos) = current {
        if pos == region.min {
            hit = true;
        }

        if grid.find_item(&pos).is_none() {
            break;
        }
        if hit && current_region.edges.contains(&pos) {
            passes += 1;
        }
        current = Some(pos + IVec2::new(0, 1));
    }

    if passes == 0 {
        return false;
    }

    passes % 2 == 0
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

/// Turn directions based on current orientation
fn dir_dirs(dir: Dir) -> (Dir, Dir) {
    match dir {
        Dir::Up => (Dir::Left, Dir::Right),
        Dir::Right => (Dir::Up, Dir::Down),
        Dir::Down => (Dir::Right, Dir::Left),
        Dir::Left => (Dir::Down, Dir::Up),
    }
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
    id: usize,
    plant: char,
    sides: usize,
    coords: HashSet<IVec2>,
    edges: HashSet<IVec2>,
    min: IVec2,
    max: IVec2,
}

impl Region {
    fn new(id: usize, plant: char) -> Self {
        Self {
            id,
            plant,
            sides: 0,
            coords: HashSet::new(),
            edges: HashSet::new(),
            min: IVec2::new(0, 0),
            max: IVec2::new(0, 0),
        }
    }

    fn add_coord(&mut self, coord: IVec2) {
        self.coords.insert(coord);
    }

    fn compute_cost(&self) -> i32 {
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

    fn compute_edges(&mut self, grid: &Grid) {
        let coords = self.sorted_coords();
        if coords.len() == 0 {
            return;
        }

        self.min = *coords.first().unwrap();
        self.max = *coords.last().unwrap();

        let mut turns: usize = 0;

        if coords.len() == 1 {
            self.edges.insert(self.min.clone());
            self.sides = 4;
            return;
        }

        // Start at the leftmost top edge and face right
        let orig_coord = self.min;
        let orig_dir = Dir::Right;

        let mut curr = orig_coord.clone();
        let mut dir = orig_dir;

        loop {
            let (out_dir, in_dir) = dir_dirs(dir);
            // Try to move outward first
            if let Some(next) = self.get_next_item(grid, &curr, out_dir) {
                // We just turned
                self.edges.insert(next.clone());
                curr = next;
                dir = out_dir;
                turns += 1;
                continue;
            }

            // Then try moving forward
            if let Some(next) = self.get_next_item(grid, &curr, dir) {
                self.edges.insert(next.clone());
                // Move forward, no turn
                curr = next;
                continue;
            }

            // Lastly, try moving inward
            if let Some(next) = self.get_next_item(grid, &curr, in_dir) {
                if !self.edges.contains(&next) {
                    // We just turned with a match
                    self.edges.insert(next.clone());
                    curr = next;
                }
            }

            // If we don't hit anything, just turn inward
            dir = in_dir;
            turns += 1;

            if curr == orig_coord && dir == orig_dir {
                // Arrived at where we started
                break;
            }
        }

        println!("turns: {}", turns);
        self.sides = turns;
    }

    fn get_next_item(&self, grid: &Grid, current: &IVec2, dir: Dir) -> Option<IVec2> {
        // Move forward based on the direction
        let pos = match dir {
            Dir::Up => current + IVec2::new(-1, 0),
            Dir::Right => current + IVec2::new(0, 1),
            Dir::Down => current + IVec2::new(1, 0),
            Dir::Left => current + IVec2::new(0, -1),
        };

        if !self.coords.contains(&pos) {
            return None;
        }
        let Some(item) = grid.find_item(&pos) else {
            return None;
        };
        if item != &self.plant {
            return None;
        }

        Some(pos)
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

    //#[test]
    //fn test_part2_sample3() {
    //    let input = get_puzzle_input("12-sample3");
    //    let result = solve_puzzle_discounted(input.as_str());
    //    assert_eq!(result, 1206);
    //}
    //
    //#[test]
    //fn test_part2_sample4() {
    //    let input = get_puzzle_input("12-sample4");
    //    let result = solve_puzzle_discounted(input.as_str());
    //    assert_eq!(result, 236);
    //}
    //
    //#[test]
    //fn test_part2_sample5() {
    //    let input = get_puzzle_input("12-sample5");
    //    let result = solve_puzzle_discounted(input.as_str());
    //    assert_eq!(result, 368);
    //}
}
