use std::error::Error;
use std::fs;

pub fn run() -> Result<(), Box<dyn Error>> {
    println!("Running Day 1!");

    let path_coords = "./src/day1/input.txt";
    let (mut coord1_vec, mut coord2_vec) = read_coords(path_coords)?;

    // We need to sort the vectors
    coord1_vec.sort();
    coord2_vec.sort();

    let diff_sum = compute_diff_sum(&coord1_vec, &coord2_vec)?;

    println!("Difference sum is: {:?}", diff_sum);

    Ok(())
}

fn compute_diff_sum(vec1: &[u64], vec2: &[u64]) -> Result<i64, Box<dyn Error>> {
    // Check that vectors are same length
    if vec1.len() != vec2.len() {
        return Err("vec1 and vec2 are not the same length".into());
    }

    // We need to compute the difference between each number in order.
    let diff_sum: i64 = vec1
        .iter()
        .zip(vec2.iter())
        .map(|(a, b)| (*a as i64 - *b as i64).abs())
        .sum();
    Ok(diff_sum)
}

fn read_coords(path: &str) -> Result<(Vec<u64>, Vec<u64>), Box<dyn Error>> {
    println!("Reading coordinates from {}", path);
    let data =
        fs::read_to_string(path).map_err(|e| format!("Unable to read file {}: {}", path, e))?;

    let mut coord1_vec: Vec<u64> = Vec::new();
    let mut coord2_vec: Vec<u64> = Vec::new();

    for (line_no, line) in data.lines().enumerate() {
        // Skip empty lines
        if line.trim().is_empty() {
            continue;
        }

        // Split lines by whitespace
        let mut parts = line.split_whitespace();

        let coord1_str = parts
            .next()
            .ok_or_else(|| format!("Missing first coordinate on line {}", line_no + 1))?;
        let coord2_str = parts
            .next()
            .ok_or_else(|| format!("Missing second coordinate on line {}", line_no + 1))?;

        let coord1: u64 = coord1_str
            .parse()
            .map_err(|e| format!("Error parsing coordinate 1 on line {}: {}", line_no + 1, e))?;
        let coord2: u64 = coord2_str
            .parse()
            .map_err(|e| format!("Error parsing coordinate 2 on line {}: {}", line_no + 1, e))?;

        coord1_vec.push(coord1);
        coord2_vec.push(coord2);
    }

    Ok((coord1_vec, coord2_vec))
}
