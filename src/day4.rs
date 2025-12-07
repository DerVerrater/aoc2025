use std::fs;

use itertools::Itertools;

use crate::{Error, Result};

pub fn part1() -> Result<i32> {
    let document = fs::read_to_string("./day4.txt").map_err(|_| Error::NoInputFile)?;
    part1_impl(document.as_str())
}

pub fn part2() -> Result<usize> {
    let document = fs::read_to_string("./day4.txt").map_err(|_| Error::NoInputFile)?;
    part2_impl(document.as_str())
}

fn part1_impl(input: &str) -> Result<i32> {
    let grid = Grid::try_new(input)?;
    let mut accessible_rolls = 0;
    for (y, x) in (0..grid.height).cartesian_product(0..grid.width) {
        // Only do the neighbor scan if we're on a paper roll.
        if let Some(Tile::Paper) = grid.tile_at(x, y) {
            // println!(" >>> Accessibility for: ({x}, {y})");
            let paper_neighbors = count_neighbors(&grid, (x, y));
            if paper_neighbors < 4 {
                // this paper roll can be accessed
                accessible_rolls += 1;
            } /* else, cannot be. Do nothing */
        }
    }
    Ok(accessible_rolls)
}

fn part2_impl(input: &str) -> Result<usize> {
    let mut grid = Grid::try_new(input)?;
    let mut removed_rolls = 0;
    let mut accessible_rolls = find_accessible(&grid);
    while !accessible_rolls.is_empty() {
        // count the (soon-to-be) removed rolls.
        removed_rolls += accessible_rolls.len();
        // remove the rolls (replace w/ Tile::Empty)
        accessible_rolls.into_iter().for_each(|(x, y)| {
            let maybe_tile = grid.tile_at_mut(x, y);
            if let Some(tile) = maybe_tile {
                *tile = Tile::Empty;
            } else {
                unreachable!("Tried emptying a removable paper roll, but Grid gave back a None. How can we remove something that doesn't exist?");
            }
        });
        // scan again
        accessible_rolls = find_accessible(&grid);
    }
    Ok(removed_rolls)
}

fn find_accessible(grid: &Grid) -> Vec<(usize, usize)> {
    let rect = (0..grid.height).cartesian_product(0..grid.width);
    let accessible_tiles: Vec<_> = rect
        .into_iter()
        // Get tile, emit (maybe_tile, coord) pairs
        .map(|(y, x)| (grid.tile_at(x, y), (x, y)))
        // convert maybe_tile to definitely_tile (turn None into Tile::Empty)
        .map(|(maybe_tile, coord)| {
            let tile = match maybe_tile {
                Some(tile) => tile,
                None => Tile::Empty,
            };
            (tile, coord)
        })
        // Remove non-Paper tiles
        .filter(|(tile, _coord)| tile == &Tile::Paper)
        // Split out only the coord
        .map(|(_tile, coord)| coord)
        // Check for accessibility, removing inaccessible candidates
        .filter(|coord| count_neighbors(grid, *coord) < 4)
        .collect();

    accessible_tiles
}

fn count_neighbors(grid: &Grid, target: (usize, usize)) -> usize {
    let (x, y) = target;
    // select kernel ranges based on whether or not we're up against the
    // zero line. -1 to 1 normally, 0 to 1 on lower border.
    let w_range = if x > 0 { (x - 1)..(x + 2) } else { x..(x + 2) };
    let h_range = if y > 0 { (y - 1)..(y + 2) } else { y..(y + 2) };
    let kernel = h_range.cartesian_product(w_range);
    // println!("Kernel: {kernel:?}");

    kernel
        .into_iter()
        // Skip kernel center
        .filter(|&(v, u)| (u, v) != (x, y))
        // Get Tile variant at (u,v)
        .map(|(v, u)| -> Tile {
            match grid.tile_at(u, v) {
                Some(tile) => tile,
                None => Tile::Empty, // Consider out-of-bounds tiles to be empty
            }
        })
        .filter(|&tile| tile == Tile::Paper)
        .count()
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
            Err(Error::Parsing)
        } else {
            Ok(Self {
                width,
                height,
                tiles,
            })
        }
    }

    fn tile_at(&self, x: usize, y: usize) -> Option<Tile> {
        if x < self.width && y < self.height {
            Some(self.tiles[y * self.width + x])
        } else {
            None // Over one or both upper bounds.
        }
    }

    fn tile_at_mut(&mut self, x: usize, y: usize) -> Option<&mut Tile> {
        if x < self.width && y < self.height {
            Some(&mut self.tiles[y * self.width + x])
        } else {
            None // Over one or both upper bounds.
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
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

    #[test]
    fn p2() {
        assert_eq!(43, part2_impl(PAPER).unwrap());
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
