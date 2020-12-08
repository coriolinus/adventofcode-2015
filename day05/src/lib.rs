//! # Day 5: Doesn't He Have Intern-Elves For This?
//!
//! Santa needs help figuring out which strings in his text file are naughty or nice.
//!
//! A nice string is one with all of the following properties:
//!
//! - It contains at least three vowels (`aeiou` only), like `aei`, `xazegov`, or
//!   `aeiouaeiouaeiou`.
//! - It contains at least one letter that appears twice in a row, like `xx`, `abcdde` (`dd`), or
//!   `aabbccdd` (`aa`, `bb`, `cc`, or `dd`).
//! - It does not contain the strings ab, cd, pq, or xy, even if they are part of one of the other
//!   requirements.
//!
//! # For example:
//!
//! - `ugknbfddgicrmopn` is nice because it has at least three vowels (`u...i...o...`), a double
//!   letter (`...dd...`), and none of the disallowed substrings.
//! - `aaa` is nice because it has at least three vowels and a double letter, even though the
//!   letters used by different rules overlap.
//! - `jchzalrnumimnmhp` is naughty because it has no double letter.
//! - `haegwjzuvuyypxyu` is naughty because it contains the string `xy`.
//! - `dvszwmarrgswjxmb` is naughty because it contains only one vowel.

use aoc2015::parse;

use lazy_static::lazy_static;
use maplit::hashset;
use std::collections::HashSet;
use std::path::Path;
use thiserror::Error;

pub struct CharVec(Vec<char>);

impl std::str::FromStr for CharVec {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(CharVec(s.chars().collect()))
    }
}

lazy_static! {
    static ref VOWELS: HashSet<char> = hashset! {'a', 'e', 'i', 'o', 'u'};
}

fn has_enough_vowels(input: &[char]) -> bool {
    input.iter().filter(|c| VOWELS.contains(c)).count() >= 3
}

fn contains_double_letter(input: &[char]) -> bool {
    input.windows(2).any(|window| window[0] == window[1])
}

const NAUGHTY: &[&[char]] = &[&['a', 'b'], &['c', 'd'], &['p', 'q'], &['x', 'y']];

fn contains_naughty_sequence(input: &[char]) -> bool {
    input.windows(2).any(|window| NAUGHTY.contains(&window))
}

pub fn is_nice(input: &CharVec) -> bool {
    has_enough_vowels(&input.0)
        && contains_double_letter(&input.0)
        && !contains_naughty_sequence(&input.0)
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let nice = parse::<CharVec>(input)?.filter(is_nice).count();
    println!("part 1 nice strings count: {}", nice);
    Ok(())
}

fn contains_eye_pattern(chars: &[char]) -> bool {
    chars.windows(3).any(|window| window[0] == window[2])
}

fn contains_repeated_double(chars: &[char]) -> bool {
    chars.windows(2).enumerate().any(|(idx, outer_window)| {
        chars[idx + 2..]
            .windows(2)
            .any(|inner_window| inner_window == outer_window)
    })
}

/// Realizing the error of his ways, Santa has switched to a better model of determining whether a
/// string is naughty or nice. None of the old rules apply, as they are all clearly ridiculous.
///
/// Now, a nice string is one with all of the following properties:
///
/// - It contains a pair of any two letters that appears at least twice in the string without
///   overlapping, like `xyxy` (`xy`) or `aabcdefgaa` (`aa`), but not like `aaa` (`aa`, but it
///   overlaps).
/// - It contains at least one letter which repeats with exactly one letter between them, like
///   `xyx`, `abcdefeghi` (`efe`), or even `aaa`.
fn is_nice2(input: &CharVec) -> bool {
    contains_eye_pattern(&input.0) && contains_repeated_double(&input.0)
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let nice = parse::<CharVec>(input)?.filter(is_nice2).count();
    println!("part 2 nice strings count: {}", nice);
    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[cfg(test)]
mod tests {
    use crate::CharVec;

    use super::{is_nice, is_nice2};
    use rstest::rstest;

    /// - `ugknbfddgicrmopn` is nice because it has at least three vowels (`u...i...o...`), a double
    ///   letter (`...dd...`), and none of the disallowed substrings.
    /// - `aaa` is nice because it has at least three vowels and a double letter, even though the
    ///   letters used by different rules overlap.
    /// - `jchzalrnumimnmhp` is naughty because it has no double letter.
    /// - `haegwjzuvuyypxyu` is naughty because it contains the string `xy`.
    /// - `dvszwmarrgswjxmb` is naughty because it contains only one vowel.
    #[rstest(
        input,
        expect,
        case("ugknbfddgicrmopn", true),
        case("aaa", true),
        case("jchzalrnumimnmhp", false),
        case("haegwjzuvuyypxyu", false),
        case("dvszwmarrgswjxmb", false)
    )]
    fn test_examples(input: &str, expect: bool) {
        let charvec: CharVec = input.parse().unwrap();
        assert_eq!(is_nice(&charvec), expect);
    }

    /// - `qjhvhtzxzqqjkmpb` is nice because is has a pair that appears twice (`qj`) and a letter
    ///   that repeats with exactly one letter between them (`zxz`).
    /// - `xxyxx` is nice because it has a pair that appears twice and a letter that repeats with
    ///   one between, even though the letters used by each rule overlap.
    /// - `uurcxstgmygtbstg` is naughty because it has a pair (`tg`) but no repeat with a single
    ///   letter between them.
    /// - `ieodomkazucvgmuy` is naughty because it has a repeating letter with one between (`odo`),
    ///   but no pair that appears twice.
    #[rstest(
        input,
        expect,
        case("qjhvhtzxzqqjkmpb", true),
        case("xxyxx", true),
        case("uurcxstgmygtbstg", false),
        case("ieodomkazucvgmuy", false)
    )]
    fn test_examples2(input: &str, expect: bool) {
        let charvec: CharVec = input.parse().unwrap();
        assert_eq!(is_nice2(&charvec), expect);
    }
}
