use std::{env, fmt::Display};

mod day1;

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Some(day) = args.get(1) {
        match day.as_str() {
            "day1" | "1" | "d1" => {
                let p1 = match day1::part1() {
                    Ok(num) => num.to_string(),
                    Err(err) => err.to_string(),
                };
                println!("Day 1, Part 1: Result {}", p1);
            }
            _ => {
                eprintln!("Day is not done yet, or is an unrecognized value.");
                std::process::exit(1);
            }
        }
    } else {
        eprintln!("aoc2025: Not enough arguments. Please name a day.")
    }
}

#[derive(Debug)]
pub enum Error {
    Parsing,
    NoInputFile,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                Error::Parsing => String::from("Parsing"),
                Error::NoInputFile => String::from("NoInputFile"),
            }
        )
    }
}
