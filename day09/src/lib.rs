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
    let entries = defrag_entries_contiguous(format_blocks(&blocks));

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
struct DiskEntryIndex {
    orig_index: usize,
    index: usize,
    length: usize,
    item: DiskEntry,
    moved: bool,
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

fn defrag_entries_contiguous(entries: Vec<DiskEntry>) -> Vec<DiskEntry> {
    let mut indexes = index_disk_entries(&entries);

    for i in indexes.iter() {
        println!("{:?}", i);
    }

    // Find file blocks one at a time starting from the end
    let mut i = indexes.len() - 1;
    while i > 0 {
        let curr_index = &indexes[i];
        if curr_index.moved {
            i -= 1;
            continue;
        }

        match curr_index.item {
            DiskEntry::File(_) => {
                // Find a block of spaces that may fit
                if let Some(sk) = find_free_space(&indexes, curr_index.length) {
                    let s_index = &indexes[sk];

                    if s_index.orig_index < curr_index.orig_index {
                        println!("curr: {:?}, space: {:?}", curr_index, s_index);

                        if s_index.length == curr_index.length {
                            // We can simply swap the values
                            let tmp = indexes[i];
                            indexes[i] = indexes[sk];
                            indexes[sk] = tmp;
                        } else {
                            // Break the space down to two parts, first part with length equal to the
                            // file and another part with the remaining spaces
                            let diff = curr_index.length.abs_diff(s_index.length);
                            let new_space = DiskEntryIndex {
                                orig_index: s_index.orig_index,
                                index: sk + 1,
                                length: diff,
                                item: DiskEntry::Space,
                                moved: false,
                            };

                            // Swap places
                            let tmp = DiskEntryIndex {
                                orig_index: curr_index.orig_index,
                                index: s_index.index,
                                length: curr_index.length,
                                item: curr_index.item,
                                moved: true,
                            };
                            indexes[i] = DiskEntryIndex {
                                orig_index: s_index.orig_index,
                                index: curr_index.index,
                                length: curr_index.length,
                                item: s_index.item,
                                moved: true,
                            };
                            indexes[sk] = tmp;

                            // Insert the new index after the space index filled with file
                            indexes.insert(sk + 1, new_space);

                            // We need to jump back up since we added a new entry
                            i += 1;
                        }
                    }
                }
            }
            DiskEntry::Space => (),
        }

        i -= 1;
    }

    // Rebuild the disk entries using the updated index
    let mut updated_entries: Vec<DiskEntry> = Vec::with_capacity(entries.len());
    for index in indexes.iter() {
        for _ in 0..index.length {
            updated_entries.push(index.item.clone());
        }
    }

    updated_entries
}

fn find_free_space(indexes: &Vec<DiskEntryIndex>, length: usize) -> Option<usize> {
    for (k, v) in indexes.iter().enumerate() {
        match v.item {
            DiskEntry::Space => {
                if v.length >= length {
                    return Some(k);
                }
            }
            DiskEntry::File(_) => (),
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

fn index_disk_entries(entries: &Vec<DiskEntry>) -> Vec<DiskEntryIndex> {
    // Index all files and space blocks so that we can locate them easily via a loop
    let mut indexes: Vec<DiskEntryIndex> = Vec::new();
    let mut prev: Option<DiskEntryIndex> = None;
    for (k, v) in entries.iter().enumerate() {
        match v {
            DiskEntry::File(f) => {
                if let Some(prev_index) = prev.as_mut() {
                    match prev_index.item {
                        DiskEntry::Space => {
                            // Suddenly encountered a space
                            // Complete the previous index and start a new space index
                            indexes.push(*prev_index);
                            prev = Some(DiskEntryIndex {
                                orig_index: k,
                                index: k,
                                length: 1,
                                item: *v,
                                moved: false,
                            });
                        }
                        DiskEntry::File(index_f) => {
                            if index_f.id == f.id {
                                // Same as previous, just increment the length
                                prev_index.length += 1;
                                prev = Some(*prev_index);
                            } else {
                                // Encountered a new file block
                                // Complete the previous index and start a new index
                                indexes.push(*prev_index);
                                prev = Some(DiskEntryIndex {
                                    orig_index: k,
                                    index: k,
                                    length: 1,
                                    item: *v,
                                    moved: false,
                                })
                            }
                        }
                    }
                } else {
                    // Start of the entries, start a new index
                    prev = Some(DiskEntryIndex {
                        orig_index: k,
                        index: k,
                        length: 1,
                        item: *v,
                        moved: false,
                    });
                }
            }
            DiskEntry::Space => {
                if let Some(prev_index) = prev.as_mut() {
                    match prev_index.item {
                        DiskEntry::Space => {
                            // Same as previous, just increment the length
                            prev_index.length += 1;
                            prev = Some(*prev_index);
                        }
                        DiskEntry::File(_) => {
                            // Suddenly encountered a space, mark the previous
                            // file as complete and record a new prev as a space
                            indexes.push(*prev_index);
                            prev = Some(DiskEntryIndex {
                                orig_index: k,
                                index: k,
                                length: 1,
                                item: *v,
                                moved: false,
                            })
                        }
                    }
                } else {
                    panic!("Entries cannot start with a space.");
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

        let entries = format_blocks(&blocks);

        let entries_str = format_disk_entries_str(&entries);
        assert_eq!(
            entries_str,
            "00...111...2...333.44.5555.6666.777.888899".to_string()
        );

        let entries = defrag_entries_contiguous(entries);

        println!("Input disk entries: ");
        println!("{}", entries_str);
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
