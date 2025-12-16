use std::error::Error;
use std::fs;

pub fn run() -> Result<(), Box<dyn Error>> {
    println!("Running Day 3!");

    let path_instructions = "./src/day3/input.txt";
    let contents = read_file(&path_instructions)?;
    let mults = find_mults(&contents);
    let mults_sum = get_mult_sums(&mults);
    assert_eq!(mults_sum, 178886550);
    println!("Parsed mult sum: {:?}", mults_sum);

    let mults_sum2 = parse_instructions(&contents);
    assert_eq!(mults_sum2, 87163705);
    println!("Instruction mult sum: {:?}", mults_sum2);

    Ok(())
}

fn read_file(path: &str) -> Result<String, Box<dyn Error>> {
    println!("Reading file from {}", path);
    let contents =
        fs::read_to_string(path).map_err(|e| format!("Unable to read file {}: {}", path, e))?;

    Ok(contents)
}

fn parse_instructions(contents: &str) -> u64 {
    let mut mul_enabled = true;
    let mut sum = 0;
    let mut i = 0;
    let content_len = contents.len();

    while i < content_len {
        if contents[i..].starts_with("do()") {
            mul_enabled = true;
            i += "do()".len();
        } else if contents[i..].starts_with("don't()") {
            mul_enabled = false;
            i += "don't()".len();
        } else if contents[i..].starts_with("mul(") {
            let start = i + "mul(".len();
            let mut end = start;

            // Find the closing parenthesis of the mul instruction
            while end < content_len && contents.as_bytes()[end] != b')' {
                end += 1;
            }
            if (end - start) > 7 {
                i += 1;
                continue;
            }

            if end < content_len {
                // Found closing ')', attempt to parse arguments
                let arg_str = &contents[start..end];
                i = end + 1; // Move past ')'

                // Check for valid mul instruction
                if mul_enabled && arg_str.contains(',') {
                    let args: Vec<&str> = arg_str.split(',').collect();
                    if args.len() == 2 {
                        if let (Ok(a), Ok(b)) =
                            (args[0].trim().parse::<u64>(), args[1].trim().parse::<u64>())
                        {
                            if args[0].trim().len() <= 3 && args[1].trim().len() <= 3 {
                                sum += a * b;
                            }
                        }
                    }
                }
            } else {
                break;
            }
        } else {
            i += 1;
        }
    }

    sum
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
