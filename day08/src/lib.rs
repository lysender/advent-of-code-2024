use std::collections::{HashMap, HashSet};

use glam::IVec2;

pub fn part1(input: &str) -> i32 {
    solve_puzzle(input)
}

pub fn part2(input: &str) -> i32 {
    solve_puzzle_harmonics(input)
}

#[derive(Debug)]
struct Coverage {
    matrix: Vec<Vec<char>>,
    max_x: i32,
    max_y: i32,
    antinodes: HashMap<IVec2, char>,
}

#[derive(Debug)]
struct Tower {
    value: char,
    pos: IVec2,
}

impl Coverage {
    fn new(matrix: Vec<Vec<char>>) -> Self {
        let max_x = (matrix.len() - 1) as i32;
        assert!(max_x > 0, "Matrix max_x must be > 0");
        let max_y = (matrix[0].len() - 1) as i32;
        assert!(max_y > 0, "Matrix max_y must be > 0");

        Self {
            matrix,
            max_x,
            max_y,
            antinodes: HashMap::new(),
        }
    }

    fn find_towers(&self) -> Vec<Tower> {
        let mut towers: Vec<Tower> = Vec::new();
        for (x, xv) in self.matrix.iter().enumerate() {
            for (y, yv) in xv.iter().enumerate() {
                if yv.is_alphanumeric() {
                    let pos = IVec2::new(x as i32, y as i32);
                    towers.push(Tower { value: *yv, pos });
                }
            }
        }

        towers
    }

    fn find_item(&self, x: i32, y: i32) -> Option<&char> {
        if x >= 0 && y >= 0 && x <= self.max_x && y <= self.max_y {
            return Some(&self.matrix[x as usize][y as usize]);
        }
        None
    }

    fn scan_section(&self, tower: &Tower) -> HashSet<(IVec2, IVec2)> {
        let mut pairs: HashSet<(IVec2, IVec2)> = HashSet::new();

        // From starting position, find the adjacent tower
        // Scan everything from current level and below starting from center
        for x in 0..=self.max_x {
            for y in 0..=self.max_y {
                // Forward x, forward y
                let px = tower.pos.x + x;
                let py = tower.pos.y + y;
                // Exclude self
                if px == tower.pos.x && py == tower.pos.y {
                    continue;
                }

                let mut right_none = false;
                if let Some(c) = self.find_item(px, py) {
                    if c == &tower.value {
                        let fpos = IVec2::new(px, py);
                        if is_coord_greater(&fpos, &tower.pos) {
                            pairs.insert((tower.pos.clone(), fpos));
                        } else {
                            pairs.insert((fpos, tower.pos.clone()));
                        }
                    }
                } else {
                    right_none = true;
                }

                // Forward x, backward y
                let px = tower.pos.x + x;
                let py = tower.pos.y - y;
                let mut left_none = false;
                if let Some(c) = self.find_item(px, py) {
                    if c == &tower.value {
                        let bpos = IVec2::new(px, py);
                        if is_coord_greater(&bpos, &tower.pos) {
                            pairs.insert((tower.pos.clone(), bpos));
                        } else {
                            pairs.insert((bpos, tower.pos.clone()));
                        }
                    }
                } else {
                    left_none = true;
                }

                if right_none && left_none {
                    break;
                }
            }
        }

        // Scan everything from current level -1 and above starting from center
        for x in 0..=self.max_x {
            for y in 0..=self.max_y {
                // Backward x, forward y
                let px = tower.pos.x - x;
                let py = tower.pos.y + y;
                // Exclude self
                if px == tower.pos.x && py == tower.pos.y {
                    continue;
                }

                let mut right_none = false;
                if let Some(c) = self.find_item(px, py) {
                    if c == &tower.value {
                        let fpos = IVec2::new(px, py);
                        if is_coord_greater(&fpos, &tower.pos) {
                            pairs.insert((tower.pos.clone(), fpos));
                        } else {
                            pairs.insert((fpos, tower.pos.clone()));
                        }
                    }
                } else {
                    right_none = true;
                }

                // Backward x, backward y
                let px = tower.pos.x - x;
                let py = tower.pos.y - y;
                let mut left_none = false;
                if let Some(c) = self.find_item(px, py) {
                    if c == &tower.value {
                        let bpos = IVec2::new(px, py);
                        if is_coord_greater(&bpos, &tower.pos) {
                            pairs.insert((tower.pos.clone(), bpos));
                        } else {
                            pairs.insert((bpos, tower.pos.clone()));
                        }
                    }
                } else {
                    left_none = true;
                }

                if right_none && left_none {
                    break;
                }
            }
        }

        pairs
    }

    fn plot_antinodes(&mut self, tower: &Tower, pairs: &HashSet<(IVec2, IVec2)>) {
        // For each pair, register its antinodes
        for pair in pairs.iter() {
            let prev = get_prev_coord(&pair.0, &pair.1);
            if let Some(_) = self.find_item(prev.x, prev.y) {
                self.antinodes.insert(prev, tower.value.clone());
            }
            let next = get_next_coord(&pair.0, &pair.1);
            if let Some(_) = self.find_item(next.x, next.y) {
                self.antinodes.insert(next, tower.value.clone());
            }
        }
    }

