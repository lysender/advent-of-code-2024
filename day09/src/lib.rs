pub fn part1(input: &str) -> i32 {
    solve_puzzle(input)
}

pub fn part2(input: &str) -> i32 {
    solve_puzzle(input)
}

fn solve_puzzle(data: &str) -> i32 {
    let blocks = parse_data(data);
    print_blocks(&blocks);
    let mut entries = format_blocks(&blocks);
    print_disk_entries(&entries);
    defrag_entries(&mut entries);
    print_disk_entries(&entries);
    0
}

#[derive(Debug, Clone)]
enum Block {
    File(FileBlock),
    Space(SpaceBlock),
}

#[derive(Debug, Clone)]
struct FileBlock {
    id: i32,
    blocks: u8,
}

#[derive(Debug, Clone)]
struct SpaceBlock {
    blocks: u8,
}

#[derive(Debug, Clone, Copy)]
enum DiskEntry {
    File(FileEntry),
    Space,
}

#[derive(Debug, Clone, Copy)]
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
    println!("== BEGIN Blocks ==");
    let str = format_blocks_str(blocks);
    println!("{}", str);
    println!("== END Blocks ==");
}

fn format_blocks_str(blocks: &Vec<Block>) -> String {
    let mut parts: Vec<String> = Vec::new();
    for block in blocks.iter() {
        parts.push(match block {
            Block::File(f) => {
                format!("{}", f.blocks)
            }
            Block::Space(s) => {
                format!("{}", s.blocks)
            }
        });
    }
    parts.join("").to_string()
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
    println!("== BEGIN Disk Entries ==");
    let str = format_disk_entries_str(entries);
    println!("{}", str);
    println!("== END Disk Entries ==");
}

fn format_disk_entries_str(entries: &Vec<DiskEntry>) -> String {
    let mut parts: Vec<String> = Vec::new();
    for entry in entries.iter() {
        parts.push(match entry {
            DiskEntry::File(f) => f.id.to_string(),
            DiskEntry::Space => ".".to_string(),
        });
    }
    parts.join("").to_string()
}

fn defrag_entries(entries: &mut Vec<DiskEntry>) {
    // Move the rightmost file block to the left most space block
    // One at a time
    // Until no more gaps left to be filled
    let mut l: usize = 0;
    let mut r: usize = entries.len() - 1;

    while l < r {
        let file = scan_file(entries, r, l);
        let space = scan_space(entries, l, r);

        match (space, file) {
            (Some(s), Some(f)) => {
                // Swap places
                let tmp = entries[f];
                entries[f] = entries[s];
                entries[s] = tmp;

                // Move the indexes
                l = s + 1;
                r = f - 1;
            }
            _ => {
                break;
            }
        }
    }
}

fn scan_file(entries: &Vec<DiskEntry>, start: usize, left: usize) -> Option<usize> {
    // Decrement start until left to find a file block starting from the right
    let mut i: usize = start;
    while left < i {
        let item = &entries[i];
        match item {
            DiskEntry::File(_) => {
                return Some(i);
            }
            _ => {
                i -= 1;
            }
        };
    }
    None
}

fn scan_space(entries: &Vec<DiskEntry>, start: usize, right: usize) -> Option<usize> {
    // Increment start until right to find a space block starting from the left
    let mut i: usize = start;
    while i < right {
        let item = &entries[i];
        match item {
            DiskEntry::Space => {
                return Some(i);
            }
            _ => {
                i += 1;
            }
        };
    }
    None
}

#[cfg(test)]
mod tests {
    use input::get_puzzle_input;

    use super::*;

    #[test]
    fn test_blocks_str() {
        let data = get_puzzle_input("09-sample");
        let blocks = parse_data(data.as_str());
        let blocks_str = format_blocks_str(&blocks);
        assert_eq!(blocks_str, "2333133121414131402".to_string(),);

        let mut entries = format_blocks(&blocks);
        let entries_str = format_disk_entries_str(&entries);
        assert_eq!(
            entries_str,
            "00...111...2...333.44.5555.6666.777.888899".to_string()
        );

        defrag_entries(&mut entries);
        let entries_str = format_disk_entries_str(&entries);
        assert_eq!(
            entries_str,
            "0099811188827773336446555566..............".to_string()
        );
    }

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
