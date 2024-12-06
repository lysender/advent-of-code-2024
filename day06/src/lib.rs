use std::collections::HashSet;

use glam::IVec2;

const CH_OBS: u8 = b'#';
const CH_EMP: u8 = b'.';

const CH_GU: u8 = b'^';
const CH_GR: u8 = b'>';
const CH_GD: u8 = b'v';
const CH_GL: u8 = b'<';

#[derive(Debug, Clone, PartialEq)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone)]
struct Guard {
    pos: IVec2,
    dir: Dir,
}

impl Guard {
    fn new(pos: IVec2, dir: Dir) -> Self {
        Self { pos, dir }
    }

    /// Rotate self's orientation
    fn rotate(&mut self) {
        self.dir = match self.dir {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        };
    }

    /// Compute the next position based on orientation
    fn forward(&self) -> IVec2 {
        match self.dir {
            Dir::Up => self.pos + IVec2::new(-1, 0),
            Dir::Right => self.pos + IVec2::new(0, 1),
            Dir::Down => self.pos + IVec2::new(1, 0),
            Dir::Left => self.pos + IVec2::new(0, -1),
        }
    }

    /// Set the current position
    fn set_pos(&mut self, pos: IVec2) {
        self.pos = pos;
    }
}

#[derive(Debug, Clone, PartialEq)]
enum CellItem {
    Empty,
    Obs,
    Guard,
}

#[derive(Debug, Clone)]
struct Grid {
    matrix: Vec<Vec<CellItem>>,
    max_x: i32,
    max_y: i32,
    guard: Guard,
}

impl Grid {
    fn new(matrix: Vec<Vec<CellItem>>, guard: Guard) -> Self {
        let max_x = (matrix.len() - 1) as i32;
        assert!(max_x > 0, "There must be at least 1 row");
        let max_y = (matrix[0].len() - 1) as i32;

        Self {
            matrix,
            max_x,
            max_y,
            guard,
        }
    }

    fn is_edge(&self) -> bool {
        match self.guard.dir {
            Dir::Up => self.guard.pos.y == 0,
            Dir::Right => self.guard.pos.x == self.max_x,
            Dir::Down => self.guard.pos.y == self.max_y,
            Dir::Left => self.guard.pos.x == 0,
        }
    }

    fn jump_next(&mut self) -> Option<CellItem> {
        if self.is_edge() {
            return None;
        }

        let prev_pos = self.guard.pos;
        let pos = self.guard.forward();

        // Make sure the new position is now out of bounds
        if pos.x < 0 || pos.y < 0 || pos.x > self.max_x || pos.y > self.max_y {
            return None;
        }

        let item = &self.matrix[pos.x as usize][pos.y as usize];

        match *item {
            CellItem::Empty => {
                // Move the guard forward
                self.guard.set_pos(pos);
                // Set the previous location empty
                self.matrix[prev_pos.x as usize][prev_pos.y as usize] = CellItem::Empty;

                // Inform that nothing is ahead
                Some(CellItem::Empty)
            }
            CellItem::Obs => {
                // Do not move forward but return the obstruction
                Some(CellItem::Obs)
            }
            _ => panic!("Not expecting anything other than empty of obs"),
        }
    }

    /// Moves next and rotates if necessary
    fn next(&mut self) -> Option<CellItem> {
        match self.jump_next() {
            Some(c) => match c {
                CellItem::Empty => Some(CellItem::Empty),
                CellItem::Obs => {
                    self.guard.rotate();
                    self.jump_next()
                }
                _ => panic!("Not expecting anything other than empty of obs"),
            },
            None => None,
        }
    }

    fn empty_space(&self, pos: &IVec2) -> bool {
        if pos.x >= 0 && pos.y >= 0 && pos.x <= self.max_x && pos.y <= self.max_y {
            return match self.matrix[pos.x as usize][pos.y as usize] {
                CellItem::Empty => true,
                _ => false,
            };
        }
        false
    }

    fn fill_obs(&mut self, pos: &IVec2) {
        self.matrix[pos.x as usize][pos.y as usize] = CellItem::Obs;
    }
}

pub fn part1(data: &str) -> i32 {
    solve_puzzle(data)
}

pub fn part2(data: &str) -> i32 {
    solve_puzzle_loops(data)
}

fn solve_puzzle(data: &str) -> i32 {
    let mut grid = parse_data(data);

    let mut moves: HashSet<IVec2> = HashSet::new();
    moves.insert(grid.guard.pos.clone());

    while let Some(_) = grid.next() {
        moves.insert(grid.guard.pos.clone());
    }

    moves.len() as i32
}

