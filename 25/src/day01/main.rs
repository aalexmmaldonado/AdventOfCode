use std::error::Error;
use std::fs;

pub fn run() -> Result<(), Box<dyn Error>> {
    println!("Running Day 01!");

    let input: String = fs::read_to_string("./src/day01/input.txt")?;
    let turns: Vec<Turn> = parse_instructions(&input)?;

    // Part 1
    let zeros_count: u64 = run_turns(&turns, 50, false);
    assert_eq!(zeros_count, 1040);
    println!("Number of zero ticks: {}", zeros_count);

    // Part 2
    let zeros_count_with_passes: u64 = run_turns(&turns, 50, true);
    println!(
        "Number of zero ticks with passes: {}",
        zeros_count_with_passes
    );
    Ok(())
}

#[derive(Debug)]
struct Turn {
    step: i64,
}

fn parse_instructions(contents: &str) -> Result<Vec<Turn>, Box<dyn Error>> {
    let mut turns: Vec<Turn> = vec![];
    let mut step: u64;

    for line in contents.lines() {
        step = line[1..].parse::<u64>()?;

        let dir = line.chars().next().unwrap();
        let turn = Turn {
            step: {
                if dir == 'L' {
                    step as i64 * -1
                } else {
                    step as i64
                }
            },
        };

        turns.push(turn);
    }
    Ok(turns)
}

fn perform_turn(value: &i64, turn: &Turn, track_zero_passes: bool) -> (i64, i64) {
    let range_size: i64 = 100;

    let start = *value;
    let end = start + turn.step;
    let new_value = end.rem_euclid(range_size);

    let mut n_zero_passes = 0;

    if track_zero_passes {
        if turn.step > 0 {
            // MOVING RIGHT (Positive)
            // We want to know how many multiples of 100 are in the range (start, end).
            // We exclude 'end' because the main loop checks if we land on 0.
            // Formula: floor((end - 1) / 100) - floor(start / 100)
            n_zero_passes = (end - 1).div_euclid(range_size) - start.div_euclid(range_size);
        } else {
            // MOVING LEFT (Negative)
            // We want to know how many multiples of 100 are in the range (end, start).
            // We exclude 'end' here as well.
            // Formula: floor((start - 1) / 100) - floor(end / 100)
            n_zero_passes = (start - 1).div_euclid(range_size) - end.div_euclid(range_size);
        }
    }

    (new_value, n_zero_passes)
}

fn run_turns(turns: &Vec<Turn>, start: i64, track_zero_passes: bool) -> u64 {
    let mut current_value: i64 = start;
    let mut zeros_count: u64 = 0;
    let mut n_zero_passes: i64;

    for turn in turns {
        (current_value, n_zero_passes) = perform_turn(&current_value, turn, track_zero_passes);
        if current_value == 0 {
            zeros_count += 1;
        }
        if n_zero_passes > 0 {
            zeros_count += n_zero_passes as u64
        }
    }

    zeros_count
}
