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

const ALLOWED_LETTERS: &'static str = "abcdefghjkmnpqrstuvwxyz";
const FIRST_ALLOWED_LETTER: char = 'a';

pub fn increment(old: &str) -> String {
    if old.is_empty() {
        return FIRST_ALLOWED_LETTER.to_string(); // handles the recursive case
    }

    let mut ret = old.to_string();
    let last_char = ret.pop().unwrap();

    if let Some(new_last_char) = increment_char(last_char) {
        ret.push(new_last_char);
        return ret;
    } else {
        // Can't get the new last char, so increment the whole string
        ret = increment(&ret);
        ret.push(FIRST_ALLOWED_LETTER);
        return ret;
    }
}

fn increment_char(old: char) -> Option<char> {
    increment_char_given_alphabet(old, ALLOWED_LETTERS)
}

fn increment_char_given_alphabet(old: char, alphabet: &str) -> Option<char> {
    let mut allowed_chs = alphabet.chars();
    loop {
        if let Some(next) = allowed_chs.next() {
            if next == old {
                break;
            }
        } else {
            // we've run out of characters
            // input wasn't in ALLOWED_LETTERS
            // so we do this by hand
            return match old {
                'i' => Some('j'),
                'l' => Some('m'),
                'o' => Some('p'),
                _ => None,
            };
        }
    }
    // None if `old` was `'z'`
    allowed_chs.next()
}

fn contains_forbidden(s: &str) -> bool {
    for ch in s.chars() {
        for f in vec!['i', 'o', 'l'] {
            if ch == f {
                return true;
            }
        }
    }
    false
}

fn contains_straight(s: &str) -> bool {
    if s.len() < 3 {
        return false;
    }

    let triple_it = s
        .chars()
        .zip(s.chars().skip(1).zip(s.chars().skip(2)))
        .map(|(x, (y, z))| (x, y, z));

    for (x, y, z) in triple_it {
        if let Some(x1) = increment_char_given_alphabet(x, "abcdefghijklmnopqrstuvwxyz") {
            if let Some(x2) = increment_char_given_alphabet(x1, "abcdefghijklmnopqrstuvwxyz") {
                if y == x1 && z == x2 {
                    return true;
                }
            }
        }
    }

    false
}

fn contains_pairs(s: &str) -> bool {
    if s.len() < 4 {
        return false;
    }

    let double_it = s.chars().zip(s.chars().skip(1)).enumerate();

    for (i, (a, b)) in double_it {
        if a == b {
            for (y, z) in s.chars().zip(s.chars().skip(1)).skip(i + 2) {
                if y == z && a != y {
                    return true;
                }
            }
        }
    }

    false
}

pub fn meets_requirements(s: &str) -> bool {
    s.len() == 8 && !contains_forbidden(s) && contains_straight(s) && contains_pairs(s)
}

pub fn next_pw(s: &str) -> String {
    let mut new = increment(s);
    while !meets_requirements(&new) {
        new = increment(&new);
        if new.len() > 8 {
            return String::from("");
        }
    }
    new
}

#[cfg(test)]
mod tests {
    use super::contains_straight;
    use super::{increment, meets_requirements, next_pw};

    #[test]
    fn test_increment() {
        let from = vec![
            "", "a", "h", "k", "n", "z", "xy", "xz", "ya", "zz", "hepxcrrq",
        ];
        let to = vec![
            "a", "b", "j", "m", "p", "aa", "xz", "ya", "yb", "aaa", "hepxcrrr",
        ];

        for (from, to) in from.iter().zip(to) {
            assert_eq!(increment(from), to.to_string());
        }
    }

    #[test]
    fn test_meets_requirements() {
        let from = vec!["hijklmmn", "abbceffg", "abbcegjk", "abcdffaa", "ghjaabcc"];
        let to = vec![false, false, false, true, true];

        for (from, to) in from.iter().zip(to) {
            assert_eq!(meets_requirements(from), to);
        }
    }

    #[test]
    fn test_next_pw() {
        let from = vec!["abcdefgh", "ghijklmn"];
        let to = vec!["abcdffaa", "ghjaabcc"];

        for (from, to) in from.iter().zip(to) {
            assert_eq!(next_pw(from), to);
        }
    }

    #[test]
    fn test_contains_straight() {
        let from = vec![
            "hijklmmn", "abbceffg", "abbcegjk", "abcdffaa", "ghjaabcc", "ghjaaabb",
        ];
        let to = vec![true, false, false, true, true, false];

        for (from, to) in from.iter().zip(to) {
            println!("Trying: {}", from);
            assert_eq!(contains_straight(from), to);
        }
    }

    #[test]
    fn test_high_increments() {
        let mut last5 = Vec::new();

        let mut cur = String::from("hepxcrrq");

        for _ in 0..100000 {
            last5.push(cur.clone());
            while last5.len() > 5 {
                last5.remove(0);
            }

            if cur.len() > 8 {
                break;
            }

            cur = increment(&cur);
        }

        for (i, l) in last5.iter().enumerate() {
            println!("last5[-{}]: {}", 5 - i, l);
        }

        if cur.len() > 8 {
            panic!();
        }
    }
}
