use std::env;

mod day00;
mod day01;

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
        Some(_) => {
            println!("Invalid day!");
        }
        None => {
            println!("No day specified!");
        }
    }
}
