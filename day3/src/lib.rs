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


#[derive(PartialEq, Eq, Hash, Default, Copy, Clone, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }

    pub fn north(&self) -> Point {
        Point { y: self.y + 1, ..*self }
    }
    pub fn south(&self) -> Point {
        Point { y: self.y - 1, ..*self }
    }
    pub fn east(&self) -> Point {
        Point { x: self.x + 1, ..*self }
    }
    pub fn west(&self) -> Point {
        Point { x: self.x - 1, ..*self }
    }

    pub fn move_from_char(&self, ch: &char) -> Point {
        match *ch {
            '^' => self.north(),
            'v' => self.south(),
            '>' => self.east(),
            '<' => self.west(),
            _ => self.clone(),
        }
    }
}

use std::collections::HashMap;

pub struct CookieCrumbs {
    pub santa: Point,
    pub trail: HashMap<Point, u32>,
}

impl CookieCrumbs {
    pub fn new() -> CookieCrumbs {
        let mut cc = CookieCrumbs {
            santa: Point::new(0, 0),
            trail: HashMap::new(),
        };

        // by the problem definition, Santa has already visited the house at the origin
        cc.trail.insert(cc.santa.clone(), 1);
        cc
    }

    pub fn move_from_char(&mut self, ch: &char) {
        let new_pt = self.santa.move_from_char(ch);
        if new_pt == self.santa {
            return; // no-op if we don't move
        }
        self.santa = new_pt;

        let mut insert = true;
        if let Some(val) = self.trail.get_mut(&self.santa) {
            *val += 1;
            insert = false;
        }
        if insert {
            self.trail.insert(self.santa.clone(), 1);
        }
    }
}

/// Main point of entry to this lib: given a string of directions, follow santa and return the
/// pattern of cookie crumbs.
///
/// - `>` delivers presents to 2 houses: one at the starting location, and one to the east.
///
///   ```
///   # use day3lib::follow_santa;
///   assert_eq!(follow_santa(">".to_string()).trail.len(), 2);
///   ```
///
/// - `^>v<` delivers presents to 4 houses in a square, including twice to the house at his
///   starting/ending location.
///
///   ```
///   # use day3lib::follow_santa;
///   assert_eq!(follow_santa("^>v<".to_string()).trail.len(), 4);
///   ```
///
/// - `^v^v^v^v^v` delivers a bunch of presents to some very lucky children at only 2 houses.
///
///   ```
///   # use day3lib::follow_santa;
///   assert_eq!(follow_santa("^v^v^v^v^v".to_string()).trail.len(), 2);
///   ```
pub fn follow_santa(path: &String) -> CookieCrumbs {
    let mut cc = CookieCrumbs::new();
    for ch in path.chars() {
        cc.move_from_char(&ch);
    }
    cc
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
/// # use day3lib::follow_n_santas;
/// # use day3lib::unique_houses;
/// let uh = unique_houses(&follow_n_santas("^v".to_string(), 2));
/// assert_eq!(uh, 3);
/// ```
/// ```
/// # use day3lib::follow_n_santas;
/// # use day3lib::unique_houses;
/// let uh = unique_houses(&follow_n_santas("^>v<".to_string(), 2));
/// assert_eq!(uh, 3);
/// ```
/// ```
/// # use day3lib::follow_n_santas;
/// # use day3lib::unique_houses;
/// let uh = unique_houses(&follow_n_santas("^v^v^v^v^v".to_string(), 2));
/// assert_eq!(uh, 11);
/// ```
pub fn follow_n_santas(path: &String, n: usize) -> Vec<CookieCrumbs> {

    // initialize the output vector
    let mut vout: Vec<CookieCrumbs> = Vec::new();
    for _ in 0..n {
        vout.push(CookieCrumbs::new());
    }

    for (i, ch) in path.chars().enumerate() {
        vout[i % n].move_from_char(&ch);
    }

    vout
}

use std::collections::HashSet;

pub fn unique_houses(v: &Vec<CookieCrumbs>) -> usize {
    let mut houses = HashSet::new();

    for cc in v {
        for house in cc.trail.keys() {
            houses.insert(house);
        }
    }

    houses.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_default() {
        assert_eq!(Point::default(), Point::new(0, 0));
    }

    #[test]
    fn test_point_directions() {
        assert_eq!(Point::default().north(), Point::new(0, 1));
        assert_eq!(Point::default().south(), Point::new(0, -1));
        assert_eq!(Point::default().east(), Point::new(1, 0));
        assert_eq!(Point::default().west(), Point::new(-1, 0));
    }

    #[test]
    fn test_point_move_from_char() {
        assert_eq!(Point::default().move_from_char(&' '), Point::new(0, 0));
        assert_eq!(Point::default().move_from_char(&'^'), Point::new(0, 1));
        assert_eq!(Point::default().move_from_char(&'v'), Point::new(0, -1));
        assert_eq!(Point::default().move_from_char(&'>'), Point::new(1, 0));
        assert_eq!(Point::default().move_from_char(&'<'), Point::new(-1, 0));
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
