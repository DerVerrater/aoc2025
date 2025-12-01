use std::fs;

use crate::Error;

/// Loads the input sequence from file and compute the answer.
pub fn part1() -> Result<u16, Error> {
    let document = fs::read_to_string("./day1.txt").map_err(|_| Error::NoInputFile)?;
    let result = part1_impl(document.as_str())?;
    Ok(result)
}

/// This is the actual implementation for part 1
///
/// It is private so that main.rs doesn't need to know this thing secretly
/// requires an input argument.
fn part1_impl(puzzle_sequence: &str) -> Result<u16, Error> {
    let mut dial = Dial::new();
    for line in puzzle_sequence.split('\n') {
        if line.is_empty() {
            break;
        };
        dial.rotate(parse_operation(line)?);
    }
    Ok(dial.times_at_zero)
}

struct Dial {
    current: i16,
    times_at_zero: u16,
}

impl Dial {
    fn new() -> Self {
        Self {
            current: 50,
            times_at_zero: 0,
        }
    }

    fn rotate(&mut self, amount: i16) {
        self.current = (self.current + amount) % 100;
        println!(
            "The dial is rotated {} to point at {}",
            amount, self.current
        );
        if self.current == 0 {
            self.times_at_zero += 1;
        }
    }
}

fn parse_operation(text: &str) -> Result<i16, Error> {
    let dir = &text[..1];
    let sign = match dir {
        "L" => Ok(-1i16),
        "R" => Ok(1i16),
        _ => Err(Error::Parsing),
    }?;
    let distance = &text[1..];
    let distance = distance.parse::<i16>().map_err(|_| Error::Parsing)?;
    Ok(sign * distance)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn p1() {
        let expected = 3;
        let computed = part1_impl(INSTRUCTIONS).unwrap();
        assert_eq!(computed, expected);
    }

    const INSTRUCTIONS: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
}
