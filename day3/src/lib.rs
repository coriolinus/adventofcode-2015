//! --- Day 3: Perfectly Spherical Houses in a Vacuum ---
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

#[derive(PartialEq, Eq, Hash, Default, Clone, Debug)]
pub struct Point {
    x: i32,
    y: i32,
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
    santa: Point,
    trail: HashMap<Point, u32>,
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
