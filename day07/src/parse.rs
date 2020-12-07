//! Tools for parsing the given instructions into a data structure mapping them.

use lazy_static::lazy_static;
use std::collections::HashSet;

lazy_static! {
    static ref LETTERS: HashSet<char> = {
        let mut letters = HashSet::new();
        for letter in "abcdefghijklmnopqrstuvwxyz".chars() {
            letters.insert(letter);
        }
        letters
    };
    static ref NUMBERS: HashSet<char> = {
        let mut numbers = HashSet::new();
        for number in "1234567890".chars() {
            numbers.insert(number);
        }
        numbers
    };
}

pub fn is_just_letters(s: &str) -> bool {
    for c in s.chars() {
        if !LETTERS.contains(&c) {
            return false;
        }
    }
    true
}

pub fn is_just_numbers(s: &str) -> bool {
    for c in s.chars() {
        if !NUMBERS.contains(&c) {
            return false;
        }
    }
    true
}

pub trait Parseable {
    type P;

    fn parse(input: &str) -> Option<Self::P>;
}
