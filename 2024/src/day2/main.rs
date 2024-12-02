use std::error::Error;
use std::fs;

pub fn run() -> Result<(), Box<dyn Error>> {
    println!("Running Day 2!");

    let path_levels = "./src/day2/input.txt";
    let levels = read_levels(path_levels)?;
    println!("{:?}", levels);

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
