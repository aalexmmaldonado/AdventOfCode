use std::collections::HashMap;
use std::error::Error;
use std::fs;

pub fn run() -> Result<(), Box<dyn Error>> {
    println!("Running Day 11!");

    let path_input = "./src/day11/input.txt";
    let contents = read_file(&path_input)?;
    let stones = get_stones(contents)?;
    let n_blinks = 75; // Set the number of blinks
    let result = accumulate_stones(&stones, n_blinks);
    println!("Number of stones after {} blinks: {}", n_blinks, result);

    Ok(())
}

fn read_file(path: &str) -> Result<String, Box<dyn Error>> {
    println!("Reading file from {}", path);
    let contents =
        fs::read_to_string(path).map_err(|e| format!("Unable to read file {}: {}", path, e))?;

    Ok(contents)
}

fn get_stones(contents: String) -> Result<Vec<usize>, Box<dyn Error>> {
    let stones = contents
        .split_whitespace()
        .map(|a| a.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()?;
    Ok(stones)
}

fn accumulate_stones(initial_stones: &[usize], n_blinks: usize) -> usize {
    // current_counts will keep track of stones and how many there are
    // Keys are the stone values and values are the quantity of these stones.
    let mut current_counts: HashMap<usize, usize> = HashMap::new();

    // Initialize counts for the initial stones
    for &stone in initial_stones {
        *current_counts.entry(stone).or_insert(0) += 1;
    }

    // Cache to store results of the blinked function to avoid recalculating
    let mut cache: HashMap<usize, Vec<usize>> = HashMap::new();

    // Loop through each blink
    for _ in 0..n_blinks {
        // Holds new stone counts for after blink
        let mut next_counts: HashMap<usize, usize> = HashMap::new();

        // Computes what happens to each stone an updates new_stone
        for (&stone, &count) in &current_counts {
            let results = cache.entry(stone).or_insert_with(|| blinked(stone));
            for &new_stone in results.iter() {
                *next_counts.entry(new_stone).or_insert(0) += count;
            }
        }
        current_counts = next_counts;
    }

    // Sum the counts to get the total number of stones
    current_counts.values().sum::<usize>() as usize
}

fn blinked(stone: usize) -> Vec<usize> {
    let mut new_stones: Vec<usize> = Vec::new();
    if stone == 0 {
        // If the stone is 0, create a new stone with value 1
        new_stones.push(1);
    } else {
        let mut digits = Vec::new();
        let mut n = stone;
        while n > 0 {
            digits.push(n % 10);
            n /= 10;
        }
        digits.reverse();
        let len = digits.len();

        if len % 2 == 0 {
            // Split into two halves
            let mid = len / 2;
            let left = digits[0..mid].iter().fold(0, |acc, &d| acc * 10 + d);
            let right = digits[mid..].iter().fold(0, |acc, &d| acc * 10 + d);
            new_stones.push(left);
            new_stones.push(right);
        } else {
            // Multiply the stone's number by 2024
            new_stones.push(stone * 2024);
        }
    }
    new_stones
}
