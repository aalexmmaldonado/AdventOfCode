use std::error::Error;
use std::fs;

pub fn run() -> Result<(), Box<dyn Error>> {
    println!("Running Day 9!");

    let path_input = "./src/day9/input.txt";
    let contents = read_file(&path_input)?;
    let file_blocks: Vec<String> = expand_disk_map(&contents);
    let file_blocks_defrag = defragment_file_blocks(&file_blocks);
    let checksum: u64 = compute_checksum(&file_blocks_defrag);
    assert!(checksum == 6340197768906);
    println!("File checksum: {:?}", checksum);

    let file_blocks_defrag_chunks = defragment_file_blocks_chunks(&file_blocks);
    let checksum: u64 = compute_checksum(&file_blocks_defrag_chunks);
    assert!(checksum == 6363913128533);
    println!("File checksum in blocks: {:?}", checksum);

    Ok(())
}

fn read_file(path: &str) -> Result<String, Box<dyn Error>> {
    println!("Reading file from {}", path);
    let contents =
        fs::read_to_string(path).map_err(|e| format!("Unable to read file {}: {}", path, e))?;

    Ok(contents)
}

fn expand_disk_map(contents: &str) -> Vec<String> {
    let mut file_index: u64 = 0; // Track the file index explicitly

    contents
        .chars() // Iterate over individual characters
        .enumerate()
        .flat_map(|(i, ch)| {
            if let Some(repeat_count) = ch.to_digit(10) {
                if i % 2 == 0 {
                    // For even indices, use the current file_index as a string
                    let current_index = file_index;
                    file_index += 1; // Increment file_index

                    // Create a vector with `repeat_count` entries of `current_index.to_string()`
                    std::iter::repeat(current_index.to_string())
                        .take(repeat_count as usize)
                        .collect::<Vec<_>>()
                } else {
                    // For odd indices, create a vector with `repeat_count` entries of "."
                    std::iter::repeat(".".to_string())
                        .take(repeat_count as usize)
                        .collect::<Vec<_>>()
                }
            } else {
                Vec::new() // Ignore non-digit characters
            }
        })
        .collect()
}

fn defragment_file_blocks(file_blocks: &[String]) -> Vec<String> {
    let n_blocks = file_blocks.len();
    let mut defrag_blocks = file_blocks.to_vec(); // Clone into a new Vec<String>
    let mut j = n_blocks - 1; // Start from the last index

    for i in 0..n_blocks {
        // We can place a file block here
        if defrag_blocks[i] == "." {
            // Move j index back until we find a non-'.'
            while j > i && defrag_blocks[j] == "." {
                j -= 1;
            }

            // If j <= i, we are done defragmenting
            if j <= i {
                break;
            }

            // Exchange
            defrag_blocks[i] = defrag_blocks[j].clone();
            defrag_blocks[j] = ".".to_string();
            j -= 1; // Move j back for the next swap
        }
    }

    defrag_blocks
}

fn defragment_file_blocks_chunks(file_blocks: &[String]) -> Vec<String> {
    let mut defrag_blocks = file_blocks.to_vec(); // Clone into a new Vec<String>
    let mut file_positions: Vec<(usize, usize, String)> = Vec::new();

    // Extract file positions and IDs (from right to left)
    let mut i = file_blocks.len();
    while i > 0 {
        i -= 1; // Decrement at the start to avoid out-of-bounds errors
        if file_blocks[i] != "." && file_blocks[i] != "0" {
            let end = i;
            let id = file_blocks[i].clone();
            while i > 0 && file_blocks[i - 1] == id {
                i -= 1;
            }
            let start = i; // Start of the current file
            file_positions.push((start, end, id));
        }
    }

    // Try to move each file into the leftmost available space
    for (start, end, id) in file_positions {
        let file_length = end - start + 1;

        let mut empty_start = None;
        let mut empty_len = 0;

        for i_empty in 0..defrag_blocks.len() {
            if defrag_blocks[i_empty] == "." {
                // Start counting empty space
                if empty_start.is_none() {
                    empty_start = Some(i_empty);
                }
                empty_len += 1;

                // Check if the file can fit in the empty space
                if empty_len == file_length {
                    if let Some(empty_start_idx) = empty_start {
                        if empty_start_idx > start {
                            break; // Skip if the empty index is higher than the file index
                        }

                        // Move the file to the empty space
                        for j in 0..file_length {
                            defrag_blocks[empty_start_idx + j] = id.clone();
                        }

                        // Clear the original file blocks
                        for j in start..=end {
                            defrag_blocks[j] = ".".to_string();
                        }

                        // File has been moved, exit the loop
                        break;
                    }
                }
            } else {
                // Reset empty space tracking
                empty_start = None;
                empty_len = 0;
            }
        }
    }

    defrag_blocks
}

fn compute_checksum(file_blocks: &[String]) -> u64 {
    file_blocks
        .iter()
        .enumerate()
        .filter_map(|(i, ch)| {
            if ch != "." {
                match ch.parse::<u64>() {
                    Ok(num) => Some(i as u64 * num),
                    Err(_) => None, // Skip invalid numbers
                }
            } else {
                None // Skip "."
            }
        })
        .sum() // Sum up all valid products
}
