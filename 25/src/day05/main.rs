use rayon::prelude::*;
use std::cmp;
use std::error::Error;
use std::fs;

pub fn run() -> Result<(), Box<dyn Error>> {
    println!("Running Day 05!");

    let input: String = fs::read_to_string("./src/day05/input.txt")?;

    let range_end = input.find("\n\n").unwrap_or(input.len());
    let fresh_block: &str = &input[..range_end];
    let avail_block: &str = &input[range_end..];

    let ranges: Vec<(i64, i64)> = parse_ranges(fresh_block);

    let fresh_p1: u64 = avail_block
        .par_lines()
        .map(|line| {
            let id = line.parse::<i64>().unwrap_or(-1);

            let is_valid = ranges.iter().any(|&(start, end)| id >= start && id <= end);

            if is_valid { 1 } else { 0 }
        })
        .sum();

    assert_eq!(511, fresh_p1);
    println!("Part 1: {:?}", fresh_p1);

    // Need to shrink the ranges to be unique.
    // If there is overlap, take the lowest lower bound and highest upper bound
    let merged_ranges = merge_ranges(ranges);

    let fresh_p2: i64 = merged_ranges.par_iter().map(|x| x.1 - x.0 + 1).sum();

    assert_eq!(350939902751909, fresh_p2);
    println!("Part 1: {:?}", fresh_p2);

    Ok(())
}

fn parse_ranges(input: &str) -> Vec<(i64, i64)> {
    input
        .lines()
        .filter(|line| line.contains('-'))
        .filter_map(|line| {
            line.split_once('-').and_then(|(s, e)| {
                let start = s.trim().parse::<i64>().ok()?;
                let end = e.trim().parse::<i64>().ok()?;
                Some((start, end))
            })
        })
        .collect()
}

fn merge_ranges(mut ranges: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    if ranges.is_empty() {
        return vec![];
    }

    ranges.sort_unstable_by_key(|r| r.0);

    let mut merged: Vec<(i64, i64)> = Vec::new();

    let (mut current_start, mut current_end) = ranges[0];

    for &(next_start, next_end) in ranges.iter().skip(1) {
        // If the next range starts before (or exactly when) the current one ends...
        if next_start <= current_end {
            // Extend the current end to the max of both.
            current_end = cmp::max(current_end, next_end);
        } else {
            // No overlap.
            merged.push((current_start, current_end));
            current_start = next_start;
            current_end = next_end;
        }
    }

    // push the final range
    merged.push((current_start, current_end));

    merged
}
