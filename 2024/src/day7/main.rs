use std::error::Error;
use std::fs;

pub fn run() -> Result<(), Box<dyn Error>> {
    println!("Running Day 7!");

    let path_input = "./src/day7/input.txt";
    let contents = read_file(&path_input)?;

    Ok(())
}

fn read_file(path: &str) -> Result<String, Box<dyn Error>> {
    println!("Reading file from {}", path);
    let contents =
        fs::read_to_string(path).map_err(|e| format!("Unable to read file {}: {}", path, e))?;

    Ok(contents)
}
