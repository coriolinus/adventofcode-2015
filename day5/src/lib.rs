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

use std::collections::HashSet;

pub fn is_nice(input: &str) -> bool {
    let input = input.trim().to_string().to_lowercase();
    if input.len() == 0 {
        return false;
    }

    let mut chars = input.chars();
    let first_char = chars.next(); // advance to the 2nd char

    let char_pairs = input.chars().zip(chars);

    let vowels_set = get_vowels();
    let naughty_set = get_naughty_pairs();

    let mut vowels = 0;
    if vowels_set.contains(&first_char.unwrap()) {
        vowels += 1;
    }

    let mut has_double = false;
    let mut has_naughty = false;

    for (first, second) in char_pairs {
        if vowels_set.contains(&second) {
            vowels += 1;
        }
        if first == second {
            has_double = true;
        }
        if !has_naughty && naughty_set.contains(&(first, second)) {
            has_naughty = true;
        }
    }

    has_double && !has_naughty && vowels >= 3
}

fn get_vowels() -> HashSet<char> {
    let mut set = HashSet::new();
    set.insert('a');
    set.insert('e');
    set.insert('i');
    set.insert('o');
    set.insert('u');
    set
}

fn get_naughty_pairs() -> HashSet<(char, char)> {
    let mut set = HashSet::new();
    set.insert(('a', 'b'));
    set.insert(('c', 'd'));
    set.insert(('p', 'q'));
    set.insert(('x', 'y'));
    set
}

pub fn count_nice(lines: &str) -> i32 {
    lines.split("\n").fold(0, |acc, line| {
        acc +
        if is_nice(line) {
            1
        } else {
            0
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// - `ugknbfddgicrmopn` is nice because it has at least three vowels (`u...i...o...`), a double
    ///   letter (`...dd...`), and none of the disallowed substrings.
    /// - `aaa` is nice because it has at least three vowels and a double letter, even though the
    ///   letters used by different rules overlap.
    /// - `jchzalrnumimnmhp` is naughty because it has no double letter.
    /// - `haegwjzuvuyypxyu` is naughty because it contains the string `xy`.
    /// - `dvszwmarrgswjxmb` is naughty because it contains only one vowel.
    #[test]
    fn test_examples() {
        assert!(is_nice("ugknbfddgicrmopn"));
        assert!(is_nice("aaa"));
        assert!(!is_nice("jchzalrnumimnmhp"));
        assert!(!is_nice("haegwjzuvuyypxyu"));
        assert!(!is_nice("dvszwmarrgswjxmb"));
    }
}
