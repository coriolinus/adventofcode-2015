use super::name::Name;
use super::parse::{is_just_letters, is_just_numbers, Parseable};

use std::collections::HashMap;

#[derive(Clone)]
pub enum Evaluable {
    Num(u16),
    Name(Name),
}

impl Evaluable {
    /// Given a lookup table containing Name -> value assignments, return my value
    ///
    /// # Panics
    /// Panics if self is a Name and lookup doesn't contain this Name as a key.
    pub fn get(&self, lookup: &HashMap<Name, u16>) -> u16 {
        match self {
            &Evaluable::Num(r) => r,
            &Evaluable::Name(ref n) => lookup.get(&n).unwrap().to_owned(),
        }
    }
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
