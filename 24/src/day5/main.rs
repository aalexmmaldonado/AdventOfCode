use std::error::Error;
use std::fs;

use std::collections::HashSet;

pub fn run() -> Result<(), Box<dyn Error>> {
    println!("Running Day 5!");

    let path_input = "./src/day5/input.txt";
    let contents = read_file(&path_input)?;
    let rules = parse_rules(&contents);
    let updates = parse_updates(&contents);
    let updates_valid = get_valid_updates(&updates, &rules);
    let middle_values = get_middle_values(&updates_valid);
    let middle_sum: u64 = middle_values.iter().sum();
    assert_eq!(middle_sum, 4996);
    println!("Valid middle sum: {:?}", middle_sum);

    let updates_invalid = get_invalid_updates(&updates, &rules);
    let updates_corrected: Vec<Vec<&str>> = updates_invalid
        .iter()
        .map(|a| fix_ordering(&a, &rules))
        .collect();
    let middle_values = get_middle_values(&updates_corrected);
    let middle_sum: u64 = middle_values.iter().sum();
    assert_eq!(middle_sum, 6311);
    println!("Corrected middle sum: {:?}", middle_sum);

    Ok(())
}

fn read_file(path: &str) -> Result<String, Box<dyn Error>> {
    println!("Reading file from {}", path);
    let contents =
        fs::read_to_string(path).map_err(|e| format!("Unable to read file {}: {}", path, e))?;

    Ok(contents)
}

fn parse_rules(content: &str) -> HashSet<&str> {
    let mut rules = HashSet::new();
    for line in content.lines().filter(|s| s.contains("|")) {
        rules.insert(line);
    }
    rules
}

fn parse_updates(content: &str) -> Vec<Vec<&str>> {
    let mut updates = Vec::new();
    for line in content
        .lines()
        .filter(|s| s.contains(",") && !s.contains("|"))
    {
        let parsed_line = line.split(',').collect();
        updates.push(parsed_line);
    }
    updates
}

fn get_valid_updates<'a>(
    updates: &'a Vec<Vec<&'a str>>,
    rules: &'a HashSet<&'a str>,
) -> Vec<Vec<&'a str>> {
    let mut updates_correct = Vec::new();

    'outer: for update in updates {
        for i in 0..update.len() {
            for j in i + 1..update.len() {
                let candidate = format!("{}|{}", update[j], update[i]);
                if rules.contains(candidate.as_str()) {
                    continue 'outer;
                }
            }
        }
        updates_correct.push(update.clone());
    }

    updates_correct
}

fn get_invalid_updates<'a>(
    updates: &'a Vec<Vec<&'a str>>,
    rules: &'a HashSet<&'a str>,
) -> Vec<Vec<&'a str>> {
    let mut updates_invalid = Vec::new();

    'outer: for update in updates {
        for i in 0..update.len() {
            for j in i + 1..update.len() {
                let candidate = format!("{}|{}", update[j], update[i]);
                if rules.contains(candidate.as_str()) {
                    updates_invalid.push(update.clone());
                    continue 'outer;
                }
            }
        }
    }

    updates_invalid
}

fn middle_value(vec: &[u64]) -> Option<u64> {
    if vec.is_empty() {
        None
    } else {
        let middle_index = vec.len() / 2;
        Some(vec[middle_index])
    }
}

fn get_middle_values(updates: &[Vec<&str>]) -> Vec<u64> {
    updates
        .iter()
        .filter_map(|v| {
            let parsed: Vec<u64> = v.iter().filter_map(|s| s.parse::<u64>().ok()).collect();
            middle_value(&parsed)
        })
        .collect()
}

fn fix_ordering<'a>(update: &'a Vec<&'a str>, rules: &HashSet<&'a str>) -> Vec<&'a str> {
    let mut update_correct = update.clone();

    let mut i = 0;
    while i < update_correct.len() {
        let mut j = i + 1;
        while j < update_correct.len() {
            let candidate = format!("{}|{}", update_correct[j], update_correct[i]);
            if rules.contains(candidate.as_str()) {
                // Swap the elements to fix the order
                update_correct.swap(i, j);
                // Restart the loop to re-check the ordering
                i = 0;
                j = 0;
            }
            j += 1;
        }
        i += 1;
    }

    update_correct
}
