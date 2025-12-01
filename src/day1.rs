use std::fs;

use crate::Error;

/// Loads the input sequence from file and compute the answer.
pub fn part1() -> Result<u16, Error> {
    let document = fs::read_to_string("./day1.txt").map_err(|_| Error::NoInputFile)?;
    let result = part1_impl(document.as_str())?;
    Ok(result)
}

pub fn part2() -> Result<u16, Error> {
    let document = fs::read_to_string("./day1.txt").map_err(|_| Error::NoInputFile)?;
    let result = part1_impl2(document.as_str())?;
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

/// Part 2 implementation
///
/// Because I'm too lazy to set up a dependency-injection thing to select
/// between the two "rotate" methods.
fn part1_impl2(puzzle_sequence: &str) -> Result<u16, Error> {
    let mut dial = Dial::new();
    for line in puzzle_sequence.split('\n') {
        if line.is_empty() {
            break;
        };
        dial.rotate2(parse_operation(line)?);
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

    fn rotate2(&mut self, amount: i16) {
        println!("Dial [{}], turn {}", self.current, amount);
        // Compute the next position
        let pos = ((self.current + amount) % 100) as i8;
        let next = {
            // Modulo position back into (-100, 100) range
            // If negative, take it's compliment as final value.
            if pos.is_negative() {
                // now it's positive, between 0 and 100. Convert to u8
                (100 + pos) as u8
            } else {
                // it's already positive, between 0 and 100. Convert to u8
                pos as u8
            }
        };
        println!(" ->> Next [{}], ({})", next, pos);
        
        // Count one towards times_at_zero if
        // 1. next pos *is* 0
        // 2. going up but next < current (happens after overflow & wrap)
        // 3. going down but next > current (happens after underflow & wrap)
        if (next == 0) {
            self.times_at_zero += 1;
            println!(" ->> Landed on zero, +1");
        // Guard against starting from zero. If we're already there, wrapping
        // doesn't count because we didn't touch it *again*. We were already
        // touching it, so counting the wrap would be double counting the touch.
        } else if self.current != 0 && (amount.is_negative() && ((self.current as u8) < next)) {
                self.times_at_zero += 1;
                println!(" ->> Rolled under, +1");
        } else if (amount.is_positive() && ((self.current as u8) > next)) {
                self.times_at_zero += 1;
                println!(" ->> Rolled over, +1");
        } else {
                println!(" --> No zero-cross. Continue");
        }

        // Count how many whole wraps might have happened (e.g.: `amount=250`)
        let loops = (amount / 100).unsigned_abs();
        if loops > 0 {
            println!(" ->> Whole wraps: +{}", loops);
        }
        self.times_at_zero += loops as u16;

        // Finally, update current pos
        self.current = next as i16;
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

    #[test]
    fn p2() {
        let expected = 6;
        let computed = part1_impl2(INSTRUCTIONS).unwrap();
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
