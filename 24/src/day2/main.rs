use std::error::Error;
use std::fs;

pub fn run() -> Result<(), Box<dyn Error>> {
    println!("Running Day 2!");

    let path_levels = "./src/day2/input.txt";
    let levels = read_levels(path_levels)?;

    let n_safe = levels.iter().filter(|&a| is_safe(a)).count();
    assert_eq!(n_safe, 334);
    println!("Safe levels (no dampener): {:?}", n_safe);

    let n_safe_with_dampener = levels.iter().filter(|&a| is_safe_with_dampener(a)).count();
    assert_eq!(n_safe_with_dampener, 400);
    println!("Safe levels (with dampener): {:?}", n_safe_with_dampener);

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

fn is_safe(nums: &[u64]) -> bool {
    if nums.len() < 2 {
        return true;
    }

    // Determine if the sequence is increasing or decreasing
    let increasing = nums[1] > nums[0];
    let decreasing = nums[1] < nums[0];

    // If the first two numbers are equal, the sequence is neither increasing nor decreasing
    if nums[1] == nums[0] {
        return false;
    }

    // Check the differences between adjacent levels
    for i in 1..nums.len() {
        let diff = nums[i].abs_diff(nums[i - 1]);

        // The difference must be at least 1 and at most 3
        if diff < 1 || diff > 3 {
            return false;
        }

        if increasing && nums[i] <= nums[i - 1] {
            return false;
        }

        if decreasing && nums[i] >= nums[i - 1] {
            return false;
        }

        if !increasing && !decreasing {
            return false;
        }
    }

    true
}

fn is_safe_with_dampener(nums: &[u64]) -> bool {
    if is_safe(nums) {
        return true;
    }

    for i in 0..nums.len() {
        let mut nums_without_i = nums.to_vec();
        nums_without_i.remove(i);
        if is_safe(&nums_without_i) {
            return true;
        }
    }

    false
}
