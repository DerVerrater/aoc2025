use std::fs;

use crate::{Error, Result};

pub fn part1() -> Result<i32> {
    let document = fs::read_to_string("./day3.txt").map_err(|_| Error::NoInputFile)?;
    let result = part1_impl(document.as_str())?;
    Ok(result)
}

pub fn part2() -> Result<i64> {
    let document = fs::read_to_string("./day3.txt").map_err(|_| Error::NoInputFile)?;
    let result = part2_impl(document.as_str())?;
    Ok(result)
}

fn part1_impl(input: &str) -> Result<i32> {
    let mut sum = 0;
    for line in input.split("\n") {
        let (slot1, idx) = find_digit(&line[..line.len() - 1])?;
        // println!("{slot1} --- {idx}");
        let (slot2, _idx) = find_digit(&line[(idx + 1)..])?;
        // println!("{slot2} --- {idx}");
        let number: String = [slot1, slot2].iter().collect();
        let number = number.parse::<i32>().map_err(|_| Error::Parsing)?;
        // println!("{number}");
        sum += number;
    }
    Ok(sum)
}

fn part2_impl(input: &str) -> Result<i64> {
    let mut sum = 0;
    for line in input.split("\n") {
        let mut start_idx = 0usize;
        let mut digits = Vec::<char>::new();
        for num_collected in 0..12 {
            let tail_buffer = 11 - num_collected;
            let (char, idx) = find_digit(&line[start_idx..(line.len() - tail_buffer)])?;
            start_idx += idx + 1;
            digits.push(char);
        }
        let number = digits
            .iter()
            .collect::<String>()
            .parse::<i64>()
            .map_err(|_| Error::Parsing)?;
        // println!("->> Got number {number}");
        sum += number;
    }
    Ok(sum)
}

// There are no guards for non-digit character input. Don't mess up.
fn find_digit(text: &str) -> Result<(char, usize)> {
    let mut highest_so_far = '0';
    let mut idx = 0;
    for (i, char) in text.chars().enumerate() {
        if highest_so_far < char {
            highest_so_far = char;
            idx = i;
        }
    }
    Ok((highest_so_far, idx))
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn p1() {
        assert_eq!(357, part1_impl(BATTERIES).unwrap())
    }

    #[test]
    fn p2() {
        assert_eq!(3_121_910_778_619, part2_impl(BATTERIES).unwrap())
    }

    const BATTERIES: &str = "987654321111111
811111111111119
234234234234278
818181911112111";
}
