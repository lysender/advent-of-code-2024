const MUL: &'static str = "mul(";
const DO: &'static str = "do()";
const DONT: &'static str = "don't()";

#[derive(Debug)]
struct Pair {
    x: i32,
    y: i32,
}

impl Pair {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn mul(&self) -> i32 {
        self.x * self.y
    }
}

#[derive(Debug)]
enum MarkerKind {
    Mul,
    Do,
    Dont,
}

#[derive(Debug)]
struct Marker {
    kind: MarkerKind,
    start: usize,
    end: usize,
}

pub fn part1(input: &str) -> i32 {
    solve_expressions(input, true)
}

pub fn part2(input: &str) -> i32 {
    solve_expressions(input, false)
}

fn solve_expressions(input: &str, always_on: bool) -> i32 {
    parse_expressions(input, always_on)
}

fn parse_expressions(input: &str, always_on: bool) -> i32 {
    let mut total: i32 = 0;

    let max_len = input.len();
    let mut buffer = &input[..];
    let mut enabled = true;

    while let Some(mark) = find_marker(buffer, always_on) {
        let start = mark.end;
        if start >= max_len {
            break;
        }

        // Slice buffer to start on the end of the marker
        buffer = &buffer[start..];

        match mark.kind {
            MarkerKind::Do => {
                enabled = true;
            }
            MarkerKind::Dont => {
                enabled = false;
            }
            MarkerKind::Mul => {
                if enabled {
                    if let Some(pair) = find_expression(buffer) {
                        total += pair.mul();
                    }
                }
            }
        }
    }

    total
}

fn find_marker(buffer: &str, always_on: bool) -> Option<Marker> {
    let mul = buffer.find(MUL);
    let doit = buffer.find(DO);
    let dont = buffer.find(DONT);

    // Return the closest marker position
    let mut result: Option<Marker> = None;
    if let Some(pos) = mul {
        result = Some(Marker {
            kind: MarkerKind::Mul,
            start: pos,
            end: pos + MUL.len(),
        });

        if always_on {
            return result;
        }
    }

    if let Some(pos) = doit {
        if let Some(cur) = result.as_ref() {
            if pos < cur.start {
                result = Some(Marker {
                    kind: MarkerKind::Do,
                    start: pos,
                    end: pos + DO.len(),
                });
            }
        } else {
            result = Some(Marker {
                kind: MarkerKind::Do,
                start: pos,
                end: pos + DO.len(),
            });
        }
    }

    if let Some(pos) = dont {
        if let Some(cur) = result.as_ref() {
            if pos < cur.start {
                result = Some(Marker {
                    kind: MarkerKind::Dont,
                    start: pos,
                    end: pos + DONT.len(),
                });
            }
        }
    }

    result
}

fn find_expression(buffer: &str) -> Option<Pair> {
    // Find the first number until a ",", invalid char or eol
    let mut first_num: Vec<char> = Vec::new();
    let mut next_start: Option<usize> = None;

    for (k, v) in buffer.chars().enumerate() {
        if v == ',' {
            // End of numbers?
            if first_num.len() == 0 {
                // Not good, bail out
                return None;
            }
            if (k + 1) < buffer.len() {
                next_start = Some(k + 1);
            }
            break;
        } else if v.is_numeric() {
            // Add more digits
            first_num.push(v);
        } else {
            // Unexpected combination, bail out
            return None;
        }
    }

    let first_str: String = first_num.iter().cloned().collect();
    let x: i32 = first_str.parse().unwrap();

    // Find the next number until a ")", invalid char or eol
    if let Some(next) = next_start {
        let next_buffer = &buffer[next..];
        let mut second_num: Vec<char> = Vec::new();
        for v in next_buffer.chars() {
            if v == ')' {
                // End of second num?
                if second_num.len() == 0 {
                    // Not good, bail out
                    return None;
                }
                // We're good to go, end the loop
                break;
            } else if v.is_numeric() {
                second_num.push(v);
            } else {
                // Unexpected combination, bail out
                return None;
            }
        }

        let second_str: String = second_num.iter().clone().collect();
        let y: i32 = second_str.parse().unwrap();

        return Some(Pair::new(x, y));
    }

    None
}

#[cfg(test)]
mod tests {
    use input::get_puzzle_input;

    use super::*;

    #[test]
    fn test_part1() {
        let input = get_puzzle_input("03-sample1");
        let total = solve_expressions(input.as_str(), true);
        assert_eq!(total, 161);
    }

    #[test]
    fn test_part2() {
        let input = get_puzzle_input("03-sample2");
        let total = solve_expressions(input.as_str(), false);
        assert_eq!(total, 48);
    }
}
