//! # Day 18: Like a GIF For Your Yard
//!
//! After the million lights incident, the fire code has gotten stricter: now, at most ten thousand
//! lights are allowed. You arrange them in a 100x100 grid.
//!
//! Never one to let you down, Santa again mails you instructions on the ideal lighting
//! configuration. With so few lights, he says, you'll have to resort to animation.
//!
//! Start by setting your lights to the included initial configuration (your puzzle input).
//! A `#` means "on", and a `.` means "off".
//!
//! Then, animate your grid in steps, where each step decides the next configuration based on the current one. Each light's next state (either on or off) depends on its current state and the current states of the eight lights adjacent to it (including diagonals). Lights on the edge of the grid might have fewer than eight neighbors; the missing ones always count as "off".
//!
//! For example, in a simplified 6x6 grid, the light marked A has the neighbors numbered 1 through 8, and the light marked B, which is on an edge, only has the neighbors marked 1 through 5:
//!
//! ```notrust
//! 1B5...
//! 234...
//! ......
//! ..123.
//! ..8A4.
//! ..765.
//! ```
//!
//! The state a light should have next is based on its current state (on or off) plus the number of neighbors that are on:
//!
//! - A light which is on stays on when `2` or `3` neighbors are on, and turns off otherwise.
//! - A light which is off turns on if exactly `3` neighbors are on, and stays off otherwise.
//! All of the lights update simultaneously; they all consider the same current state before moving to the next.
//!
//! Here's a few steps from an example configuration of another 6x6 grid:
//!
//! ```notrust
//! Initial state:
//! .#.#.#
//! ...##.
//! #....#
//! ..#...
//! #.#..#
//! ####..
//!
//! After 1 step:
//! ..##..
//! ..##.#
//! ...##.
//! ......
//! #.....
//! #.##..
//!
//! After 2 steps:
//! ..###.
//! ......
//! ..###.
//! ......
//! .#....
//! .#....
//!
//! After 3 steps:
//! ...#..
//! ......
//! ...#..
//! ..##..
//! ......
//! ......
//!
//! After 4 steps:
//! ......
//! ......
//! ..##..
//! ..##..
//! ......
//! ......
//! ```
//!
//! After 4 steps, this example has four lights on.
//!
//! In your grid of 100x100 lights, given your initial configuration, how many lights are on after 100 steps?

use aoclib::geometry::{tile::DisplayWidth, Map};
use std::path::Path;
use thiserror::Error;

#[cfg(feature = "animate")]
pub mod animate;

pub const ITERATIONS: u8 = 100;

#[derive(Debug, Clone, Copy, PartialEq, Eq, parse_display::FromStr, parse_display::Display)]
pub enum Light {
    #[display("#")]
    On,
    #[display(".")]
    Off,
}

impl DisplayWidth for Light {
    const DISPLAY_WIDTH: usize = 1;
}

impl Light {
    #[inline(always)]
    fn is_on(&self) -> bool {
        *self == Light::On
    }
}

pub type Grid = Map<Light>;

pub fn next_state(grid: &Grid) -> Grid {
    let mut successor = grid.clone();

    successor.for_each_point_mut(|light, point| {
        let adjacent_on = grid
            .adjacencies(point)
            .filter(|&adj| grid[adj].is_on())
            .count();

        match (light.is_on(), adjacent_on) {
            (true, n) if (2..=3).contains(&n) => {
                // a light which is on stays on when 2 or 3 neighbors are on
            }
            (true, _) => {
                // ...and turns off otherwise
                *light = Light::Off
            }
            (false, 3) => {
                // a light which is off turns on if exactly 3 neighbors are on
                *light = Light::On;
            }
            (false, _) => {
                // ...and stays off otherwise
            }
        }
    });

    successor
}

pub fn next_state_stuck(grid: &Grid) -> Grid {
    let mut grid = next_state(grid);

    for corner in [
        grid.top_left(),
        grid.top_right(),
        grid.bottom_left(),
        grid.bottom_right(),
    ]
    .iter()
    {
        grid[*corner] = Light::On;
    }

    grid
}

pub fn count_on(grid: &Grid) -> usize {
    grid.iter().filter(|light| light.is_on()).count()
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let file = std::fs::File::open(input)?;
    let buffer = std::io::BufReader::new(file);
    let mut grid = Grid::try_from(buffer)?;
    for _ in 0..ITERATIONS {
        grid = next_state(&grid);
    }
    let on = count_on(&grid);
    println!("{:5} lights on after {} iterations", on, ITERATIONS);
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let file = std::fs::File::open(input)?;
    let buffer = std::io::BufReader::new(file);
    let mut grid = Grid::try_from(buffer)?;
    for _ in 0..ITERATIONS {
        grid = next_state_stuck(&grid);
    }
    let on = count_on(&grid);
    println!(
        "{:5} lights on after {} iterations (part 2)",
        on, ITERATIONS
    );
    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[cfg(feature = "animate")]
    #[error("encoding gif")]
    Gif(#[from] gif::EncodingError),
    #[error("could not read map")]
    MapConversion(#[from] aoclib::geometry::map::MapConversionErr),
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
.#.#.#
...##.
#....#
..#...
#.#..#
####..
";

    fn get_example() -> Grid {
        Grid::try_from(EXAMPLE.trim()).unwrap()
    }

    #[test]
    fn test_example() {
        let mut grid = get_example();
        assert_eq!(count_on(&grid), 15);
        println!();
        println!("Initial State:");
        println!("{}", grid.to_string());

        // 1st step
        grid = next_state(&grid);
        println!("After 1st step:");
        println!("{}", grid.to_string());
        assert_eq!(count_on(&grid), 11);

        // 2nd step
        grid = next_state(&grid);
        println!("After 2nd step:");
        println!("{}", grid.to_string());
        assert_eq!(count_on(&grid), 8);

        // 3rd step
        grid = next_state(&grid);
        println!("After 3rd step:");
        println!("{}", grid.to_string());
        assert_eq!(count_on(&grid), 4);

        // 4th step
        grid = next_state(&grid);
        println!("After 4th step:");
        println!("{}", grid.to_string());
        assert_eq!(count_on(&grid), 4);
    }
}
