use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;

use rayon::prelude::*;

pub fn run() -> Result<(), Box<dyn Error>> {
    println!("Running Day 6!");

    let path_input = "./src/day6/input.txt";
    let contents = read_file(&path_input)?;

    let (map, start_x, start_y, dir) = parse_map(&contents)?;

    let visited_count = simulate(&map, (start_x, start_y), dir);
    assert_eq!(visited_count, 5199);
    println!("Number of distinct visited positions: {}", visited_count);

    // Find all loop-causing obstruction positions
    let loop_positions = find_loop_positions(&mut map.clone(), (start_x, start_y), dir);
    assert!(loop_positions.len() < 2925);

    println!(
        "Number of loop-causing obstruction positions: {}",
        loop_positions.len()
    );

    Ok(())
}

fn read_file(path: &str) -> Result<String, Box<dyn Error>> {
    println!("Reading file from {}", path);
    let contents =
        fs::read_to_string(path).map_err(|e| format!("Unable to read file {}: {}", path, e))?;
    Ok(contents)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn forward(&self, (x, y): (usize, usize)) -> (usize, usize) {
        match self {
            Direction::Up => (x.saturating_sub(1), y),
            Direction::Right => (x, y + 1),
            Direction::Down => (x + 1, y),
            Direction::Left => (x, y.saturating_sub(1)),
        }
    }
}

fn parse_map(contents: &str) -> Result<(Vec<Vec<char>>, usize, usize, Direction), Box<dyn Error>> {
    let lines: Vec<&str> = contents.lines().collect();
    let mut map: Vec<Vec<char>> = Vec::new();

    for line in &lines {
        map.push(line.chars().collect());
    }

    // Find guard initial position and direction
    let mut start_x = 0;
    let mut start_y = 0;
    let mut dir = Direction::Up;
    let mut found = false;
    for (i, row) in map.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            dir = match c {
                '^' => {
                    start_x = i;
                    start_y = j;
                    found = true;
                    Direction::Up
                }
                '>' => {
                    start_x = i;
                    start_y = j;
                    found = true;
                    Direction::Right
                }
                'v' => {
                    start_x = i;
                    start_y = j;
                    found = true;
                    Direction::Down
                }
                '<' => {
                    start_x = i;
                    start_y = j;
                    found = true;
                    Direction::Left
                }
                _ => continue,
            };
            if found {
                break;
            }
        }
        if found {
            break;
        }
    }

    if !found {
        return Err("No guard found in the map".into());
    }

    // Replace the guard symbol with '.' because it's essentially an empty space after we start
    map[start_x][start_y] = '.';

    Ok((map, start_x, start_y, dir))
}

fn simulate(map: &[Vec<char>], start: (usize, usize), mut dir: Direction) -> usize {
    let rows = map.len();
    let cols = map[0].len();
    let mut visited = HashSet::new();
    let mut pos = start;
    visited.insert(pos);

    loop {
        let next_pos = dir.forward(pos);

        if out_of_bounds(next_pos, rows, cols) {
            // Guard leaves the map
            break;
        }

        if map[next_pos.0][next_pos.1] == '#' {
            // There's an obstacle, turn right
            dir = dir.turn_right();
        } else {
            // Move forward
            pos = next_pos;
            visited.insert(pos);
        }
    }

    visited.len()
}

fn out_of_bounds((x, y): (usize, usize), rows: usize, cols: usize) -> bool {
    x >= rows || y >= cols
}

fn find_loop_positions(
    map: &[Vec<char>],
    start_pos: (usize, usize),
    start_dir: Direction,
) -> Vec<(usize, usize)> {
    let rows = map.len();
    let cols = map[0].len();

    // We'll consider every coordinate in parallel
    (0..rows)
        .into_par_iter()
        .flat_map(|x| {
            (0..cols).into_par_iter().filter_map(move |y| {
                // Only consider candidate obstruction positions
                if (x, y) != start_pos && map[x][y] == '.' {
                    // Make a copy of map so we can modify it safely in parallel
                    let mut map_copy = map.to_vec();
                    map_copy[x][y] = '#';
                    if causes_loop(&map_copy, start_pos, start_dir) {
                        Some((x, y))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
        })
        .collect()
}

fn causes_loop(map: &[Vec<char>], start_pos: (usize, usize), start_dir: Direction) -> bool {
    let rows = map.len();
    let cols = map[0].len();

    let mut pos = start_pos;
    let mut dir = start_dir;
    let mut state_counts = HashMap::new();

    // Record the initial state
    state_counts.insert((pos, dir), 1u32);

    loop {
        let next_pos = dir.forward(pos);

        if out_of_bounds(next_pos, rows, cols) {
            // Guard leaves the map: no loop.
            return false;
        }

        if map[next_pos.0][next_pos.1] == '#' {
            // Turn right due to an obstacle
            dir = dir.turn_right();
        } else {
            // Move forward
            pos = next_pos;
        }

        let count = state_counts.entry((pos, dir)).or_insert(0);
        *count += 1;

        // If the state has been encountered at least 3 times, we declare a loop.
        if *count >= 3 {
            return true;
        }
    }
}
