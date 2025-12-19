use std::error::Error;
use std::fs;

pub fn run() -> Result<(), Box<dyn Error>> {
    println!("Running Day 01!");

    // Want to avoid heap-allocated string. We will use memory maps instead.
    let input = fs::read("./src/day01/input.txt")?;

    let (p1, p2) = solve(&input);
    assert_eq!(p1, 1040);
    println!("Part 1: {}", p1);
    assert_eq!(p2, 6027);
    println!("Part 2: {}", p2);
    Ok(())
}

fn solve(input: &[u8]) -> (u64, u64) {
    let mut p1_count: u64 = 0;
    let mut p2_count: u64 = 0;

    let mut current_val: i64 = 50;
    let range_size: i64 = 100;

    let mut i = 0;
    let len = input.len();

    while i < len {
        let b = input[i];

        // If it's not 'L' or 'R', skip.
        if b < b'A' {
            i += 1;
            continue;
        }

        // Branchless Direction Parsing
        // 'L' is 0x4C (0100 1100) -> bit 1 is 0
        // 'R' is 0x52 (0101 0010) -> bit 1 is 1
        // We isolate bit 1:
        // L: (0x4C & 2) = 0. We want -1. -> (0 - 1) = -1
        // R: (0x52 & 2) = 2. We want  1. -> (2 - 1) = 1
        let sign = (b & 2) as i64 - 1;
        i += 1;

        let mut mag: i64 = 0;
        loop {
            if i >= len {
                break;
            }

            // "trick": wrapping_sub maps '0'..'9' to 0..9.
            // Any other char becomes a large u8 > 9.
            // Eliminates one branch (checking >= '0' AND <= '9').
            let digit = input[i].wrapping_sub(b'0');
            if digit > 9 {
                break;
            }

            mag = (mag << 3) + (mag << 1) + digit as i64;
            i += 1;
        }

        let step = mag * sign;

        let start = current_val;
        let end = start + step;

        current_val = end.rem_euclid(range_size);

        // Part 1
        if current_val == 0 {
            p1_count += 1;
            p2_count += 1;
        }

        // Part 2
        if step > 0 {
            let passes = (end - 1).div_euclid(range_size) - start.div_euclid(range_size);
            if passes > 0 {
                p2_count += passes as u64;
            }
        } else {
            let passes = (start - 1).div_euclid(range_size) - end.div_euclid(range_size);
            if passes > 0 {
                p2_count += passes as u64;
            }
        }
    }

    (p1_count, p2_count)
}
