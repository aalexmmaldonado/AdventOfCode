use std::env;

mod day1;
mod day10;
mod day11;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() {
    let args: Vec<String> = env::args().collect();

    let day_selection = args.get(1).map(String::as_str);
    match day_selection {
        Some("1") => {
            if let Err(e) = day1::run() {
                eprintln!("Error running Day 1: {}", e);
                std::process::exit(1);
            }
        }
        Some("2") => {
            if let Err(e) = day2::run() {
                eprintln!("Error running Day 2: {}", e);
                std::process::exit(1);
            }
        }
        Some("3") => {
            if let Err(e) = day3::run() {
                eprintln!("Error running Day 3: {}", e);
                std::process::exit(1);
            }
        }
        Some("4") => {
            if let Err(e) = day4::run() {
                eprintln!("Error running Day 4: {}", e);
                std::process::exit(1);
            }
        }
        Some("5") => {
            if let Err(e) = day5::run() {
                eprintln!("Error running Day 5: {}", e);
                std::process::exit(1);
            }
        }
        Some("6") => {
            if let Err(e) = day6::run() {
                eprintln!("Error running Day 6: {}", e);
                std::process::exit(1);
            }
        }
        Some("7") => {
            if let Err(e) = day7::run() {
                eprintln!("Error running Day 7: {}", e);
                std::process::exit(1);
            }
        }
        Some("8") => {
            if let Err(e) = day8::run() {
                eprintln!("Error running Day 8: {}", e);
                std::process::exit(1);
            }
        }
        Some("9") => {
            if let Err(e) = day9::run() {
                eprintln!("Error running Day 9: {}", e);
                std::process::exit(1);
            }
        }
        Some("10") => {
            if let Err(e) = day10::run() {
                eprintln!("Error running Day 10: {}", e);
                std::process::exit(1);
            }
        }
        Some("11") => {
            if let Err(e) = day11::run() {
                eprintln!("Error running Day 11: {}", e);
                std::process::exit(1);
            }
        }
        Some(_) => {
            println!("Invalid day!");
        }
        None => {
            println!("No day specified!");
        }
    }
}
