use std::env;

mod day00;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;

fn main() {
    let args: Vec<String> = env::args().collect();

    let day_selection = args.get(1).map(String::as_str);
    match day_selection {
        Some("00") => {
            if let Err(e) = day00::run() {
                eprintln!("Error running Day 00: {}", e);
                std::process::exit(1);
            }
        }
        Some("01") => {
            if let Err(e) = day01::run() {
                eprintln!("Error running Day 01: {}", e);
                std::process::exit(1);
            }
        }
        Some("02") => {
            if let Err(e) = day02::run() {
                eprintln!("Error running Day 02: {}", e);
                std::process::exit(1);
            }
        }
        Some("03") => {
            if let Err(e) = day03::run() {
                eprintln!("Error running Day 03: {}", e);
                std::process::exit(1);
            }
        }
        Some("04") => {
            if let Err(e) = day04::run() {
                eprintln!("Error running Day 04: {}", e);
                std::process::exit(1);
            }
        }
        Some("05") => {
            if let Err(e) = day05::run() {
                eprintln!("Error running Day 05: {}", e);
                std::process::exit(1);
            }
        }
        Some("06") => {
            if let Err(e) = day06::run() {
                eprintln!("Error running Day 06: {}", e);
                std::process::exit(1);
            }
        }
        Some("07") => {
            if let Err(e) = day07::run() {
                eprintln!("Error running Day 07: {}", e);
                std::process::exit(1);
            }
        }
        Some("08") => {
            if let Err(e) = day08::run() {
                eprintln!("Error running Day 08: {}", e);
                std::process::exit(1);
            }
        }
        Some("09") => {
            if let Err(e) = day09::run() {
                eprintln!("Error running Day 09: {}", e);
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
        Some("12") => {
            if let Err(e) = day12::run() {
                eprintln!("Error running Day 12: {}", e);
                std::process::exit(1);
            }
        }
        Some(_) => {
            println!("Invalid day!");
        }
        _ => {
            println!("No day specified!");
        }
    }
}