    fn plot_harmonics_antinodes(&mut self, tower: &Tower, pairs: &HashSet<(IVec2, IVec2)>) {
        // For each pair, register its antinodes
        for pair in pairs.iter() {
            // The pair itself becomes antinodes
            self.antinodes.insert(pair.0.clone(), tower.value);
            self.antinodes.insert(pair.1.clone(), tower.value);

            // Backward until the edge
            let mut a: IVec2 = pair.0;
            let mut b: IVec2 = pair.1;

            loop {
                let prev = get_prev_coord(&a, &b);
                if let Some(_) = self.find_item(prev.x, prev.y) {
                    self.antinodes.insert(prev, tower.value);
                    // Prev becomes the new a
                    b = a;
                    a = prev;
                } else {
                    break;
                }
            }

            // Forward until the edge
            let mut a: IVec2 = pair.0;
            let mut b: IVec2 = pair.1;
            loop {
                let next = get_next_coord(&a, &b);
                if let Some(_) = self.find_item(next.x, next.y) {
                    self.antinodes.insert(next, tower.value);
                    // Next becomes the new b
                    a = b;
                    b = next;
                } else {
                    break;
                }
            }
        }
    }
}

fn solve_puzzle(input: &str) -> i32 {
    let mut coverage = parse_data(input);
    let towers = coverage.find_towers();

    for tower in towers.iter() {
        let pairs = coverage.scan_section(tower);
        coverage.plot_antinodes(tower, &pairs);
    }
    coverage.antinodes.len() as i32
}

fn solve_puzzle_harmonics(input: &str) -> i32 {
    let mut coverage = parse_data(input);
    let towers = coverage.find_towers();

    for tower in towers.iter() {
        let pairs = coverage.scan_section(tower);
        coverage.plot_harmonics_antinodes(tower, &pairs);
    }
    coverage.antinodes.len() as i32
}

fn parse_data(data: &str) -> Coverage {
    let matrix: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();
    Coverage::new(matrix)
}

fn is_coord_greater(v1: &IVec2, v2: &IVec2) -> bool {
    if v1.x > v2.x {
        // X-axis is high priority
        return true;
    }
    if v1.x == v2.x {
        return v1.y > v2.y;
    }
    false
}

fn get_next_coord(a: &IVec2, b: &IVec2) -> IVec2 {
    // Find the next coord from the pair coord moving forward
    let diff_x = b.x - a.x;
    let diff_y = b.y - a.y;
    IVec2::new(b.x + diff_x, b.y + diff_y)
}

fn get_prev_coord(a: &IVec2, b: &IVec2) -> IVec2 {
    // Find the prev coord from the pair coord moving backward
    let diff_x = b.x - a.x;
    let diff_y = b.y - a.y;
    IVec2::new(a.x - diff_x, a.y - diff_y)
}

#[cfg(test)]
mod tests {
    use input::get_puzzle_input;

    use super::*;

    #[test]
    fn test_is_greater_ivec2() {
        assert!(is_coord_greater(&IVec2::new(5, 5), &IVec2::new(5, 4)));
        assert!(is_coord_greater(&IVec2::new(5, 5), &IVec2::new(4, 5)));
        assert!(is_coord_greater(&IVec2::new(5, 5), &IVec2::new(4, 4)));
        assert!(is_coord_greater(&IVec2::new(5, 5), &IVec2::new(5, 0)));
        assert!(!is_coord_greater(&IVec2::new(5, 5), &IVec2::new(6, 6)));
    }

    #[test]
    fn test_get_next_coord() {
        assert_eq!(
            get_next_coord(&IVec2::new(2, 5), &IVec2::new(4, 4)),
            IVec2::new(6, 3)
        );
        assert_eq!(
            get_next_coord(&IVec2::new(5, 6), &IVec2::new(8, 8)),
            IVec2::new(11, 10)
        );
        assert_eq!(
            get_next_coord(&IVec2::new(8, 8), &IVec2::new(9, 9)),
            IVec2::new(10, 10)
        );
    }

    #[test]
    fn test_get_prev_coord() {
        assert_eq!(
            get_prev_coord(&IVec2::new(2, 5), &IVec2::new(3, 7)),
            IVec2::new(1, 3)
        );
        assert_eq!(
            get_prev_coord(&IVec2::new(2, 5), &IVec2::new(4, 4)),
            IVec2::new(0, 6)
        );
        assert_eq!(
            get_prev_coord(&IVec2::new(8, 8), &IVec2::new(9, 9)),
            IVec2::new(7, 7)
        );
    }

    #[test]
    fn test_part1() {
        let input = get_puzzle_input("08-sample");
        let result = solve_puzzle(input.as_str());
        assert_eq!(result, 14);
    }

    #[test]
    fn test_part2() {
        let input = get_puzzle_input("08-sample");
        let result = solve_puzzle_harmonics(input.as_str());
        assert_eq!(result, 34);
    }
}
