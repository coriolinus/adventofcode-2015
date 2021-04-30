//! # Day 11: Corporate Policy
//!
//! Santa's previous password expired, and he needs help choosing a new one.
//!
//! To help him remember his new password after the old one expires, Santa has devised a method of
//! coming up with a password based on the previous one. Corporate policy dictates that passwords
//! must be exactly eight lowercase letters (for security reasons), so he finds his new password by
//! incrementing his old password string repeatedly until it is valid.
//!
//! Incrementing is just like counting with numbers: xx, xy, xz, ya, yb, and so on. Increase the
//! rightmost letter one step; if it was `z`, it wraps around to `a`, and repeat with the next
//! letter to the left until one doesn't wrap around.
//!
//! Unfortunately for Santa, a new Security-Elf recently started, and he has imposed some
//! additional password requirements:
//!
//! - Passwords must include one increasing straight of at least three letters, like `abc`, `bcd`,
//!   `cde`, and so on, up to `xyz`. They cannot skip letters; `abd` doesn't count.
//! - Passwords may not contain the letters `i`, `o`, or `l`, as these letters can be mistaken for
//!   other characters and are therefore confusing.
//! - Passwords must contain at least two different, non-overlapping pairs of letters, like `aa`,
//!   `bb`, or `zz`.

use aoclib::parse;
use std::{fmt, path::Path};
use thiserror::Error;

// low order bytes are stored in low order indices
#[derive(Clone, Debug)]
struct Password(Vec<u8>);

impl std::str::FromStr for Password {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s
            .chars()
            .all(|ch| ch.is_ascii_alphabetic() && ch.is_ascii_lowercase())
        {
            Err("password must contain only lowercase ascii alphabetic chars")
        } else {
            let mut bytes = s.as_bytes().to_vec();
            bytes.reverse();
            Ok(Password(bytes))
        }
    }
}

impl fmt::Display for Password {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let mut bytes = self.0.clone();
        bytes.reverse();
        write!(
            f,
            "{}",
            std::str::from_utf8(&bytes).map_err(|_| fmt::Error)?
        )
    }
}

// returns `carry`
fn incr_char(ch: &mut u8) -> bool {
    let mut carry = false;
    *ch += 1;
    if *ch > b'z' {
        *ch = b'a';
        carry = true;
    }
    carry
}

impl Password {
    fn increment(&mut self) {
        let mut carry = true;
        for ch in self.0.iter_mut() {
            carry = incr_char(ch);
            if !carry {
                break;
            }
        }
        if carry {
            // overflow
            self.0.push(b'a');
        }
    }

    fn includes_increasing_straight(&self) -> bool {
        // note: this looks like a decreasing straight because
        // the password is stored backwards internally
        self.0
            .windows(3)
            .any(|window| window[0] == window[1] + 1 && window[1] == window[2] + 1)
    }

    fn includes_forbidden_char(&self) -> bool {
        self.0
            .iter()
            .any(|&ch| ch == b'i' || ch == b'o' || ch == b'l')
    }

    fn includes_at_least_two_non_overlapping_pairs(&self) -> bool {
        let mut last_window_position = None;
        let mut already_found_pair = false;
        for (idx, window) in self.0.windows(2).enumerate() {
            if window[0] == window[1] {
                if idx > 0 && last_window_position == Some(idx - 1) {
                    continue;
                }
                last_window_position = Some(idx);
                if !already_found_pair {
                    already_found_pair = true;
                } else {
                    return true;
                }
            }
        }
        false
    }

    pub fn valid(&self) -> bool {
        !self.includes_forbidden_char()
            && self.includes_increasing_straight()
            && self.includes_at_least_two_non_overlapping_pairs()
    }

    pub fn increment_checked(&mut self) {
        let mut is_valid = false;
        while !is_valid {
            self.increment();
            is_valid = self.valid();
        }
    }
}

pub fn part1(input: &Path) -> Result<(), Error> {
    for (idx, mut password) in parse::<Password>(input)?.enumerate() {
        password.increment_checked();
        println!("part 1 line {}: {}", idx, password);
    }
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    for (idx, mut password) in parse::<Password>(input)?.enumerate() {
        password.increment_checked();
        password.increment_checked();
        println!("part 2 line {}: {}", idx, password);
    }
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

    #[test]
    fn test_increment() {
        let from = vec!["", "a", "z", "xy", "xz", "ya", "zz", "hepxcrrq"];
        let to = vec!["a", "b", "aa", "xz", "ya", "yb", "aaa", "hepxcrrr"];

        for (from, to) in from.iter().zip(to) {
            dbg!(from);
            let mut password = dbg!(from.parse::<Password>()).unwrap();
            password.increment();
            assert_eq!(password.to_string(), to);
        }
    }

    #[test]
    fn test_valid() {
        let from = vec!["hijklmmn", "abbceffg", "abbcegjk", "abcdffaa", "ghjaabcc"];
        let to = vec![false, false, false, true, true];

        for (from, to) in from.iter().zip(to) {
            let password = from.parse::<Password>().unwrap();
            assert_eq!(password.valid(), to);
        }
    }

    #[test]
    fn test_increment_checked() {
        let from = vec!["abcdefgh", "ghijklmn"];
        let to = vec!["abcdffaa", "ghjaabcc"];

        for (from, to) in from.iter().zip(to) {
            let mut password = from.parse::<Password>().unwrap();
            password.increment_checked();
            assert_eq!(password.to_string(), to);
        }
    }

    #[test]
    fn test_contains_straight() {
        let from = vec![
            "hijklmmn", "abbceffg", "abbcegjk", "abcdffaa", "ghjaabcc", "ghjaaabb",
        ];
        let to = vec![true, false, false, true, true, false];

        for (from, to) in from.iter().zip(to) {
            let password = from.parse::<Password>().unwrap();
            assert_eq!(password.includes_increasing_straight(), to);
        }
    }
}
