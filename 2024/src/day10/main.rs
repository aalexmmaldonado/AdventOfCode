use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::fs;

pub fn run() -> Result<(), Box<dyn Error>> {
    println!("Running Day 10!");

    let path_input = "./src/day10/input.txt";
    let contents = read_file(&path_input)?;

    let map = parse_map(&contents);
    let total_score = calculate_trailhead_scores(&map);
    assert!(total_score == 820);
    println!("Total score of all trailheads: {}", total_score);

    let total_rating = calculate_trailhead_ratings(&map);
    assert!(total_rating == 1786);
    println!("Total rating of all trailheads: {}", total_rating);

    Ok(())
}

fn read_file(path: &str) -> Result<String, Box<dyn Error>> {
    println!("Reading file from {}", path);
    let contents =
        fs::read_to_string(path).map_err(|e| format!("Unable to read file {}: {}", path, e))?;

    Ok(contents)
}

fn parse_map(contents: &str) -> Vec<Vec<u32>> {
    contents
        .lines()
        .map(|line| line.chars().filter_map(|c| c.to_digit(10)).collect())
        .collect()
}

fn calculate_trailhead_scores(map: &[Vec<u32>]) -> u32 {
    let mut total_score = 0;

    // Loop though each index.
    for row in 0..map.len() {
        for col in 0..map[0].len() {
            // Only start at zero
            if map[row][col] == 0 {
                total_score += bfs_trailhead_score(map, row as isize, col as isize);
            }
        }
    }

    total_score
}

// Implements breadth first search
fn bfs_trailhead_score(map: &[Vec<u32>], start_row: isize, start_col: isize) -> u32 {
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)]; // right, down, left, up
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back((start_row, start_col, 0)); // (row, col, current height)
    visited.insert((start_row, start_col));

    let mut score = 0;

    while let Some((row, col, height)) = queue.pop_front() {
        if map[row as usize][col as usize] == 9 {
            score += 1;
            continue;
        }

        // Loops through each possible direction
        for &(dr, dc) in &directions {
            // Indices of potential moves
            let new_row = row + dr;
            let new_col = col + dc;

            // Checks if new indices are out of bounds.
            // Also checks if we have already tried this index.
            if new_row >= 0
                && new_col >= 0
                && (new_row as usize) < map.len()
                && (new_col as usize) < map[0].len()
                && !visited.contains(&(new_row, new_col))
            {
                // Checks if the height only goes up one.
                // If it does, then we consider this a potential trail.
                let next_height = map[new_row as usize][new_col as usize];
                if next_height == height + 1 {
                    // Keeps track of where to check for moves next.
                    visited.insert((new_row, new_col));
                    queue.push_back((new_row, new_col, next_height));
                }
            }
        }
    }

    score
}

// Calculate total trailhead ratings
fn calculate_trailhead_ratings(map: &[Vec<u32>]) -> u32 {
    let mut total_rating = 0;

    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if map[row][col] == 0 {
                total_rating +=
                    dfs_trailhead_rating(map, row as isize, col as isize, &mut HashSet::new());
            }
        }
    }

    total_rating
}

// Depth-First Search to count all distinct hiking trails
fn dfs_trailhead_rating(
    map: &[Vec<u32>],
    row: isize,
    col: isize,
    visited: &mut HashSet<(isize, isize)>,
) -> u32 {
    // If we reach height 9, this is a valid trail
    if map[row as usize][col as usize] == 9 {
        return 1;
    }

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut trail_count = 0;

    // Temporarily mark the current position as visited
    visited.insert((row, col));

    for &(dr, dc) in &directions {
        let new_row = row + dr;
        let new_col = col + dc;

        if new_row >= 0
            && new_col >= 0
            && (new_row as usize) < map.len()
            && (new_col as usize) < map[0].len()
            && !visited.contains(&(new_row, new_col))
        {
            let current_height = map[row as usize][col as usize];
            let next_height = map[new_row as usize][new_col as usize];

            // Continue the trail only if the height increases by 1
            if next_height == current_height + 1 {
                trail_count += dfs_trailhead_rating(map, new_row, new_col, visited);
            }
        }
    }

    // Backtrack: unmark the current position as visited
    visited.remove(&(row, col));

    trail_count
}
