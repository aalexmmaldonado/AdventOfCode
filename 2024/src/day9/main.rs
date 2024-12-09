use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;

pub fn run() -> Result<(), Box<dyn Error>> {
    println!("Running Day 9!");

    let path_input_test = "./src/day9/input-test.txt";
    let contents = read_file(&path_input_test)?;
    println!("{:?}", contents);

    Ok(())
}

fn read_file(path: &str) -> Result<String, Box<dyn Error>> {
    println!("Reading file from {}", path);
    let contents =
        fs::read_to_string(path).map_err(|e| format!("Unable to read file {}: {}", path, e))?;

    Ok(contents)
}
