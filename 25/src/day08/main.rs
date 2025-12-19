use std::error::Error;
use std::fs;

pub fn run() -> Result<(), Box<dyn Error>> {
    println!("Running Day 08!");

    let input: String = fs::read_to_string("./src/day08/input.txt")?;

    Ok(())
}
