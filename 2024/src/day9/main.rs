use std::collections::{HashMap, HashSet};
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
