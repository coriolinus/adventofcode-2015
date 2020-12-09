//! # Day 10: Elves Look, Elves Say
//!
//! Today, the Elves are playing a game called look-and-say. They take turns making sequences by
//! reading aloud the previous sequence and using that reading as the next sequence. For example,
//! `211` is read as "one two, two ones", which becomes `1221` (`1` `2`, `2` `1`s).
//!
//! Look-and-say sequences are generated iteratively, using the previous value as input for the
//! next step. For each step, take the previous value, and replace each run of digits (like `111`)
//! with the number of digits (`3`) followed by the digit itself (`1`).

use aoc2015::parse;
use std::path::Path;
use thiserror::Error;

pub fn look_and_say(sequence: &str) -> String {
    if sequence.is_empty() {
        return String::new();
    }

    let mut output = String::with_capacity(sequence.len() * 2);
    let mut current = sequence.chars().next().expect("non-empty; qed");
    let mut cur_count: u32 = 0;

    for ch in sequence.chars() {
        if ch != current {
            output += &cur_count.to_string();
            output.push(current);

            current = ch;
            cur_count = 0;
        }
        cur_count += 1;
    }

    output += &cur_count.to_string();
    output.push(current);

    output
}

pub fn look_and_say_n(sequence: &str, n: usize) -> String {
    let mut sequence = sequence.to_string();
    for _ in 0..n {
        sequence = look_and_say(&sequence);
    }
    sequence
}

pub fn part1(input: &Path) -> Result<(), Error> {
    for (idx, line) in parse::<String>(input)?.enumerate() {
        let l_s = look_and_say_n(&line, 40);
        println!("part 1 line {}: {}", idx, l_s.len());
    }
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    for (idx, line) in parse::<String>(input)?.enumerate() {
        let l_s = look_and_say_n(&line, 50);
        println!("part 2 line {}: {}", idx, l_s.len());
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
    use super::look_and_say;

    /// - `1` becomes `11` (1 copy of digit 1).
    /// - `11` becomes `21` (2 copies of digit 1).
    /// - `21` becomes `1211` (one 2 followed by one 1).
    /// - `1211` becomes `111221` (one 1, one 2, and two 1s).
    /// - `111221` becomes `312211` (three 1s, two 2s, and one 1).
    #[test]
    fn test_examples() {
        assert_eq!(look_and_say("1"), "11".to_string());
        assert_eq!(look_and_say("11"), "21".to_string());
        assert_eq!(look_and_say("21"), "1211".to_string());
        assert_eq!(look_and_say("1211"), "111221".to_string());
        assert_eq!(look_and_say("111221"), "312211".to_string());
    }
}
