const CX: u8 = b'X';
const CM: u8 = b'M';
const CA: u8 = b'A';
const CS: u8 = b'S';

const MOVERS: [[i32; 2]; 8] = [
    [1, 0],
    [1, 1],
    [0, 1],
    [-1, 1],
    [-1, 0],
    [-1, -1],
    [0, -1],
    [1, -1],
];

pub fn part1(input: &str) -> i32 {
    solve_puzzle(input)
}

pub fn part2(input: &str) -> i32 {
    solve_x_puzzle(input)
}

fn solve_puzzle(input: &str) -> i32 {
    let table = parse_matrix(input);
    let row_len = table.len();
    if row_len == 0 {
        return 0;
    }
    let col_len = table[0].len();
    if col_len == 0 {
        return 0;
    }

    let mut total: i32 = 0;

    for x in 0..row_len {
        for y in 0..col_len {
            let patterns =
                find_patterns(&table, x as i32, y as i32, row_len as i32, col_len as i32);
            total += patterns;
        }
    }
    total
}

fn solve_x_puzzle(input: &str) -> i32 {
    let table = parse_matrix(input);
    let row_len = table.len();
    if row_len == 0 {
        return 0;
    }
    let col_len = table[0].len();
    if col_len == 0 {
        return 0;
    }

    let mut total: i32 = 0;

    for x in 0..row_len {
        for y in 0..col_len {
            let patterns =
                find_x_patterns(&table, x as i32, y as i32, row_len as i32, col_len as i32);
            total += patterns;
        }
    }
    total
}

fn parse_matrix(input: &str) -> Vec<Vec<u8>> {
    let mut row_len: Option<usize> = None;
    let mut rows: Vec<Vec<u8>> = Vec::new();

    for line in input.lines() {
        let row: Vec<u8> = line.as_bytes().to_vec();
        // Ensure it is a proper matrix
        if let Some(length) = row_len {
            assert_eq!(length, row.len());
        } else {
            row_len = Some(row.len());
        }

        rows.push(row);
    }
    rows
}

fn find_patterns(matrix: &Vec<Vec<u8>>, x: i32, y: i32, max_x: i32, max_y: i32) -> i32 {
    // Find east
    // Find south east
    // Find south
    // Find sout west
    // Find west
    // Find north west
    // Find north
    // Find north east
    let mut result: i32 = 0;

    for pos in MOVERS.iter() {
        let x1 = pos[0];
        let x2 = x1 + x1;
        let x3 = x2 + x1;

        let y1 = pos[1];
        let y2 = y1 + y1;
        let y3 = y2 + y1;

        // Bounds check
        if (x + x3) >= max_x || (x + x3) < 0 {
            continue;
        }
        if (y + y3) >= max_y || (y + y3) < 0 {
            continue;
        }

        if matrix[x as usize][y as usize] == CX {
            let xx1 = (x + x1) as usize;
            let yy1 = (y + y1) as usize;

            if matrix[xx1][yy1] == CM {
                let xx2 = (x + x2) as usize;
                let yy2 = (y + y2) as usize;

                if matrix[xx2][yy2] == CA {
                    let xx3 = (x + x3) as usize;
                    let yy3 = (y + y3) as usize;

                    if matrix[xx3][yy3] == CS {
                        result += 1;
                    }
                }
            }
        }
    }
    result
}

fn find_x_patterns(matrix: &Vec<Vec<u8>>, x: i32, y: i32, max_x: i32, max_y: i32) -> i32 {
    // Find pattern that looks like X of MAS, including reverse like SAM
    // A must be always at the center to make this work
    // M - S
    // - A -
    // M - S
    let nw_x = x - 1;
    let nw_y = y - 1;
    let ne_x = x + 1;
    let ne_y = y - 1;
    let sw_x = x - 1;
    let sw_y = y + 1;
    let se_x = x + 1;
    let se_y = y + 1;

    // Check bounds
    let mut valid_bounds = false;
    if nw_x >= 0 && nw_x < max_x && nw_y >= 0 && nw_y < max_y {
        if ne_x >= 0 && ne_x < max_x && ne_y >= 0 && ne_y < max_y {
            if sw_x >= 0 && sw_x < max_x && sw_y >= 0 && sw_y < max_y {
                if se_x >= 0 && se_x < max_x && se_y >= 0 && se_y < max_y {
                    valid_bounds = true;
                }
            }
        }
    }

    if !valid_bounds {
        return 0;
    }

    // Center must be A
    if matrix[x as usize][y as usize] == CA {
        // Diagonal NW to SE must be M-S or S-M
        let mut valid_d1 = false;
        if (matrix[nw_x as usize][nw_y as usize] == CM
            && matrix[se_x as usize][se_y as usize] == CS)
            || (matrix[nw_x as usize][nw_y as usize] == CS
                && matrix[se_x as usize][se_y as usize] == CM)
        {
            valid_d1 = true;
        }
        // Diagonal SW to NE must be M-S or S-M
        let mut valid_d2 = false;
        if (matrix[sw_x as usize][sw_y as usize] == CM
            && matrix[ne_x as usize][ne_y as usize] == CS)
            || (matrix[sw_x as usize][sw_y as usize] == CS
                && matrix[ne_x as usize][ne_y as usize] == CM)
        {
            valid_d2 = true;
        }

        if valid_d1 && valid_d2 {
            return 1;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use crate::get_puzzle_input;

    use super::*;

    #[test]
    fn test_part1() {
        let input = get_puzzle_input("04-sample");
        let result = solve_puzzle(input.as_str());
        assert_eq!(result, 18);
    }

    #[test]
    fn test_part2() {
        let input = get_puzzle_input("04-sample");
        let result = solve_x_puzzle(input.as_str());
        assert_eq!(result, 9);
    }
}
