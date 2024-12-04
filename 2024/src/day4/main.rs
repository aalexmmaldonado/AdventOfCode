use std::error::Error;
use std::fs;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub fn run() -> Result<(), Box<dyn Error>> {
    println!("Running Day 4!");

    let path_instructions = "./src/day4/input.txt";
    let word = String::from("XMAS");
    let contents = read_file(&path_instructions)?;
    let grid = to_grid(&contents);

    let word_locs = search_grid_for_word(&grid, &word);
    let n_word_found = word_locs.len();
    assert_eq!(n_word_found, 2575);
    println!("Times '{}' was found: {:?}", word, n_word_found);

    let x_word_locs = search_grid_for_x_word(&grid);
    let n_word_found = x_word_locs.len();
    assert!(n_word_found == 2041);
    println!("Times 'MAS' was found in X pattern: {:?}", n_word_found);

    Ok(())
}

fn read_file(path: &str) -> Result<String, Box<dyn Error>> {
    println!("Reading file from {}", path);
    let contents =
        fs::read_to_string(path).map_err(|e| format!("Unable to read file {}: {}", path, e))?;

    Ok(contents)
}

fn to_grid(contents: &String) -> Vec<Vec<&str>> {
    let grid: Vec<Vec<&str>> = contents
        .lines()
        .map(|line| line.split("").filter(|&s| !s.is_empty()).collect())
        .collect();
    grid
}

#[derive(Debug, EnumIter)]
enum GridDirection {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl GridDirection {
    fn delta(&self) -> (isize, isize) {
        match self {
            GridDirection::Up => (-1, 0),
            GridDirection::Down => (1, 0),
            GridDirection::Left => (0, -1),
            GridDirection::Right => (0, 1),
            GridDirection::UpLeft => (-1, -1),
            GridDirection::UpRight => (-1, 1),
            GridDirection::DownLeft => (1, -1),
            GridDirection::DownRight => (1, 1),
        }
    }
}

fn traverse_grid(
    start: (usize, usize),
    direction: &GridDirection,
    rows: usize,
    cols: usize,
    step: usize,
) -> Option<(usize, usize)> {
    let (row, col) = start;
    let (delta_row, delta_col) = direction.delta();

    // Calculate new position
    let mut new_row = row as isize;
    let mut new_col = col as isize;
    for _ in 0..step {
        new_row = new_row + delta_row;
        new_col = new_col + delta_col;
    }

    // Check bounds
    if new_row >= 0 && new_row < rows as isize && new_col >= 0 && new_col < cols as isize {
        Some((new_row as usize, new_col as usize))
    } else {
        None // Out of bounds
    }
}

fn check_for_word(
    word: &str,
    start: (usize, usize),
    direction: &GridDirection,
    grid: &Vec<Vec<&str>>,
) -> bool {
    let (row, col) = start;
    let rows = grid.len();
    let cols = grid[0].len();

    let letters: Vec<&str> = word.split("").filter(|&s| !s.is_empty()).collect();

    // Check if starting position matches
    if grid[row][col] != letters[0] {
        return false;
    }

    for i_letter in 1..letters.len() {
        let next_loc = traverse_grid(start, direction, rows, cols, i_letter);
        match next_loc {
            None => return false,
            Some(next_loc) => {
                let (row_next, col_next) = next_loc;
                if grid[row_next][col_next] != letters[i_letter] {
                    return false;
                }
            }
        }
    }

    true
}

fn search_grid_for_word(grid: &Vec<Vec<&str>>, word: &str) -> Vec<(usize, usize, GridDirection)> {
    let mut word_locs: Vec<(usize, usize, GridDirection)> = Vec::new();
    let rows = grid.len();
    let cols = grid[0].len();

    for i in 0..rows {
        for j in 0..cols {
            for direction in GridDirection::iter() {
                if check_for_word(word, (i, j), &direction, grid) {
                    word_locs.push((i, j, direction));
                }
            }
        }
    }

    word_locs
}

fn get_other_letter(letter: &str) -> &str {
    if letter == "S" {
        return "M";
    } else if letter == "M" {
        return "S";
    } else {
        panic!("You have an invalid letter!");
    };
}

fn search_grid_for_x_word(grid: &Vec<Vec<&str>>) -> Vec<(usize, usize)> {
    let mut word_locs: Vec<(usize, usize)> = Vec::new();
    let rows = grid.len();
    let cols = grid[0].len();

    let letter_center = "A";

    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] != letter_center {
                continue;
            }

            let mut letter = "";

            let next_loc = traverse_grid((i, j), &GridDirection::UpLeft, rows, cols, 1);
            match next_loc {
                None => continue,
                Some(next_loc) => {
                    let (row_next, col_next) = next_loc;
                    letter = grid[row_next][col_next];
                    if (letter == "X") | (letter == "A") {
                        continue;
                    }
                    let opposite_loc =
                        traverse_grid((i, j), &GridDirection::DownRight, rows, cols, 1);
                    match opposite_loc {
                        None => continue,
                        Some(opposite_loc) => {
                            let (row_next, col_next) = opposite_loc;
                            if grid[row_next][col_next] != get_other_letter(&letter) {
                                continue;
                            }
                        }
                    }
                }
            }

            let next_loc = traverse_grid((i, j), &GridDirection::UpRight, rows, cols, 1);
            match next_loc {
                None => continue,
                Some(next_loc) => {
                    let (row_next, col_next) = next_loc;
                    letter = grid[row_next][col_next];
                    if (letter == "X") | (letter == "A") {
                        continue;
                    }
                    let opposite_loc =
                        traverse_grid((i, j), &GridDirection::DownLeft, rows, cols, 1);
                    match opposite_loc {
                        None => continue,
                        Some(opposite_loc) => {
                            let (row_next, col_next) = opposite_loc;
                            if grid[row_next][col_next] != get_other_letter(&letter) {
                                continue;
                            }
                        }
                    }
                }
            }

            // let next_loc = traverse_grid((i, j), &GridDirection::DownLeft, rows, cols, 1);
            // match next_loc {
            //     None => continue,
            //     Some(next_loc) => {
            //         let (row_next, col_next) = next_loc;
            //         let next_letter = grid[row_next][col_next];
            //         println!("{}", next_letter);
            //         if next_letter != letter {
            //             continue;
            //         }
            //         let opposite_loc =
            //             traverse_grid((i, j), &GridDirection::DownRight, rows, cols, 1);
            //         match opposite_loc {
            //             None => continue,
            //             Some(opposite_loc) => {
            //                 let (row_next, col_next) = opposite_loc;
            //                 let next_letter = grid[row_next][col_next];
            //                 println!("{}", next_letter);
            //                 if next_letter != letter {
            //                     continue;
            //                 }
            //             }
            //         }
            //     }
            // }

            word_locs.push((i, j));
        }
    }

    word_locs
}
