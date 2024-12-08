use std::error::Error;
use std::fs;

pub fn run() -> Result<(), Box<dyn Error>> {
    println!("Running Day 7!");

    let path_input = "./src/day7/input.txt";
    let contents = read_file(path_input)?;

    let all_ops = vec!['+', '*'];
    let total_sum = process_lines(&contents, &all_ops)?;
    assert_eq!(total_sum, 2299996598890);
    println!("Total calibration result: {}", total_sum);

    let all_ops = vec!['+', '*', '|'];
    let total_sum_all = process_lines(&contents, &all_ops)?;
    assert_eq!(total_sum_all, 362646859298554);
    println!("Total calibration result (using '||'): {}", total_sum_all);

    Ok(())
}

fn read_file(path: &str) -> Result<String, Box<dyn Error>> {
    println!("Reading file from {}", path);
    let contents =
        fs::read_to_string(path).map_err(|e| format!("Unable to read file {}: {}", path, e))?;
    Ok(contents)
}

fn process_lines(contents: &str, ops: &[char]) -> Result<i64, Box<dyn Error>> {
    let mut total_sum = 0;

    for line in contents.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() != 2 {
            // skip malformed lines
            continue;
        }

        let target_value_str = parts[0].trim();
        let target_value: i64 = target_value_str.parse()?;

        let nums_str = parts[1].trim();
        let nums: Vec<i64> = nums_str
            .split_whitespace()
            .map(|n| n.parse::<i64>())
            .collect::<Result<Vec<i64>, _>>()
            .map_err(|e| format!("Parsing error on line '{}': {}", line, e))?;

        if can_produce_target(&nums, target_value, ops) {
            total_sum += target_value;
        }
    }

    Ok(total_sum)
}

fn can_produce_target(nums: &[i64], target: i64, ops: &[char]) -> bool {
    if nums.is_empty() {
        return false;
    }
    if nums.len() == 1 {
        return nums[0] == target;
    }

    let num_operators = nums.len() - 1;
    let num_ops = ops.len() as u64;

    let total_combinations = num_ops.pow(num_operators as u32);

    for mut mask in 0..total_combinations {
        let mut value = nums[0];

        for i in 0..num_operators {
            let op_index = (mask % num_ops as u64) as usize;
            mask /= num_ops;

            let next_num = nums[i + 1];
            let op = ops[op_index];

            value = match op {
                '+' => value + next_num,
                '*' => value * next_num,
                '|' => concat_numbers(value, next_num),
                _ => unreachable!(),
            };
        }

        if value == target {
            return true;
        }
    }

    false
}

fn concat_numbers(a: i64, b: i64) -> i64 {
    // Concatenate digits of b onto a.
    if b == 0 {
        return a * 10;
    }
    let digits = num_digits(b);
    a * 10_i64.pow(digits) + b
}

fn num_digits(mut x: i64) -> u32 {
    if x == 0 {
        return 1;
    }
    let mut count = 0;
    while x > 0 {
        x /= 10;
        count += 1;
    }
    count
}
