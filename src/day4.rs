use std::fs;

use itertools::Itertools;

use crate::{Error, Result};

pub fn part1() -> Result<i32> {
    let document = fs::read_to_string("./day4.txt").map_err(|_| Error::NoInputFile)?;
    part1_impl(document.as_str())
}

fn part1_impl(input: &str) -> Result<i32> {
    let grid = Grid::try_new(input)?;
    let mut accessible_rolls = 0;
    for (y, x) in (0..grid.height).cartesian_product(0..grid.width) {
        // Only do the neighbor scan if we're on a paper roll.
        if let Some(Tile::Paper) = grid.tile_at(x, y) {
            // println!(" >>> Accessibility for: ({x}, {y})");
            // select kernel ranges based on whether or not we're up against the
            // zero line. -1 to 1 normally, 0 to 1 on lower border.
            let w_range = if x > 0 { (x - 1)..(x + 2) } else { x..(x + 2) };
            let h_range = if y > 0 { (y - 1)..(y + 2) } else { y..(y + 2) };
            let kernel = h_range.cartesian_product(w_range);
            // println!("Kernel: {kernel:?}");
            let mut paper_neighbors = 0;
            let mut empty_neighbors = 0;
            for (v, u) in kernel {
                // skip the center spot (u,v is on x,y)
                if u == x && v == y {
                    // println!(" ->> On kernel center, skipping this iteration.");
                    continue;
                }
                if let Some(tile) = grid.tile_at(u, v) {
                    // println!(" ->> Checking tile ({u}, {v}): {tile:?}");
                    match tile {
                        Tile::Empty => {
                            empty_neighbors += 1;
                        }
                        Tile::Paper => {
                            paper_neighbors += 1;
                        }
                    }
                } else {
                    // eprintln!(
                    //     "Tile at ({u}, {v}) was None. Manually verify this is an out-of-bounds access"
                    // );
                }
            }

            if paper_neighbors < 4 {
                // this paper roll can be accessed
                accessible_rolls += 1;
            } /* else, cannot be. Do nothing */
        }
    }
    Ok(accessible_rolls)
}

struct Grid {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
}

impl Grid {
    fn try_new(input: &str) -> Result<Self> {
        let height = input.split("\n").count();
        let width = input.split("\n").next().ok_or(Error::InvalidInput)?.len();
        let tiles: Vec<_> = input
            .chars()
            .filter_map(|c| match c {
                '.' => Some(Tile::Empty),
                '@' => Some(Tile::Paper),
                _ => None,
            })
            .collect();
        if tiles.len() != (width * height) {
            return Err(Error::Parsing);
        } else {
            return Ok(Self {
                width,
                height,
                tiles,
            });
        }
    }

    fn tile_at(&self, x: usize, y: usize) -> Option<Tile> {
        if x < self.width && y < self.height {
            Some(self.tiles[y * self.width + x])
        } else {
            None // Over one or both upper bounds.
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Tile {
    Empty,
    Paper,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn p1() {
        assert_eq!(13, part1_impl(PAPER).unwrap());
    }

    const PAPER: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
}
