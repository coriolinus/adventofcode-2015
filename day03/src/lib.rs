//! # Day 3: Perfectly Spherical Houses in a Vacuum
//!
//! Santa is delivering presents to an infinite two-dimensional grid of houses.
//!
//! He begins by delivering a present to the house at his starting location, and then an elf at the
//! North Pole calls him via radio and tells him where to move next. Moves are always exactly one
//! house to the north (`^`), south (`v`), east (`>`), or west (`<`). After each move, he delivers
//! another present to the house at his new location.
//!
//! However, the elf back at the north pole has had a little too much eggnog, and so his directions
//! are a little off, and Santa ends up visiting some houses more than once. How many houses
//! receive at least one present?
//!
//! For example:
//!
//! - `>` delivers presents to 2 houses: one at the starting location, and one to the east.
//! - `^>v<` delivers presents to 4 houses in a square, including twice to the house at his
//!   starting/ending location.
//! - `^v^v^v^v^v` delivers a bunch of presents to some very lucky children at only 2 houses.

use aoclib::{
    geometry::{Direction, Point},
    parse,
};
use std::collections::{HashMap, HashSet};
use std::path::Path;
use thiserror::Error;

#[derive(Clone, Debug)]
pub struct CookieCrumbs {
    pub santa: Point,
    pub trail: HashMap<Point, u32>,
}

impl Default for CookieCrumbs {
    fn default() -> Self {
        let mut cc = CookieCrumbs {
            santa: Point::new(0, 0),
            trail: HashMap::new(),
        };

        // by the problem definition, Santa has already visited the house at the origin
        cc.trail.insert(cc.santa, 1);
        cc
    }
}

impl CookieCrumbs {
    pub fn new() -> CookieCrumbs {
        Self::default()
    }

    pub fn move_from_char(&mut self, ch: char) -> Result<(), Error> {
        let direction = match ch {
            '^' => Ok(Direction::Up),
            'v' => Ok(Direction::Down),
            '<' => Ok(Direction::Left),
            '>' => Ok(Direction::Right),
            _ => Err(Error::ParseDirection(ch)),
        }?;

        self.santa += direction;
        *self.trail.entry(self.santa).or_default() += 1;

        Ok(())
    }
}

/// Main point of entry to this lib: given a string of directions, follow santa and return the
/// pattern of cookie crumbs.
///
/// - `>` delivers presents to 2 houses: one at the starting location, and one to the east.
///
///   ```
///   # use day03::follow_santa;
///   assert_eq!(follow_santa(">").unwrap().trail.len(), 2);
///   ```
///
/// - `^>v<` delivers presents to 4 houses in a square, including twice to the house at his
///   starting/ending location.
///
///   ```
///   # use day03::follow_santa;
///   assert_eq!(follow_santa("^>v<").unwrap().trail.len(), 4);
///   ```
///
/// - `^v^v^v^v^v` delivers a bunch of presents to some very lucky children at only 2 houses.
///
///   ```
///   # use day03::follow_santa;
///   assert_eq!(follow_santa("^v^v^v^v^v").unwrap().trail.len(), 2);
///   ```
pub fn follow_santa(path: &str) -> Result<CookieCrumbs, Error> {
    let mut cc = CookieCrumbs::new();
    for ch in path.chars() {
        cc.move_from_char(ch)?;
    }
    Ok(cc)
}

pub fn part1(input: &Path) -> Result<(), Error> {
    for (idx, line) in parse::<String>(input)?.enumerate() {
        let delivered = follow_santa(&line)?.trail.len();
        println!("line {}: {} houses delivered to", idx, delivered);
    }
    Ok(())
}

/// Given a string of directions, divide them among `n` anonymous santas.
/// Each receives one direction in turn following the last.
///
/// # Narrative
///
/// The next year, to speed up the process, Santa creates a robot version of himself, Robo-Santa,
/// to deliver presents with him.
///
/// # Examples
///
/// - `^v` delivers presents to 3 houses, because Santa goes north, and then Robo-Santa goes south.
/// - `^>v<` now delivers presents to 3 houses, and Santa and Robo-Santa end up back where they
///   started.
/// - `^v^v^v^v^v` now delivers presents to 11 houses, with Santa going one direction and
///   Robo-Santa going the other.
///
/// # Code Examples
/// ```
/// # use day03::follow_n_santas;
/// # use day03::unique_houses;
/// let uh = unique_houses(&follow_n_santas("^v", 2).unwrap());
/// assert_eq!(uh, 3);
/// ```
/// ```
/// # use day03::follow_n_santas;
/// # use day03::unique_houses;
/// let uh = unique_houses(&follow_n_santas("^>v<", 2).unwrap());
/// assert_eq!(uh, 3);
/// ```
/// ```
/// # use day03::follow_n_santas;
/// # use day03::unique_houses;
/// let uh = unique_houses(&follow_n_santas("^v^v^v^v^v", 2).unwrap());
/// assert_eq!(uh, 11);
/// ```
pub fn follow_n_santas(path: &str, n: usize) -> Result<Vec<CookieCrumbs>, Error> {
    // initialize the output vector
    let mut vout = vec![CookieCrumbs::default(); n];

    for (i, ch) in path.chars().enumerate() {
        vout[i % n].move_from_char(ch)?;
    }

    Ok(vout)
}

pub fn unique_houses(v: &[CookieCrumbs]) -> usize {
    let mut houses: HashSet<Point> = HashSet::new();

    for cc in v {
        houses.extend(cc.trail.keys());
    }

    houses.len()
}

pub fn part2(input: &Path) -> Result<(), Error> {
    for (idx, line) in parse::<String>(input)?.enumerate() {
        let unique = unique_houses(&follow_n_santas(&line, 2)?);
        println!("2 santas: line {}: {} houses delivered to", idx, unique);
    }
    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("parsing direction from: {0}")]
    ParseDirection(char),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_default() {
        assert_eq!(Point::default(), Point::new(0, 0));
    }

    #[test]
    fn test_cc_new() {
        let cc = CookieCrumbs::new();
        assert_eq!(cc.santa, Point::default());
        assert_eq!(cc.trail.len(), 1);
        let first_crumb = cc.trail.keys().next().unwrap();
        assert_eq!(first_crumb, &Point::default());
        let first_visits = cc.trail.values().next().unwrap();
        assert_eq!(first_visits, &1);
    }
}
