use std::env;

mod day1;

fn main() {
    let args: Vec<String> = env::args().collect();

    let day_selection = args.get(1).map(String::as_str);
    match day_selection {
        Some("1") => day1::run(),
        Some(_) => println!("Invalid day!"),
        None => println!("No day specified!"),
    }
}