fn solve_puzzle_loops(data: &str) -> i32 {
    // Run once to find all cells where we can insert an obstruction
    // Candicate cells are within the original path
    let orig_grid = parse_data(data);
    let mut grid = orig_grid.clone();
    let start_pos = grid.guard.pos.clone();
    let mut blockers: HashSet<IVec2> = HashSet::new();

    while let Some(_) = grid.next() {
        blockers.insert(grid.guard.pos.clone());
    }

    // Ensure to remove the starting pos
    blockers.remove(&start_pos);

    let mut result = 0;

    for v in blockers.iter() {
        let mut test_grid = orig_grid.clone();
        if test_grid.empty_space(v) {
            test_grid.fill_obs(v);
            if has_loop(test_grid) {
                result += 1;
            }
        }
    }

    result
}

fn has_loop(mut grid: Grid) -> bool {
    let mut fast_grid = grid.clone();
    let mut looping = false;

    loop {
        let g1 = grid.next();
        fast_grid.next();
        let g2 = fast_grid.next();

        match (g1, g2) {
            (None, None) => {
                break;
            }
            (None, Some(_)) => {
                break;
            }
            (Some(_), None) => {
                break;
            }
            (Some(_), Some(_)) => {
                // Compare the positions of the guards
                if grid.guard.pos.eq(&fast_grid.guard.pos) && grid.guard.dir == fast_grid.guard.dir
                {
                    looping = true;
                    break;
                }
            }
        };
    }

    looping
}

fn parse_data(data: &str) -> Grid {
    let mut matrix: Vec<Vec<CellItem>> = Vec::new();
    let mut guard: Option<Guard> = None;

    for (x, v) in data.lines().enumerate() {
        let mut row: Vec<CellItem> = Vec::new();
        for (y, c) in v.as_bytes().iter().enumerate() {
            let cv = match *c {
                CH_OBS => CellItem::Obs,
                CH_GU => {
                    assert!(guard.is_none(), "There must be no guard yet");
                    guard = Some(Guard::new(IVec2::new(x as i32, y as i32), Dir::Up));
                    CellItem::Guard
                }
                CH_GR => {
                    assert!(guard.is_none(), "There must be no guard yet");
                    guard = Some(Guard::new(IVec2::new(x as i32, y as i32), Dir::Right));
                    CellItem::Guard
                }
                CH_GD => {
                    assert!(guard.is_none(), "There must be no guard yet");
                    guard = Some(Guard::new(IVec2::new(x as i32, y as i32), Dir::Down));
                    CellItem::Guard
                }
                CH_GL => {
                    assert!(guard.is_none(), "There must be no guard yet");
                    guard = Some(Guard::new(IVec2::new(x as i32, y as i32), Dir::Left));
                    CellItem::Guard
                }
                CH_EMP => CellItem::Empty,
                _ => panic!("Invalid cell value."),
            };
            row.push(cv);
        }
        matrix.push(row);
    }

    Grid::new(matrix, guard.expect("Guard must be present"))
}

#[cfg(test)]
mod tests {
    use input::get_puzzle_input;

    use super::*;

    #[test]
    fn test_rotate() {
        let pos = IVec2::new(5, 5);
        let mut guard = Guard::new(pos, Dir::Up);
        guard.rotate();

        let result = match guard.dir {
            Dir::Right => true,
            _ => false,
        };
        assert!(result);
    }

    #[test]
    fn test_forward() {
        let pos = IVec2::new(5, 5);
        let guard = Guard::new(pos, Dir::Up);
        let next_pos = guard.forward();

        assert_eq!(next_pos, IVec2::new(4, 5));
    }

    #[test]
    fn test_edge() {
        let pos = IVec2::new(1, 0);
        let guard = Guard::new(pos, Dir::Right);
        let matrix = vec![
            vec![CellItem::Empty, CellItem::Guard],
            vec![CellItem::Empty, CellItem::Empty],
        ];
        let grid = Grid::new(matrix, guard);
        assert!(grid.is_edge());
    }

    #[test]
    fn test_loop1() {
        let data = get_puzzle_input("06-sample-loop1");
        let grid = parse_data(data.as_str());
        assert!(has_loop(grid));
    }

    #[test]
    fn test_loop2() {
        let data = get_puzzle_input("06-sample-loop2");
        let grid = parse_data(data.as_str());
        assert!(has_loop(grid));
    }

    #[test]
    fn test_loop3() {
        let data = get_puzzle_input("06-sample-loop3");
        let grid = parse_data(data.as_str());
        assert!(has_loop(grid));
    }

    #[test]
    fn test_no_loop() {
        let data = get_puzzle_input("06-sample");
        let grid = parse_data(data.as_str());
        assert!(!has_loop(grid));
    }

    #[test]
    fn test_part1() {
        let input = get_puzzle_input("06-sample");
        let result = solve_puzzle(input.as_str());
        assert_eq!(result, 41);
    }

    #[test]
    fn test_part2() {
        let input = get_puzzle_input("06-sample");
        let result = solve_puzzle_loops(input.as_str());
        assert_eq!(result, 6);
    }
}
