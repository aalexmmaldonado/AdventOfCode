use std::error::Error;
use std::fs;

pub fn run() -> Result<(), Box<dyn Error>> {
    println!("Running Day 4!");

    let path_instructions = "./src/day4/input.txt";
    let contents = read_file(&path_instructions)?;

    Ok(())
}

fn read_file(path: &str) -> Result<String, Box<dyn Error>> {
    println!("Reading file from {}", path);
    let contents =
        fs::read_to_string(path).map_err(|e| format!("Unable to read file {}: {}", path, e))?;

    Ok(contents)
}
