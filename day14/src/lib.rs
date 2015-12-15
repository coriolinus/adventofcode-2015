//! # Day 14: Reindeer Olympics
//!
//! This year is the Reindeer Olympics! Reindeer can fly at high speeds, but must rest occasionally
//! to recover their energy. Santa would like to know which of his reindeer is fastest, and so he
//! has them race.
//!
//! Reindeer can only either be flying (always at their top speed) or resting (not moving at all),
//! and always spend whole seconds in either state.
//!
//! For example, suppose you have the following Reindeer:
//!
//! - Comet can fly **14 km/s for 10 seconds**, but then must rest for **127** seconds.
//! - Dancer can fly **16 km/s for 11 seconds**, but then must rest for **162** seconds.
//!
//! After one second, Comet has gone 14 km, while Dancer has gone 16 km. After ten seconds, Comet
//! has gone 140 km, while Dancer has gone 160 km. On the eleventh second, Comet begins resting
//! (staying at 140 km), and Dancer continues on for a total distance of 176 km. On the 12th
//! second, both reindeer are resting. They continue to rest until the 138th second, when Comet
//! flies for another ten seconds. On the 174th second, Dancer flies for another 11 seconds.
//!
//! In this example, after the 1000th second, both reindeer are resting, and Comet is in the lead
//! at 1120 km (poor Dancer has only gotten 1056 km by that point). So, in this situation, Comet
//! would win (if the race ended at 1000 seconds).
//!
//! Given the descriptions of each reindeer (in your puzzle input), after exactly 2503 seconds,
//! what distance has the winning reindeer traveled?

use std::collections::HashMap;
use std::str::FromStr;

extern crate util;
use util::parse::{Parser, ParseError};

/// What a Reindeer is currently doing.
///
/// Both Flying and Resting keep track of how many seconds remain in that state.
/// When that number reaches `0`, the state immediately switches to the other state at its maximum
/// duration.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ReindeerState {
    Flying(u32),
    Resting(u32),
}
use ReindeerState::{Flying, Resting};

#[derive(Clone, Debug)]
pub struct Reindeer {
    pub name: String,
    /// km/s
    pub speed: u32,
    /// seconds
    pub fly_duration: u32,
    /// seconds
    pub rest_duration: u32,

    pub distance: u32,

    pub state: ReindeerState,
}

impl Reindeer {
    pub fn new(name: String, speed: u32, fly: u32, rest: u32) -> Reindeer {
        Reindeer {
            name: name,
            speed: speed,
            fly_duration: fly,
            rest_duration: rest,
            distance: 0,
            state: ReindeerState::Flying(fly),
        }
    }

    pub fn parse_lines(lines: &str) -> Result<Vec<Reindeer>, ParseError> {
        let mut ret = Vec::new();
        for line in lines.split('\n') {
            let r = Reindeer::from_line(line);
            if r.is_err() {
                let re = r.unwrap_err();
                if re == ParseError::InputIsEmpty {
                    continue;
                } else {
                    return Err(re);
                }
            }
            ret.push(r.unwrap());
        }

        Ok(ret)
    }

    pub fn from_line(line: &str) -> Result<Reindeer, ParseError> {
        let line = line.trim();
        if line.is_empty() {
            return Err(ParseError::InputIsEmpty);
        }

        let parser = Parser::default()
                         .force_lowercase(false)
                         .require_at_least(Some(15))
                         .require_fewer_than(Some(16))
                         .fixed_tokens({
                             let mut h = HashMap::new();
                             // 0 -> Name
                             h.insert(1, "can".to_string());
                             h.insert(2, "fly".to_string());
                             // 3 -> Speed
                             h.insert(4, "km/s".to_string());
                             h.insert(5, "for".to_string());
                             // 6 -> fly_duration
                             h.insert(7, "seconds,".to_string());
                             h.insert(8, "but".to_string());
                             h.insert(9, "then".to_string());
                             h.insert(10, "must".to_string());
                             h.insert(11, "rest".to_string());
                             h.insert(12, "for".to_string());
                             // 13 -> rest_duration
                             h.insert(14, "seconds.".to_string());
                             h
                         });

        match parser.parse(line) {
            Ok(v) => {
                let ref name = v.tokens[0];
                let ref speedt = v.tokens[1];
                let ref flyt = v.tokens[2];
                let ref restt = v.tokens[3];

                let speed = u32::from_str(speedt).unwrap();
                let fly = u32::from_str(flyt).unwrap();
                let rest = u32::from_str(restt).unwrap();

                Ok(Reindeer::new(name.to_owned(), speed, fly, rest))
            }
            Err(e) => Err(e),
        }
    }

    pub fn tick(&mut self) {
        match self.state {
            Flying(_) => self.distance += self.speed,
            _ => {}
        }

        self.state = match self.state {
            Flying(n) => {
                if n == 1 {
                    Resting(self.rest_duration)
                } else {
                    Flying(n - 1)
                }
            }
            Resting(n) => {
                if n == 1 {
                    Flying(self.fly_duration)
                } else {
                    Resting(n - 1)
                }
            }
        };
    }

    pub fn tick_all(rs: &mut Vec<Reindeer>) {
        for r in rs {
            r.tick();
        }
    }

    pub fn fast_forward(rs: &mut Vec<Reindeer>, seconds: usize) {
        for _ in 0..seconds {
            Reindeer::tick_all(rs);
        }
    }

    pub fn farthest(rs: &Vec<Reindeer>) -> Option<Reindeer> {
        let mut ret: Option<Reindeer> = None;

        for r in rs {
            ret = match ret {
                Some(r_) => {
                    if r_.distance < r.distance {
                        Some(r.clone())
                    } else {
                        Some(r_)
                    }
                }
                None => Some(r.clone()),
            }
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ReindeerState::{Flying, Resting};

    fn get_comet() -> Reindeer {
        Reindeer::new("Comet".to_string(), 14, 10, 127)
    }

    fn get_dancer() -> Reindeer {
        Reindeer::new("Dancer".to_string(), 16, 11, 162)
    }

    #[test]
    fn test_example_seconds() {
        let mut comet = get_comet();
        let mut dancer = get_dancer();

        comet.tick();
        dancer.tick();

        assert_eq!(14, comet.distance);
        assert_eq!(16, dancer.distance);

        for _ in 0..9 {
            comet.tick();
            dancer.tick();
        }

        assert_eq!(140, comet.distance);
        assert_eq!(160, dancer.distance);

        comet.tick();
        dancer.tick();

        assert_eq!(176, dancer.distance);
        assert_eq!(140, comet.distance);
        assert_eq!(Resting(126), comet.state);
        assert_eq!(Resting(162), dancer.state);

        comet.tick();
        dancer.tick();

        assert_eq!(Resting(125), comet.state);
        assert_eq!(Resting(161), dancer.state);
    }

    #[test]
    fn test_one_thousand_seconds() {
        let mut comet = get_comet();
        let mut dancer = get_dancer();

        for _ in 0..1000 {
            comet.tick();
            dancer.tick();
        }

        match comet.state {
            Resting(_) => {}
            Flying(_) => panic!("Comet should be resting!"),
        }
        match dancer.state {
            Resting(_) => {}
            Flying(_) => panic!("Dancer should be resting!"),
        }

        assert_eq!(comet.distance, 1120);
        assert_eq!(dancer.distance, 1056);
    }
}
