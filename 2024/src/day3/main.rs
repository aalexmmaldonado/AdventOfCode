use std::error::Error;
use std::fs;

pub fn run() -> Result<(), Box<dyn Error>> {
    println!("Running Day 3!");

    let path_instructions = "./src/day3/input.txt";
    let contents = read_file(&path_instructions)?;
    let mults = find_mults(&contents);
    println!("Parsed muls: {:?}", mults);
    let mults_sum = get_mult_sums(&mults);
    assert_eq!(mults_sum, 178886550);
    println!("Parsed mult sum: {:?}", mults_sum);

    Ok(())
}

fn read_file(path: &str) -> Result<String, Box<dyn Error>> {
    println!("Reading file from {}", path);
    let contents =
        fs::read_to_string(path).map_err(|e| format!("Unable to read file {}: {}", path, e))?;

    Ok(contents)
}

fn find_mults(contents: &str) -> Vec<String> {
    contents
        .split("mul(")
        .skip(1)
        .filter_map(|segment| segment.split(')').next())
        .filter(|s| s.contains(',') && s.len() <= 7)
        .map(|s| s.to_string())
        .collect()
}

fn get_mult_sums(mults: &Vec<String>) -> u64 {
    mults
        .iter()
        .map(|s| {
            let nums: Vec<u64> = s
                .split(',')
                .filter_map(|num_str| num_str.trim().parse::<u64>().ok())
                .collect();
            if nums.len() == 2 {
                nums[0] * nums[1]
            } else {
                0
            }
        })
        .sum()
}
