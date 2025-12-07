use crate::{Error, Result};
use std::{fs, ops::{RangeInclusive}, str::FromStr};

pub fn part1() -> Result<usize> {
    let document = fs::read_to_string("./day5.txt").map_err(|_| Error::NoInputFile)?;
    part1_impl(document.as_str())
}

fn part1_impl(input: &str) -> Result<usize> {
    let db = Database::<usize>::try_from(input)?;
    
    let mut contained = 0;
    for id in db.ids.iter() {
        if db.contains(id) {
            contained += 1;
        }
    }
    Ok(contained)
}

struct Database<Idx> {
    ranges: Vec<RangeInclusive<Idx>>,
    ids: Vec<Idx>,
}

impl<Idx> Database<Idx>
where
    Idx: PartialOrd + FromStr,
{
    fn try_from(txt: &str) -> Result<Self> {
        let mut parts = txt.split("\n\n");
        let text_ranges = parts.next().ok_or(Error::Parsing)?;
        let ids = parts.next().ok_or(Error::Parsing)?;

        let ranges: Vec<_> = text_ranges
            .split("\n")
            .map(|line| -> Result<RangeInclusive<Idx>> {
                let mut s = line.split("-");
                let start = s
                    .next()
                    .and_then(|s| s.parse::<Idx>().ok())
                    .ok_or(Error::Parsing)?;
                let end = s
                    .next()
                    .and_then(|s| s.parse::<Idx>().ok())
                    .ok_or(Error::Parsing)?;
                Ok(start..=end)
            })
            .collect::<Result<_>>()?;

        let ids: Vec<_> = ids
            .split("\n")
            .map(|line| line.parse::<Idx>().map_err(|_| Error::Parsing))
            .collect::<Result<_>>()?;

        Ok(Self {
            ranges: ranges,
            ids: ids,
        })
    }

    /// Searches each range for a positive hit, otherwise `false`.
    fn contains(&self, id: &Idx) -> bool {
        for range in self.ranges.iter() {
            if range.contains(id) {
                return true;
            } else {
                continue;
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn p1() {
        assert_eq!(3, part1_impl(DATABASE).unwrap());
    }

    const DATABASE: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
}
