use std::collections::{HashSet, VecDeque};

use glam::IVec2;

pub fn part1(input: &str) -> i32 {
    solve_puzzle(input)
}

pub fn part2(input: &str) -> i32 {
    solve_puzzle_trails(input)
}

fn solve_puzzle(input: &str) -> i32 {
    let grid = parse_data(input);
    let mut trail_heads: HashSet<(IVec2, IVec2)> = HashSet::new();

    for start in grid.starts.iter() {
        find_trail_heads(&grid, start, &mut trail_heads);
    }

    // For each starting position, find all trails that leads to an end of trail
    trail_heads.len() as i32
}

fn solve_puzzle_trails(input: &str) -> i32 {
    let grid = parse_data(input);
    let mut trails: HashSet<Vec<IVec2>> = HashSet::new();

    for start in grid.starts.iter() {
        find_distinct_trails(&grid, start, &mut trails);
    }

    // For each starting position, find all trails that leads to an end of trail
    trails.len() as i32
}

fn find_trail_heads(grid: &Grid, start: &IVec2, result: &mut HashSet<(IVec2, IVec2)>) {
    // Use BFS (breadth first search) algorithm to find a trail
    // by spreading evenly until the peak is found
    let vrow: Vec<bool> = vec![false; grid.max_y as usize + 1];
    let mut visited: Vec<Vec<bool>> = vec![vrow; grid.max_x as usize + 1];
    let mut queue: VecDeque<IVec2> = VecDeque::new();
    queue.push_back(start.clone());

    // Mark the start visited
    visited[start.x as usize][start.y as usize] = true;

    let around: Vec<IVec2> = vec![
        IVec2::new(0, 1),
        IVec2::new(1, 0),
        IVec2::new(0, -1),
        IVec2::new(-1, 0),
    ];

    while let Some(q) = queue.pop_front() {
        if let Some(q_val) = grid.find_item(q.x, q.y) {
            if *q_val == 9 {
                // End of the trail, store the trail head
                result.insert((start.clone(), q));
            } else {
                // Find candidate next steps around the node
                for next in around.iter() {
                    let pos = q + next;
                    if let Some(next_val) = grid.find_item(pos.x, pos.y) {
                        // Candidate items must be one step higher than currently in queue
                        if *next_val == (q_val + 1) && *next_val <= 9 {
                            if !visited[pos.x as usize][pos.y as usize] {
                                visited[pos.x as usize][pos.y as usize] = true;
                                queue.push_back(pos);
                            }
                        }
                    }
                }
            }
        }
    }
}

fn find_distinct_trails(grid: &Grid, start: &IVec2, result: &mut HashSet<Vec<IVec2>>) {
    // Use DFS (Depth first search) algorithm to find a trail
    // by going deep first to capture the whole path
    let vrow: Vec<bool> = vec![false; grid.max_y as usize + 1];
    let mut visited: Vec<Vec<bool>> = vec![vrow; grid.max_x as usize + 1];
    let mut paths: Vec<IVec2> = Vec::new();
    paths.push(start.clone());
    find_trails_inner(grid, &mut visited, start, &paths, result);
}

fn find_trails_inner(
    grid: &Grid,
    visited: &mut Vec<Vec<bool>>,
    curr: &IVec2,
    paths: &Vec<IVec2>,
    result: &mut HashSet<Vec<IVec2>>,
) {
    if let Some(curr_val) = grid.find_item(curr.x, curr.y) {
        // Mark as visited
        visited[curr.x as usize][curr.y as usize] = true;

        if *curr_val == 9 {
            // Found a trail
            result.insert(paths.clone());
            return;
        }

        // Visit sorrounding nodes
        let around: Vec<IVec2> = vec![
            IVec2::new(0, 1),
            IVec2::new(1, 0),
            IVec2::new(0, -1),
            IVec2::new(-1, 0),
        ];

        for next in around.iter() {
            let pos = curr + next;
            if let Some(next_val) = grid.find_item(pos.x, pos.y) {
                // Candidate must be +1 greater than current value
                if *next_val == (curr_val + 1) && *next_val <= 9 {
                    let mut new_paths = paths.clone();
                    new_paths.push(pos);
                    find_trails_inner(grid, visited, &pos, &new_paths, result);
                }
            }
        }
    }
}

#[derive(Debug)]
struct Grid {
    rows: Vec<Vec<u8>>,
    max_x: i32,
    max_y: i32,
    starts: Vec<IVec2>,
}

impl Grid {
    fn new(rows: Vec<Vec<u8>>) -> Self {
        let max_x = (rows.len() as i32) - 1;
        assert!(max_x > 0, "Grid rows must be greater than 0");
        let max_y = (rows[0].len() as i32) - 1;
        assert!(max_y > 0, "Grid columns must be greater than 0");

        let mut starts: Vec<IVec2> = Vec::new();

        for x in 0..=max_x {
            for y in 0..=max_y {
                let item = rows[x as usize][y as usize];
                if item == 0 {
                    starts.push(IVec2::new(x, y));
                }
            }
        }

        Self {
            rows,
            max_x,
            max_y,
            starts,
        }
    }

    fn find_item(&self, x: i32, y: i32) -> Option<&u8> {
        if x >= 0 && y >= 0 && x <= self.max_x && y <= self.max_y {
            return Some(&self.rows[x as usize][y as usize]);
        }
        None
    }
}

fn parse_data(data: &str) -> Grid {
    let result: Vec<Vec<u8>> = data
        .lines()
        .map(|line| {
            let row: Vec<u8> = line
                .chars()
                .map(|c| {
                    let val: u8 = c.to_string().parse().unwrap();
                    val
                })
                .collect();
            row
        })
        .collect();

    Grid::new(result)
}

#[cfg(test)]
mod tests {
    use input::get_puzzle_input;

    use super::*;

    #[test]
    fn test_part1() {
        let input = get_puzzle_input("10-sample");
        let result = solve_puzzle(input.as_str());
        assert_eq!(result, 36);
    }

    #[test]
    fn test_part2() {
        let input = get_puzzle_input("10-sample");
        let result = solve_puzzle_trails(input.as_str());
        assert_eq!(result, 81);
    }
}
