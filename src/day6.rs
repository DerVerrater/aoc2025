use std::fs;

use crate::{Error, Result};

// The sample data and real input have a different number of number rows and
// the parser isn't smart enough to deal with that. This selects between the
// two counts for `cargo test` and `cargo run`
#[cfg(test)]
const INPUT_ROW_COUNT: usize = 3;
#[cfg(not(test))]
const INPUT_ROW_COUNT: usize = 4;

pub fn part1() -> Result<usize> {
    let document = fs::read_to_string("./day6.txt").map_err(|_| Error::NoInputFile)?;
    part1_impl(document.as_str())
}

fn part1_impl(input: &str) -> Result<usize> {
    let width = input
        .split("\n")
        .take(1)
        .map(|line| line.split(" "))
        .count();

    // pre-allocate the list of problems
    let mut problems: Vec<Vec<usize>> = Vec::new();
    problems.reserve(width);

    let lines = input
        .split("\n")
        .map(parse_line)
        .filter(|item| item.is_ok())
        .map(|what| what.unwrap());

    let worksheet = lines.fold(Vec::<Vec<ItemType>>::new(), spread_items);
    let subtotals = worksheet.into_iter().map(|problem| {
        let operator = problem.iter().rev().next().unwrap();
        let answer = match operator {
            ItemType::Number(_num) => {
                panic!("Last element is not an operator. Algo error, panicking!")
            }
            ItemType::OpAdd => {
                let answer: usize = problem
                    .iter()
                    .take(INPUT_ROW_COUNT)
                    .map(|itemtype| {
                        if let ItemType::Number(num) = itemtype {
                            num
                        } else {
                            panic!(
                                "One of first four items was not a number. Algo error, panicking!"
                            );
                        }
                    })
                    .sum();
                answer
            }
            ItemType::OpMul => problem
                .iter()
                .take(INPUT_ROW_COUNT)
                .map(|itemtype| {
                    if let ItemType::Number(num) = itemtype {
                        *num
                    } else {
                        panic!(
                            "One of the first four items was not a number. Algo error, panicking!"
                        );
                    }
                })
                .reduce(|acc, val| acc * val)
                .unwrap(),
        };
        answer
    });
    Ok(subtotals.sum())
}

/// Parses a line, returning it as a list of [`ItemType`]s. If conversion
/// from [`core::str::parse`] fails, a [`crate::Error::Parsing`] error result
/// is returned, instead.
fn parse_line(input: &str) -> Result<Vec<ItemType>> {
    let parts = input
        .split_whitespace()
        .map(|col| col.trim())
        .map(|item| match item {
            "+" => Ok(ItemType::OpAdd),
            "*" => Ok(ItemType::OpMul),
            text => Ok(ItemType::Number(text.parse::<usize>()?)),
        });
    parts.collect()
}

/// Utility for spreading each column of a line into the sub-lists in the main
/// worksheet list.
fn spread_items(mut collection: Vec<Vec<ItemType>>, incoming: Vec<ItemType>) -> Vec<Vec<ItemType>> {
    for (idx, item) in incoming.into_iter().enumerate() {
        let slot = match collection.get_mut(idx) {
            Some(text) => Some(text),
            None => {
                collection.push(Vec::new());
                collection.get_mut(idx)
            },
        }.expect("Failed to retrived AND default-insert a vector for the collection. Something weird must have happened (like an OOM condition). Panicking!");
        slot.push(item);
    }
    collection
}

#[derive(Debug, PartialEq)]
enum ItemType {
    Number(usize),
    OpAdd,
    OpMul,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn p1() {
        assert_eq!(4277556, part1_impl(HOMEWORK).unwrap());
    }

    #[test]
    fn check_parseline() {
        let expected = vec![
            ItemType::Number(123),
            ItemType::Number(328),
            ItemType::Number(51),
            ItemType::Number(64),
        ];

        let input = HOMEWORK
            .lines()
            .next()
            .expect("Couldn't find one line in the input");
        let res = parse_line(input).expect("Parsing error occurred on a line");
        assert_eq!(res, expected);
    }

    #[test]
    fn check_spread() {
        let expected = vec![
            vec![
                ItemType::Number(123),
                ItemType::Number(45),
                ItemType::Number(6),
                ItemType::OpMul,
            ],
            vec![
                ItemType::Number(328),
                ItemType::Number(64),
                ItemType::Number(98),
                ItemType::OpAdd,
            ],
            vec![
                ItemType::Number(51),
                ItemType::Number(387),
                ItemType::Number(215),
                ItemType::OpMul,
            ],
            vec![
                ItemType::Number(64),
                ItemType::Number(23),
                ItemType::Number(314),
                ItemType::OpAdd,
            ],
        ];

        let lines = HOMEWORK
            .split("\n")
            .map(parse_line)
            .filter(|item| item.is_ok())
            .map(|what| what.unwrap());
        let worksheet = lines.fold(Vec::<Vec<ItemType>>::new(), spread_items);

        assert_eq!(worksheet, expected);
    }

    const HOMEWORK: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
}
