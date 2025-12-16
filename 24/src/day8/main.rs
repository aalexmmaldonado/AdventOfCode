use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;

pub fn run() -> Result<(), Box<dyn Error>> {
    println!("Running Day 8!");

    let path_input = "./src/day8/input.txt";
    let contents = read_file(&path_input)?;
    let grid = create_grid(&contents);
    let grid = encode_grid(grid);
    let locations = get_locations(&grid);
    let antinodes = get_antinodes(&locations, grid.len() as i64, grid[0].len() as i64, false);
    let n_antinodes = antinodes.len();
    assert_eq!(n_antinodes, 240);
    println!("Number of antinodes: {:?}", n_antinodes);

    let antinodes_periodic =
        get_antinodes(&locations, grid.len() as i64, grid[0].len() as i64, true);
    let n_antinodes_periodic = antinodes_periodic.len();
    assert_eq!(n_antinodes_periodic, 955);
    println!("Number of periodic antinodes: {:?}", n_antinodes_periodic);

    Ok(())
}

fn read_file(path: &str) -> Result<String, Box<dyn Error>> {
    println!("Reading file from {}", path);
    let contents =
        fs::read_to_string(path).map_err(|e| format!("Unable to read file {}: {}", path, e))?;

    Ok(contents)
}

fn create_grid(contents: &str) -> Vec<Vec<&str>> {
    let grid: Vec<Vec<&str>> = contents
        .lines()
        .map(|line| line.split("").filter(|&s| !s.is_empty()).collect())
        .collect();
    grid
}

fn encode_grid(grid: Vec<Vec<&str>>) -> Vec<Vec<i16>> {
    grid.iter()
        .map(|row| {
            row.iter()
                .map(|&s| {
                    s.chars()
                        .next()
                        .map(|c| if c == '.' { 0 } else { c as i16 })
                        .unwrap_or_default()
                })
                .collect()
        })
        .collect()
}

fn get_locations(grid_encoded: &Vec<Vec<i16>>) -> HashMap<i16, Vec<(usize, usize)>> {
    let mut index_map: HashMap<i16, Vec<(usize, usize)>> = HashMap::new();

    for (row_index, row) in grid_encoded.iter().enumerate() {
        for (col_index, &value) in row.iter().enumerate() {
            if value != 0 {
                index_map
                    .entry(value)
                    .or_insert_with(Vec::new)
                    .push((row_index, col_index));
            }
        }
    }

    index_map
}

fn is_contained(r: &i64, c: &i64, n_rows: &i64, n_columns: &i64) -> bool {
    let contained = *r >= 0 && *r < *n_rows && *c >= 0 && *c < *n_columns;
    contained
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a.abs()
    } else {
        gcd(b, a % b)
    }
}

fn get_antinodes(
    locations: &HashMap<i16, Vec<(usize, usize)>>,
    n_rows: i64,
    n_columns: i64,
    use_harmonics: bool,
) -> Vec<(usize, usize)> {
    let mut antinode_set: HashSet<(usize, usize)> = HashSet::new();

    for (_freq, coords) in locations.iter() {
        // Consider all pairs of antennas for this frequency
        for i in 0..coords.len() {
            for j in (i + 1)..coords.len() {
                let (r0, c0) = coords[i];
                let (r1, c1) = coords[j];

                if !use_harmonics {
                    // Original logic (no harmonics):
                    // Just place the two antinodes at P1 and P2
                    let p1_r: i64 = 2 * r0 as i64 - r1 as i64;
                    let p1_c: i64 = 2 * c0 as i64 - c1 as i64;

                    if is_contained(&p1_r, &p1_c, &n_rows, &n_columns) {
                        antinode_set.insert((p1_r as usize, p1_c as usize));
                    }

                    let p2_r: i64 = 2 * r1 as i64 - r0 as i64;
                    let p2_c: i64 = 2 * c1 as i64 - c0 as i64;

                    if is_contained(&p2_r, &p2_c, &n_rows, &n_columns) {
                        antinode_set.insert((p2_r as usize, p2_c as usize));
                    }
                } else {
                    // Harmonics logic:
                    // An antinode occurs at ANY grid position on the line defined by these two antennas
                    let dr = r1 as i64 - r0 as i64;
                    let dc = c1 as i64 - c0 as i64;
                    let g = gcd(dr, dc);
                    let step_r = dr / g;
                    let step_c = dc / g;

                    // Move forward along the line from (r0, c0)
                    let mut k = 0;
                    loop {
                        let nr = r0 as i64 + k * step_r;
                        let nc = c0 as i64 + k * step_c;
                        if !is_contained(&nr, &nc, &n_rows, &n_columns) {
                            break;
                        }
                        antinode_set.insert((nr as usize, nc as usize));
                        k += 1;
                    }

                    // Move backward along the line from (r0, c0)
                    // Start k at 1 because k=0 was already included above
                    let mut k = 1;
                    loop {
                        let nr = r0 as i64 - k * step_r;
                        let nc = c0 as i64 - k * step_c;
                        if !is_contained(&nr, &nc, &n_rows, &n_columns) {
                            break;
                        }
                        antinode_set.insert((nr as usize, nc as usize));
                        k += 1;
                    }
                }
            }
        }
    }

    antinode_set.into_iter().collect()
}
