use crate::{Error, Result};
use pcre2::bytes::Regex;
use std::fs;

pub fn part1() -> Result<i64> {
    let document = fs::read_to_string("./day2.txt").map_err(|_| Error::NoInputFile)?;
    let result = part1_impl(document.as_str())?;
    Ok(result)
}

fn part1_impl(input: &str) -> Result<i64> {
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
                    .and_then(|num| Some(num + 1))
                    .ok_or(Error::Parsing)?,
            ))
        })
        .collect::<Result<_>>()?;

    let reg = Regex::new(r"^(\d+)\1$").unwrap();
    let mut sum: i64 = 0;
    for (start, stop) in ranges {
        // scan range looking for repeated patterns
        for idx in start as i64..stop as i64 {
            let stringified = idx.to_string();
            if reg.is_match(stringified.as_bytes()).unwrap() {
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
        assert_eq!(1227775554, part1_impl(PRODUCT_RANGES).unwrap());
    }

    const PRODUCT_RANGES: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
}
