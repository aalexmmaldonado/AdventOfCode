use std::error::Error;
use std::fs;

pub fn run() -> Result<(), Box<dyn Error>> {
    println!("Running Day 12!");

    let _input: String = fs::read_to_string("./src/day12/input.txt")?;

    Ok(())
}
