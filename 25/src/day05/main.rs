use std::error::Error;
use std::fs;

pub fn run() -> Result<(), Box<dyn Error>> {
    println!("Running Day 05!");

    let _input: String = fs::read_to_string("./src/day05/input.txt")?;

    Ok(())
}
