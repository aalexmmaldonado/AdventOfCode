use std::error::Error;
use std::fs;

pub fn run() -> Result<(), Box<dyn Error>> {
    println!("Running Day 04!");

    let input: String = fs::read_to_string("./src/day04/input.txt")?;

    Ok(())
}
