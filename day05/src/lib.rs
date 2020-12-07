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
fn is_nice2(input: &str) -> bool {
    let input = input.trim().to_string().to_lowercase();
    if input.len() == 0 {
        return false;
    }

    // simple to iterate through the string and find a pair which repeats with a gap
    let char_skip_char = input.chars().zip(input.chars().skip(2));
    let mut rep_skip = false;
    for (first, second) in char_skip_char {
        if first == second {
            rep_skip = true;
            break;
        }
    }

    // break early to skip the expensive stuff, if we contain
    if !rep_skip {
        return rep_skip;
    }

    // now, the hard part: non-overlapping repeated pairs
    let mut nono_doubles = false;
    let in_len = input.len();
    let e_char_pairs = input.chars().zip(input.chars().skip(1)).enumerate();
    'outer: for (i, (first, second)) in e_char_pairs {
        if (i + 3) < in_len {
            let next_char_pairs = input.chars().zip(input.chars().skip(1)).skip(i + 2);
            for (third, fourth) in next_char_pairs {
                if first == third && second == fourth {
                    nono_doubles = true;
                    break 'outer;
                }
            }
        }
    }

    rep_skip && nono_doubles
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

pub fn count_nice(lines: &str) -> u32 {
    lines
        .split("\n")
        .fold(0, |acc, line| acc + if is_nice(line) { 1 } else { 0 })
}

pub fn count_nice2(lines: &str) -> u32 {
    lines
        .split("\n")
        .fold(0, |acc, line| acc + if is_nice2(line) { 1 } else { 0 })
}

#[cfg(test)]
mod tests {
    use super::is_nice;
    use super::is_nice2;

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

    /// - `qjhvhtzxzqqjkmpb` is nice because is has a pair that appears twice (`qj`) and a letter
    ///   that repeats with exactly one letter between them (`zxz`).
    /// - `xxyxx` is nice because it has a pair that appears twice and a letter that repeats with
    ///   one between, even though the letters used by each rule overlap.
    /// - `uurcxstgmygtbstg` is naughty because it has a pair (`tg`) but no repeat with a single
    ///   letter between them.
    /// - `ieodomkazucvgmuy` is naughty because it has a repeating letter with one between (`odo`),
    ///   but no pair that appears twice.
    #[test]
    fn test_examples2() {
        assert!(is_nice2("qjhvhtzxzqqjkmpb"));
        assert!(is_nice2("xxyxx"));
        assert!(!is_nice2("uurcxstgmygtbstg"));
        assert!(!is_nice2("ieodomkazucvgmuy"));
    }
}
