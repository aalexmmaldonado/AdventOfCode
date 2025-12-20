use std::error::Error;
use std::fs;

pub fn run() -> Result<(), Box<dyn Error>> {
    println!("Running Day 02!");
    let input: String = fs::read_to_string("./src/day02/input.txt")?;

    let mut p1_sum: i64 = 0;
    let mut p2_sum: i64 = 0;
    for prod_ids in input.as_str().lines().flat_map(|line| line.split(',')) {
        let id_range: Vec<i64> = prod_ids
            .split('-')
            .map(|x| x.parse::<i64>().unwrap())
            .collect();

        for n in id_range[0]..=id_range[1] {
            let num_digits = n.ilog10() + 1;

            // Only check strictly even digit counts (e.g. 123123 -> 6 digits)
            if num_digits % 2 == 0 {
                let divisor = 10_i64.pow(num_digits / 2);
                let left = n / divisor;
                let right = n % divisor;

                if left == right {
                    p1_sum += n;
                }
            }

            // Check possible repetition counts 'k' (parts)
            // For 6 digits, we check splitting into 2, 3, or 6 parts.
            for k in 2..=num_digits {
                // Can we split digits into k equal chunks?
                if num_digits % k == 0 {
                    let chunk_len = num_digits / k;

                    // Extract the pattern
                    // We shift right by the remaining digits to isolate the top chunk
                    let pattern_divisor = 10_i64.pow(num_digits - chunk_len);
                    let pattern = n / pattern_divisor;

                    // Reconstruct the number using the pattern of length k
                    // repeated k times.
                    let mut candidate = 0;
                    let shift = 10_i64.pow(chunk_len);

                    for _ in 0..k {
                        candidate = candidate * shift + pattern;
                    }

                    if candidate == n {
                        p2_sum += n;
                        break; // Stop checking divisors once we find first match
                    }
                }
            }
        }
    }

    assert_eq!(37314786486, p1_sum);
    println!("Part 1: {:?}", p1_sum);
    assert_eq!(47477053982, p2_sum);
    println!("Part 2: {:?}", p2_sum);

    Ok(())
}
