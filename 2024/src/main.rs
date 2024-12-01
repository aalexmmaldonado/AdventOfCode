use std::env;

mod day1;
mod day2;

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
        Some(_) => {
            println!("Invalid day!");
        }
        None => {
            println!("No day specified!");
        }
    }
}
