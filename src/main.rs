use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Some(day) = args.get(1) {
        match day.as_str() {
            "day1" | "1" | "d1" => todo!("Make day1 module, solve puzzle."),
            _ => {
                eprintln!("Day is not done yet, or is an unrecognized value.");
                std::process::exit(1);
            }
        }
    } else {
        eprintln!("aoc2025: Not enough arguments. Please name a day.")
    }
}
