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

use aoclib::{
    geometry::{Map, Point},
    parse,
};

use lalrpop_util::lalrpop_mod;
use std::{path::Path, str::FromStr};
use thiserror::Error;

lalrpop_mod!(pub parser);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    TurnOn,
    TurnOff,
    Toggle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Command {
    pub(crate) instruction: Instruction,
    pub(crate) from: Point,
    pub(crate) to: Point,
}

impl FromStr for Command {
    type Err = lalrpop_util::ParseError<usize, String, &'static str>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parser = parser::CommandParser::new();
        parser
            .parse(s)
            .map_err(|err| err.map_token(|t| t.to_string()))
    }
}

impl Command {
    fn apply<Light>(&self, map: &mut Map<Light>)
    where
        Instruction: ManipulateLight<Light>,
    {
        let min_x = self.from.x.min(self.to.x);
        let max_x = self.from.x.max(self.to.x);
        let min_y = self.from.y.min(self.to.y);
        let max_y = self.from.y.max(self.to.y);

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                self.instruction.manipulate(&mut map[Point::new(x, y)])
            }
        }
    }
}

trait ManipulateLight<Light> {
    fn manipulate(&self, light: &mut Light);
}

impl ManipulateLight<bool> for Instruction {
    fn manipulate(&self, light: &mut bool) {
        match self {
            Self::TurnOn => *light = true,
            Self::TurnOff => *light = false,
            Self::Toggle => *light = !*light,
        }
    }
}

impl ManipulateLight<u8> for Instruction {
    fn manipulate(&self, light: &mut u8) {
        match self {
            Self::TurnOn => *light = light.checked_add(1).expect("overflow"),
            Self::TurnOff => *light = light.saturating_sub(1),
            Self::Toggle => *light = light.checked_add(2).expect("overflow"),
        }
    }
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let mut map: Map<bool> = Map::new(1000, 1000);
    for command in parse::<Command>(input)? {
        command.apply(&mut map);
    }
    let lit = map.iter().filter(|light| **light).count();
    println!("{} lit", lit);
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let mut map: Map<u8> = Map::new(1000, 1000);
    for command in parse::<Command>(input)? {
        command.apply(&mut map);
    }
    let brightness = map.iter().map(|light| *light as u64).sum::<u64>();
    println!("brightness: {}", brightness);
    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! apply {
        ($s:expr, $map:expr) => {
            let command: Command = match $s.parse() {
                Ok(command) => command,
                Err(err) => {
                    println!("{}", err);
                    let mut err: &dyn std::error::Error = &err;
                    while let Some(source) = err.source() {
                        err = source;
                        println!("{}", err);
                    }
                    panic!()
                }
            };
            command.apply(&mut $map);
        };
    }

    /// For example:
    ///
    /// - `turn on 0,0 through 999,999` would turn on (or leave on) every light.
    /// - `toggle 0,0 through 999,0` would toggle the first line of 1000 lights, turning off the
    ///   ones that were on, and turning on the ones that were off.
    /// - `turn off 499,499 through 500,500` would turn off (or leave off) the middle four lights.
    #[test]
    fn test_examples() {
        macro_rules! expect {
            ($qty:expr, $map:expr) => {
                assert_eq!($map.iter().filter(|light| **light).count(), $qty);
            };
        }

        let mut lts: Map<bool> = Map::new(1000, 1000);

        expect!(0, lts);

        apply!("toggle 0,0 through 999,0", lts);
        expect!(1000, lts);

        apply!("turn on 0,0 through 999,999", lts);
        expect!(1_000_000, lts);

        apply!("toggle 0,0 through 999,0", lts);
        expect!(999000, lts);

        apply!("turn off 499,499 through 500,500", lts);
        expect!(998996, lts);
    }

    /// For example:
    ///
    /// - `turn on 0,0 through 0,0` would increase the total brightness by 1.
    /// - `toggle 0,0 through 999,999` would increase the total brightness by 2000000.
    #[test]
    fn test_part_2() {
        macro_rules! expect {
            ($qty:expr, $map:expr) => {
                assert_eq!($map.iter().map(|light| *light as u64).sum::<u64>(), $qty);
            };
        }

        let mut lts: Map<u8> = Map::new(1000, 1000);

        expect!(0, lts);

        apply!("turn on 0,0 through 0,0", lts);
        expect!(1, lts);

        apply!("toggle 0,0 through 999,999", lts);
        expect!(2000001, lts);
    }
}
