pub fn part1(input: &str) -> i32 {
    solve_puzzle(input)
}

pub fn part2(input: &str) -> i32 {
    solve_puzzle(input)
}

fn solve_puzzle(data: &str) -> i32 {
    let blocks = parse_data(data);
    print_blocks(&blocks);
    let entries = format_blocks(&blocks);
    print_disk_entries(&entries);
    0
}

#[derive(Debug)]
enum Block {
    File(FileBlock),
    Space(SpaceBlock),
}

#[derive(Debug)]
struct FileBlock {
    id: i32,
    blocks: u8,
}

#[derive(Debug)]
struct SpaceBlock {
    blocks: u8,
}

#[derive(Debug)]
enum DiskEntry {
    File(FileEntry),
    Space,
}

#[derive(Debug)]
struct FileEntry {
    id: i32,
}

fn parse_data(data: &str) -> Vec<Block> {
    let blocks: Vec<Block> = data
        .trim()
        .chars()
        .enumerate()
        .map(|(k, v)| {
            let n: u8 = v.to_string().parse().unwrap();
            if k % 2 == 0 {
                let id: i32 = k as i32 / 2;
                Block::File(FileBlock { id, blocks: n })
            } else {
                Block::Space(SpaceBlock { blocks: n })
            }
        })
        .collect();
    blocks
}

fn print_blocks(blocks: &Vec<Block>) {
    println!("** BEGIN Blocks **");
    for block in blocks.iter() {
        match block {
            Block::File(f) => {
                print!("{}", f.blocks);
            }
            Block::Space(s) => {
                print!("{}", s.blocks);
            }
        }
    }
    println!("");
    println!("** END Blocks **");
}

fn format_blocks(blocks: &Vec<Block>) -> Vec<DiskEntry> {
    let mut entries: Vec<DiskEntry> = Vec::new();
    for block in blocks.iter() {
        match block {
            Block::File(f) => {
                for _ in 0..f.blocks {
                    entries.push(DiskEntry::File(FileEntry { id: f.id }));
                }
            }
            Block::Space(s) => {
                for _ in 0..s.blocks {
                    entries.push(DiskEntry::Space);
                }
            }
        }
    }
    entries
}

fn print_disk_entries(entries: &Vec<DiskEntry>) {
    println!("** BEGIN Disk Entries **");
    for entry in entries.iter() {
        match entry {
            DiskEntry::File(f) => {
                print!("{}", f.id);
            }
            DiskEntry::Space => {
                print!(".");
            }
        }
    }
    println!("");
    println!("** END Disk Entries **");
}

#[cfg(test)]
mod tests {
    use input::get_puzzle_input;

    use super::*;

    #[test]
    fn test_part1() {
        let input = get_puzzle_input("09-sample");
        let result = solve_puzzle(input.as_str());
        assert_eq!(result, 1928);
    }

    #[test]
    fn test_part2() {
        let input = get_puzzle_input("09-sample");
        let result = solve_puzzle(input.as_str());
        assert_eq!(result, 1928);
    }
}
