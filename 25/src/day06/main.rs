use std::error::Error;
use std::fs;

type Problem = (Vec<u128>, char);

#[derive(PartialEq, Eq)]
enum NumberOrientation {
    Horizontal,
    Vertical,
}

pub fn run() -> Result<(), Box<dyn Error>> {
    println!("Running Day 06!");

    let input: String = fs::read_to_string("./src/day06/input.txt")?;

    let problems_p1: Vec<Problem> = get_problems(&input, NumberOrientation::Horizontal);

    let sum_p1: u128 = problems_p1
        .iter()
        .map(|(nums, op)| match op {
            '+' => nums.iter().sum::<u128>(),
            '*' => nums.iter().product::<u128>(),
            _ => panic!("Unknown operator encountered: {}", op),
        })
        .sum();

    assert_eq!(sum_p1, 3785892992137);
    println!("Part 1: {}", sum_p1);

    let problems_p2: Vec<Problem> = get_problems(&input, NumberOrientation::Vertical);

    let sum_p2: u128 = problems_p2
        .iter()
        .map(|(nums, op)| match op {
            '+' => nums.iter().sum::<u128>(),
            '*' => nums.iter().product::<u128>(),
            _ => panic!("Unknown operator encountered: {}", op),
        })
        .sum();

    assert_eq!(sum_p1, 7669802156452);
    println!("Part 2: {}", sum_p2);

    Ok(())
}

fn get_problems(content: &str, orientation: NumberOrientation) -> Vec<Problem> {
    let lines: Vec<&str> = content.lines().filter(|l| !l.is_empty()).collect();

    let max_width: usize = lines.iter().map(|l| l.len()).max().unwrap_or(0);

    let mut parsed_problems: Vec<Problem> = Vec::new();
    let mut col_idx: usize = 0;

    while col_idx < max_width {
        // Every problem is separated by a whole column of b' '
        // We can use this to get a vertical slice of str for the problem.
        let is_separator: bool = lines
            .iter()
            .all(|line| line.as_bytes().get(col_idx).is_none_or(|&b| b == b' '));
        if is_separator {
            col_idx += 1;
            continue;
        }

        // If we are here, we are at the start of a problem block.
        // Find where this block ends (the next separator).
        let start: usize = col_idx;
        while col_idx < max_width {
            let is_gap: bool = lines
                .iter()
                .all(|line| line.as_bytes().get(col_idx).is_none_or(|&b| b == b' '));
            if is_gap {
                break;
            }
            col_idx += 1;
        }
        let end: usize = col_idx;

        // Parse this specific block (from column `start` to `end`)
        let mut numbers = Vec::new();
        let mut operator: char = '+';

        if orientation == NumberOrientation::Horizontal {
            for (row_idx, line) in lines.iter().enumerate() {
                if let Some(chunk) = line.get(start..end) {
                    let text: &str = chunk.trim();
                    if text.is_empty() {
                        continue;
                    }

                    if row_idx == lines.len() - 1 {
                        operator = text.chars().next().unwrap();
                    } else if let Ok(num) = text.parse::<u128>() {
                        numbers.push(num);
                    }
                }
            }
        } else if orientation == NumberOrientation::Vertical {
            // Extract Operator from the last line of this block
            if let Some(last_line) = lines.last()
                && let Some(chunk) = last_line.get(start..end)
                && let Some(op_char) = chunk.trim().chars().next()
            {
                operator = op_char;
            }

            // Build numbers column-by-column
            for c in start..end {
                let mut digit_str = String::new();

                for line in &lines[..lines.len() - 1] {
                    if let Some(&byte) = line.as_bytes().get(c) {
                        let ch: char = byte as char;
                        if ch.is_ascii_digit() {
                            digit_str.push(ch);
                        }
                    }
                }

                if !digit_str.is_empty()
                    && let Ok(num) = digit_str.parse::<u128>()
                {
                    numbers.push(num);
                }
            }
        }

        parsed_problems.push((numbers, operator));
    }

    parsed_problems
}
