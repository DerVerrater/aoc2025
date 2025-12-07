use crate::{Error, Result};
use pcre2::bytes::Regex;
use std::fs;

pub fn part1() -> Result<i64> {
    let document = fs::read_to_string("./day2.txt").map_err(|_| Error::NoInputFile)?;
    let result = part1_impl(document.as_str())?;
    Ok(result)
}

pub fn part2() -> Result<i64> {
    let document = fs::read_to_string("./day2.txt").map_err(|_| Error::NoInputFile)?;
    let result = part2_impl(document.as_str())?;
    Ok(result)
}

fn part1_impl(input: &str) -> Result<i64> {
    let regex = Regex::new(r"^(\d+)\1$").unwrap();
    solver_impl(input, regex)
}

/// Part 2 is just part 1, but with the capture group repeating several times.
fn part2_impl(input: &str) -> Result<i64> {
    let regex = Regex::new(r"^(\d+)\1+$").unwrap();
    solver_impl(input, regex)
}

fn solver_impl(input: &str, regex: Regex) -> Result<i64> {
    let ranges: Vec<(i64, i64)> = input
        .split(",")
        .map(|range| -> Result<(_, _)> {
            let mut i = range.split("-");
            Ok((
                i.next()
                    .and_then(|text| text.parse::<i64>().ok())
                    .ok_or(Error::Parsing)?,
                i.next()
                    .and_then(|text| text.parse::<i64>().ok())
                    .map(|num| num + 1)
                    .ok_or(Error::Parsing)?,
            ))
        })
        .collect::<Result<_>>()?;
    let mut sum: i64 = 0;
    for (start, stop) in ranges {
        // scan range looking for repeated patterns
        for idx in start..stop {
            let stringified = idx.to_string();
            if regex.is_match(stringified.as_bytes()).unwrap() {
                sum += idx;
            }
        }
    }
    Ok(sum)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn p1() {
        assert_eq!(1_227_775_554, part1_impl(PRODUCT_RANGES).unwrap());
    }

    #[test]
    fn p2() {
        assert_eq!(4_174_379_265, part2_impl(PRODUCT_RANGES).unwrap());
    }
    const PRODUCT_RANGES: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
}
