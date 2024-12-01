use std::fs;


pub fn run() {
    println!("Running Day 1!");
    let path_coords = "./src/day1/input.txt";
    match read_coords(path_coords) {
        Ok((mut coord1_vec, mut coord2_vec)) => {
            // Sort the vectors
            coord1_vec.sort();
            coord2_vec.sort();
            match compute_diff_sum(coord1_vec, coord2_vec){
                Ok(diff_sum) => println!("Difference sum is: {:?}", diff_sum),
                Err(e) => eprint!("Error computing difference sum: {}", e)
            }
        }
        Err(e) => eprintln!("Error reading coordinates: {}", e),
    }
}

fn compute_diff_sum(vec1: Vec<u32>, vec2: Vec<u32>) -> Result<i64, String> {
    // Check that vectors are same length
    if vec1.len() != vec2.len() {
        return Err(String::from("vec1 and vec2 are not same length"));
    } else {
        // We need to compute the difference between each number in order.
        let mut diff_sum: i64 = 0;
        for (a, b) in vec1.iter().zip(vec2.iter()) {
            diff_sum += (a).abs_diff(*b) as i64;
        }
        Ok(diff_sum)
    }
}


fn read_coords(path: &str) -> Result<(Vec<u32>, Vec<u32>), String> {
    println!("Reading coordinates from {}", path);
    let data = fs::read_to_string(path).expect("Unable to read file");
    let mut coord1_vec: Vec<u32> = vec![];
    let mut coord2_vec: Vec<u32> = vec![];

    for (line_no, line) in data.lines().enumerate() {
        // Skip empty lines
        if line.trim().is_empty() {
            continue;
        }

        // Split lines by whitespace
        let mut parts = line.split_whitespace();

        // Parse lines and push to Vec
        match (parts.next(), parts.next()) {
            (Some(coord1_str), Some(coord2_str)) => {
                // Parse each coordinate and append to Vec
                let coord1 = coord1_str.parse::<u32>().map_err(|e| {
                    format!("Error parsing coordinate 1 on line {}: {}", line_no + 1, e)
                })?;
                let coord2 = coord2_str.parse::<u32>().map_err(|e| {
                    format!("Error parsing coordinate 2 on line {}: {}", line_no + 1, e)
                })?;
                coord1_vec.push(coord1);
                coord2_vec.push(coord2);
            }
            _ => {
                return Err(format!(
                    "Invalid format on line {}: '{}'",
                    line_no + 1,
                    line
                ))
            }
        }
    }

    Ok((coord1_vec, coord2_vec))

}