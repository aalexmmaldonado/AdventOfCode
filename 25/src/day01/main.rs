use std::error::Error;
use std::fs;

pub fn run() -> Result<(), Box<dyn Error>> {
    println!("Running Day 01!");

    let input: String = fs::read_to_string("./src/day01/input.txt")?;
    let turns: Vec<Turn> = parse_instructions(&input)?;
    let zeros_count: u64 = run_turns(&turns, 50);
    println!("Number of zeros: {}", zeros_count);
    Ok(())
}

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

fn perform_turn(value: &i64, turn: &Turn) -> i64 {
    let range_size: i64 = 100;

    // This calculates (value + step) modulo 100
    (value + turn.step).rem_euclid(range_size)
}

fn run_turns(turns: &Vec<Turn>, start: i64) -> u64 {
    let mut current_value: i64 = start;
    let mut zeros_count: u64 = 0;

    for turn in turns {
        current_value = perform_turn(&current_value, turn);
        if current_value == 0 {
            zeros_count += 1;
        }
    }

    zeros_count
}
