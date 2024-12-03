use std::error::Error;
use std::fs;

pub fn run() -> Result<(), Box<dyn Error>> {
    println!("Running Day 2!");

    let path_levels = "./src/day2/input.txt";
    let levels = read_levels(path_levels)?;
    
    let n_safe: _ = levels.iter()
        .map(|a| is_safe_basic(a))
        .filter(|b| *b)  // if it is true.
        .collect::<Vec<bool>>()
        .len();
    println!("{:?}", n_safe);

    Ok(())
}


fn read_levels(path: &str) -> Result<Vec<Vec<u64>>, Box<dyn Error>> {
    println!("Reading levels from {}", path);
    let data =
        fs::read_to_string(path).map_err(|e| format!("Unable to read file {}: {}", path, e))?;

    let mut levels: Vec<Vec<u64>> = Vec::new();

    for (_, line) in data.lines().enumerate() {
        // Skip empty lines
        if line.trim().is_empty() {
            continue;
        }

        // Split lines by whitespace
        let level: Vec<u64> = line
            .split_whitespace()
            .map(|a| a.parse::<u64>())
            .collect::<Result<Vec<_>, _>>()?;
        
        levels.push(level);
    }

    Ok(levels)
}

fn is_safe_basic(nums: &[u64]) -> bool {
    // Initialize flags for increasing and decreasing
    let mut increasing = true;
    let mut decreasing = true;

    // Iterate through the list
    for i in 1..nums.len() {
        if nums[i].abs_diff(nums[i - 1]) > 3 {
            return false;
        }
        if nums[i] > nums[i - 1] {
            decreasing = false;
        }
        if nums[i] < nums[i - 1] {
            increasing = false;
        }
        if nums[i] == nums[i - 1] {
            return false;
        }
        // If neither increasing nor decreasing, we can stop early
        if !increasing && !decreasing {
            return false;
        }
    }

    // If either increasing or decreasing, the list is monotonic
    increasing || decreasing
}
