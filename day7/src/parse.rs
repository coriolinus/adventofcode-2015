//! Tools for parsing the given instructions into a data structure mapping them.

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

fn is_just_letters(s: &str) -> bool {
    for c in s.chars() {
        if !LETTERS.contains(&c) {
            return false;
        }
    }
    true
}

fn is_just_numbers(s: &str) -> bool {
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

pub struct Name {
    val: String,
}

impl Name {
    pub fn get(&self) -> &str {
        &self.val
    }
}

impl Parseable for Name {
    type P = Name;
    fn parse(v: &str) -> Option<Name> {
        if is_just_letters(v) {
            Some(Name { val: v.to_string() })
        } else {
            None
        }
    }
}

pub enum Evaluable {
    Num(u16),
    Name(Name),
}

impl Parseable for Evaluable {
    type P = Evaluable;

    fn parse(ev: &str) -> Option<Evaluable> {
        if is_just_letters(ev) {
            let n = Name::parse(ev);
            if n.is_none() {
                None
            } else {
                Some(Evaluable::Name(n.unwrap()))
            }
        } else if is_just_numbers(ev) {
            let n = u16::from_str_radix(ev, 10);
            if n.is_err() {
                None
            } else {
                Some(Evaluable::Num(n.unwrap()))
            }
        } else {
            None
        }
    }
}
