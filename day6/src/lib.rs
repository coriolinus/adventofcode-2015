//! --- Day 6: Probably a Fire Hazard ---
//!
//! Because your neighbors keep defeating you in the holiday house decorating contest year after
//! year, you've decided to deploy one million lights in a 1000x1000 grid.
//!
//! Furthermore, because you've been especially nice this year, Santa has mailed you instructions
//! on how to display the ideal lighting configuration.
//!
//! Lights in your grid are numbered from 0 to 999 in each direction; the lights at each corner are
//! at `0,0`, `0,999`, `999,999`, and `999,0`. The instructions include whether to turn on,
//! turn off, or toggle various inclusive ranges given as coordinate pairs. Each coordinate pair
//! represents opposite corners of a rectangle, inclusive; a coordinate pair like `0,0 through 2,2`
//! therefore refers to 9 lights in a 3x3 square. The lights all start turned off.
//!
//! To defeat your neighbors this year, all you have to do is set up your lights by doing the
//! instructions Santa sent you in order.
//!
//! For example:
//!
//! - turn on `0,0` through `999,999` would turn on (or leave on) every light.
//! - toggle `0,0` through `999,0` would toggle the first line of 1000 lights, turning off the ones
//!   that were on, and turning on the ones that were off.
//! - `turn off 499,499 through 500,500` would turn off (or leave off) the middle four lights.
//!
//! After following the instructions, how many lights are lit?

extern crate day3lib;
use day3lib::Point;

use std::collections::HashMap;

pub trait Lightable {
    type L: Default + Copy;

    fn new() -> Self;

    fn count(&self) -> usize;
    fn turn_on(&mut self, point: Point);
    fn turn_off(&mut self, point: Point);
    fn toggle(&mut self, point: Point);

    fn get(&self, point: Point) -> Self::L;
    fn set(&mut self, point: Point, v: &Self::L);

    fn parse_lines(&mut self, lines: &str) {
        for line in lines.split('\n') {
            self.parse_line(line);
        }

    }

    fn parse_line(&mut self, instr: &str) -> bool {
        let instr = instr.trim().to_lowercase();
        if instr.is_empty() {
            return false;
        }

        let mut tokenizer = instr.split_whitespace();

        let first = tokenizer.next();
        if first.is_none() {
            return false;
        }

        let inst = if first.unwrap() == "turn" {
            tokenizer.next()
        } else {
            first
        };
        if inst.is_none() {
            return false;
        }
        let inst = inst.unwrap();

        let from = parse_point(tokenizer.next());
        if from.is_none() {
            return false;
        }
        let from = from.unwrap();

        let through = tokenizer.next();
        if through.is_none() || through.unwrap() != "through" {
            return false;
        }
        drop(through);

        let to = parse_point(tokenizer.next());
        if to.is_none() {
            return false;
        }
        let to = to.unwrap();

        for pt in Through::new(from, to) {
            match inst {
                "on" => self.turn_on(pt),
                "off" => self.turn_off(pt),
                "toggle" => self.toggle(pt),
                _ => return false,
            }
        }

        true
    }
}

pub struct Lights<T> {
    lights: HashMap<Point, T>,
}

impl Lightable for Lights<bool> {
    type L = bool;

    fn new() -> Lights<bool> {
        let hm = HashMap::new();

        Lights { lights: hm }
    }

    fn count(&self) -> usize {
        self.lights.values().filter(|&x| *x).count()
    }

    fn turn_on(&mut self, point: Point) {
        self.lights.insert(point, true);
    }

    fn turn_off(&mut self, point: Point) {
        self.lights.insert(point, false);
    }


    fn toggle(&mut self, point: Point) {
        let target = !self.get(point);
        self.lights.insert(point, target);
    }

    fn get(&self, point: Point) -> bool {
        if let Some(val) = self.lights.get(&point) {
            *val
        } else {
            false
        }
    }

    fn set(&mut self, point: Point, v: &bool) {
        self.lights.insert(point, *v);
    }
}

fn parse_point(input: Option<&str>) -> Option<Point> {
    if input.is_none() {
        return None;
    }
    let input = input.unwrap();
    let mut it = input.split(",");
    let x = it.next();
    let y = it.next();
    if x.is_none() || y.is_none() {
        return None;
    }
    if let (Ok(x), Ok(y)) = (i32::from_str_radix(x.unwrap(), 10),
                             i32::from_str_radix(y.unwrap(), 10)) {
        return Some(Point::new(x, y));
    }
    None
}

pub struct Through {
    from: Point,
    to: Point,
    cur_x: i32,
    cur_y: i32,
    finished: bool,
}

impl Through {
    pub fn new(from: Point, to: Point) -> Through {
        Through {
            from: from,
            to: to,
            cur_x: from.x,
            cur_y: from.y,
            finished: false,
        }
    }
}

impl Iterator for Through {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        let ret = Point::new(self.cur_x, self.cur_y);
        self.cur_x += 1;
        if self.cur_x > self.to.x {
            self.cur_x = self.from.x;
            self.cur_y += 1;
            if self.cur_y > self.to.y {
                self.finished = true;
            }
        }
        Some(ret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// For example:
    ///
    /// - `turn on 0,0 through 999,999` would turn on (or leave on) every light.
    /// - `toggle 0,0 through 999,0` would toggle the first line of 1000 lights, turning off the
    ///   ones that were on, and turning on the ones that were off.
    /// - `turn off 499,499 through 500,500` would turn off (or leave off) the middle four lights.
    #[test]
    fn test_examples() {
        let mut lts = Lights::new();
        assert_eq!(0, lts.count());

        lts.parse_line("turn on 0,0 through 999,999");
        assert_eq!(1000000, lts.count());

        lts.parse_line("toggle 0,0 through 999,0");
        assert_eq!(999000, lts.count());

        lts.parse_line("turn off 499,499 through 500,500");
        assert_eq!(998996, lts.count());
    }
}
