pub fn part1(input: &str) -> i64 {
    solve_puzzle(input)
}

pub fn part2(input: &str) -> i64 {
    solve_puzzle_contiguous(input)
}

fn solve_puzzle(data: &str) -> i64 {
    let blocks = parse_data(data);
    let mut entries = format_blocks(&blocks);
    defrag_entries(&mut entries);

    let total: i64 = entries
        .iter()
        .enumerate()
        .map(|(k, b)| match b {
            DiskEntry::File(f) => f.id as i64 * (k as i64),
            DiskEntry::Space => 0,
        })
        .sum();
    total
}

fn solve_puzzle_contiguous(data: &str) -> i64 {
    let blocks = parse_data(data);
    let mut entries = format_blocks(&blocks);
    defrag_entries_contiguous(&mut entries);

    let total: i64 = entries
        .iter()
        .enumerate()
        .map(|(k, b)| match b {
            DiskEntry::File(f) => f.id as i64 * (k as i64),
            DiskEntry::Space => 0,
        })
        .sum();
    total
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

#[derive(Debug, Clone, Copy)]
struct FileEntryIndex {
    index: usize,
    length: usize,
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
                swap_entries(entries, s, f);

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

fn defrag_entries_contiguous(entries: &mut Vec<DiskEntry>) {
    let indexes = index_file_entries(&entries);

    let mut space_start: usize = 0;
    for file in indexes.iter().rev() {
        // Find a space block that fits this file block
        let s_index = find_free_space(entries, space_start, file.index, file.length);
        if let Some(s_index) = s_index {
            // Swap values
            for i in 0..file.length {
                swap_entries(entries, s_index + i, file.index + i);
            }
        }

        let space_one = find_one_space(entries, space_start, file.index);
        if let Some(s_1) = space_one {
            if space_start < s_1 {
                space_start = s_1;
            }
        }
    }
}

fn swap_entries(entries: &mut Vec<DiskEntry>, a: usize, b: usize) {
    let tmp = entries[b];
    entries[b] = entries[a];
    entries[a] = tmp;
}

fn find_one_space(entries: &Vec<DiskEntry>, start: usize, end: usize) -> Option<usize> {
    for i in start..=end {
        let item = entries[i];
        match item {
            DiskEntry::Space => {
                return Some(i);
            }
            DiskEntry::File(_) => (),
        }
    }
    None
}

fn find_free_space(
    entries: &Vec<DiskEntry>,
    start: usize,
    end: usize,
    length: usize,
) -> Option<usize> {
    let mut s_index: Option<usize> = None;
    let mut s_length: usize = 0;

    for i in start..=end {
        let item = entries[i];
        match item {
            DiskEntry::Space => {
                if s_index.is_some() {
                    // Continue the count
                    s_length += 1;

                    if s_length >= length {
                        // We got a free space to fit in
                        return Some(s_index.unwrap());
                    }
                } else {
                    // Start a new count
                    s_index = Some(i);
                    s_length = 1;

                    if s_length >= length {
                        // We got a free space to fit in
                        return Some(i);
                    }
                }
            }
            DiskEntry::File(_) => {
                if s_index.is_some() {
                    // Reset
                    s_index = None;
                    s_length = 0;
                }
            }
        }
    }

    None
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

fn index_file_entries(entries: &Vec<DiskEntry>) -> Vec<FileEntryIndex> {
    // Index all files and space blocks so that we can locate them easily via a loop
    let mut indexes: Vec<FileEntryIndex> = Vec::new();
    let mut prev: Option<FileEntryIndex> = None;
    for (k, v) in entries.iter().enumerate() {
        match v {
            DiskEntry::File(f) => {
                if let Some(prev_index) = prev.as_mut() {
                    if prev_index.id == f.id {
                        // Same as previous, just increment the length
                        prev_index.length += 1;
                        prev = Some(*prev_index);
                    } else {
                        // Encountered a new file block
                        // Complete the previous index and start a new index
                        indexes.push(*prev_index);
                        prev = Some(FileEntryIndex {
                            index: k,
                            length: 1,
                            id: f.id,
                        });
                    }
                } else {
                    // Start of the entries, start a new index
                    prev = Some(FileEntryIndex {
                        index: k,
                        length: 1,
                        id: f.id,
                    });
                }
            }
            DiskEntry::Space => {
                if let Some(prev_index) = prev.as_mut() {
                    // Suddenly encountered a space, mark the previous
                    // file as complete and record a new prev as a space
                    indexes.push(*prev_index);
                    // Start over to collect new file blocks
                    prev = None;
                }
            }
        }
    }

    if let Some(prev_index) = prev.take() {
        indexes.push(prev_index);
    }

    indexes
}

#[cfg(test)]
mod tests {
    use input::get_puzzle_input;

    use super::*;

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
    fn test_blocks_str_contiguous() {
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

        defrag_entries_contiguous(&mut entries);

        let entries_str = format_disk_entries_str(&entries);
        assert_eq!(
            entries_str,
            "00992111777.44.333....5555.6666.....8888..".to_string()
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
        let result = solve_puzzle_contiguous(input.as_str());
        assert_eq!(result, 2858);
    }
}
