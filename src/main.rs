use std::{env, fmt::Display};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

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

                let p2 = match day1::part2() {
                    Ok(num) => num.to_string(),
                    Err(err) => err.to_string(),
                };
                println!("Day 1, Part 2: Result {}", p2);
            }
            "day2" | "2" | "d2" => {
                let p1 = match day2::part1() {
                    Ok(num) => num.to_string(),
                    Err(err) => err.to_string(),
                };
                println!("Day 2, Part 1: Result {}", p1);

                let p2 = match day2::part2() {
                    Ok(num) => num.to_string(),
                    Err(err) => err.to_string(),
                };
                println!("Day 2, Part 2: Result {}", p2);
            }
            "day3" | "3" | "d3" => {
                let p1 = match day3::part1() {
                    Ok(num) => num.to_string(),
                    Err(err) => err.to_string(),
                };
                println!("Day 3, Part 1: Result {}", p1);

                let p2 = match day3::part2() {
                    Ok(num) => num.to_string(),
                    Err(err) => err.to_string(),
                };
                println!("Day 3, Part 2: Result {}", p2);
            }
            "day4" | "4" | "d4" => {
                let p1 = match day4::part1() {
                    Ok(num) => num.to_string(),
                    Err(err) => err.to_string(),
                };
                println!("Day 4, Part 1: Result {}", p1);

                let p2 = match day4::part2() {
                    Ok(num) => num.to_string(),
                    Err(err) => err.to_string(),
                };
                println!("Day 4, Part 2: Result {}", p2);
            }
            "day5" | "5" | "d5" => {
                let p1 = match day5::part1() {
                    Ok(num) => num.to_string(),
                    Err(err) => err.to_string(),
                };
                println!("Day 5, Part 1: Result {}", p1);
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
    InvalidInput,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                Error::Parsing => String::from("Error::Parsing"),
                Error::NoInputFile => String::from("Error::NoInputFile"),
                Error::InvalidInput => String::from("Error::InvalidInput"),
            }
        )
    }
}

pub type Result<T> = std::result::Result<T, crate::Error>;
