use ndarray::Array2;
use std::error::Error;
use std::fs;

pub fn run() -> Result<(), Box<dyn Error>> {
    println!("Running Day 04!");

    let input: String = fs::read_to_string("./src/day04/input.txt")?;
    let lines: Vec<&str> = input.lines().filter(|line| !line.is_empty()).collect();

    let rows = lines.len();
    let cols = lines.first().map_or(0, |l| l.len());

    let data: Vec<i64> = lines
        .into_iter()
        .flat_map(|line| line.chars())
        .map(|c| match c {
            '.' => 0,
            '@' => 1,
            _ => panic!("Unexpected character: {}", c),
        })
        .collect();

    let mut rolls = Array2::from_shape_vec((rows, cols), data)
        .expect("Error creating array from shape and data");

    // Basically just creating another array where the cell value is the number
    // of adjacent roll.
    let mut neighbor_counts = Array2::<i64>::zeros(rolls.raw_dim());

    let mut p1_count = 0;
    for ((r, c), cell) in neighbor_counts.indexed_iter_mut() {
        if rolls[[r, c]] == 0 {
            continue;
        }

        for dr in -1..=1 {
            for dc in -1..=1 {
                // Skip current roll
                if dr == 0 && dc == 0 {
                    continue;
                }

                let nr = r as isize + dr;
                let nc = c as isize + dc;

                if rolls[[r, c]] == 0 {
                    continue;
                }

                if nr >= 0
                    && nc >= 0
                    && let Some(&val) = rolls.get((nr as usize, nc as usize))
                {
                    *cell += val;
                }
            }
        }
        if *cell >= 0 && *cell < 4 {
            p1_count += 1
        };
    }

    assert_eq!(1523, p1_count);
    println!("Part 1: {:?}", p1_count);

    let mut p2_count = 0;

    loop {
        let removed_this_round = process_removals(&mut rolls);

        if removed_this_round == 0 {
            break;
        }

        p2_count += removed_this_round;
    }

    assert_eq!(9290, p2_count);
    println!("Part 2: {:?}", p2_count);

    Ok(())
}

/// Scans the grid, identifies accessible rolls (neighbors < 4),
/// removes them (sets to 0), and returns the count of removed items.
fn process_removals(rolls: &mut Array2<i64>) -> i64 {
    let (rows, cols) = rolls.dim();
    let mut to_remove = vec![];

    for r in 0..rows {
        for c in 0..cols {
            if rolls[[r, c]] == 0 {
                continue;
            }

            let mut neighbors = 0;

            // Check 8 neighbors
            for dr in -1..=1 {
                for dc in -1..=1 {
                    if dr == 0 && dc == 0 {
                        continue;
                    }

                    let nr = r as isize + dr;
                    let nc = c as isize + dc;

                    if nr >= 0
                        && nc >= 0
                        && let Some(&val) = rolls.get((nr as usize, nc as usize))
                    {
                        neighbors += val;
                    }
                }
            }

            if neighbors < 4 {
                to_remove.push((r, c));
            }
        }
    }

    let count = to_remove.len() as i64;

    // Apply the removals
    for (r, c) in to_remove {
        rolls[[r, c]] = 0;
    }

    count
}
