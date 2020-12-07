//! # Day 16: Aunt Sue
//!
//! Your Aunt Sue has given you a wonderful gift, and you'd like to send her a thank you card.
//!  However, there's a small problem: she signed it "From, Aunt Sue".
//!
//! You have 500 Aunts named "Sue".
//!
//! So, to avoid sending the card to the wrong person, you need to figure out which Aunt Sue
//! (which you conveniently number 1 to 500, for sanity) gave you the gift. You open the present
//! and, as luck would have it, good ol' Aunt Sue got you a My First Crime Scene Analysis Machine!
//! Just what you wanted. Or needed, as the case may be.
//!
//! The My First Crime Scene Analysis Machine (MFCSAM for short) can detect a few specific
//! compounds in a given sample, as well as how many distinct kinds of those compounds there are.
//! According to the instructions, these are what the MFCSAM can detect:
//!
//! - `children`, by human DNA age analysis.
//! - `cats`. It doesn't differentiate individual breeds.
//! - Several seemingly random breeds of dog: `samoyeds`, `pomeranians`, `akitas`, and `vizslas`.
//! - `goldfish`. No other kinds of fish.
//! - `trees`, all in one group.
//! - `cars`, presumably by exhaust or gasoline or something.
//! - `perfumes`, which is handy, since many of your Aunts Sue wear a few kinds.
//!
//! In fact, many of your Aunts Sue have many of these. You put the wrapping from the gift into the
//! MFCSAM. It beeps inquisitively at you a few times and then prints out a message on ticker tape:
//!
//! ```notrust
//! children: 3
//! cats: 7
//! samoyeds: 2
//! pomeranians: 3
//! akitas: 0
//! vizslas: 0
//! goldfish: 5
//! trees: 3
//! cars: 2
//! perfumes: 1
//! ```
//!
//! You make a list of the things you can remember about each Aunt Sue. Things missing from your
//! list aren't zero - you simply don't remember the value.
//!
//! What is the number of the Sue that got you the gift?

use std::collections::HashMap;
use std::str::FromStr;

use util::parse::Parser;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum MfcsamItem {
    Children,
    Cats,
    Samoyeds,
    Pomeranians,
    Akitas,
    Vizslas,
    Goldfish,
    Trees,
    Cars,
    Perfumes,
}
use MfcsamItem::{
    Akitas, Cars, Cats, Children, Goldfish, Perfumes, Pomeranians, Samoyeds, Trees, Vizslas,
};

fn get_mfcsam_item(item: &str) -> Option<MfcsamItem> {
    let mut h = HashMap::new();

    h.insert("children", Children);
    h.insert("children:", Children);
    h.insert("cats", Cats);
    h.insert("cats:", Cats);
    h.insert("samoyeds", Samoyeds);
    h.insert("samoyeds:", Samoyeds);
    h.insert("pomeranians", Pomeranians);
    h.insert("pomeranians:", Pomeranians);
    h.insert("akitas", Akitas);
    h.insert("akitas:", Akitas);
    h.insert("vizslas", Vizslas);
    h.insert("vizslas:", Vizslas);
    h.insert("goldfish", Goldfish);
    h.insert("goldfish:", Goldfish);
    h.insert("trees", Trees);
    h.insert("trees:", Trees);
    h.insert("cars", Cars);
    h.insert("cars:", Cars);
    h.insert("perfumes", Perfumes);
    h.insert("perfumes:", Perfumes);

    if let Some(m) = h.get(item) {
        Some(m.clone())
    } else {
        None
    }
}

pub type MfcsamQtys = HashMap<MfcsamItem, u8>;

pub fn mfcsam_result() -> MfcsamQtys {
    let mut h = HashMap::new();
    h.insert(Children, 3);
    h.insert(Cats, 7);
    h.insert(Samoyeds, 2);
    h.insert(Pomeranians, 3);
    h.insert(Akitas, 0);
    h.insert(Vizslas, 0);
    h.insert(Goldfish, 5);
    h.insert(Trees, 3);
    h.insert(Cars, 2);
    h.insert(Perfumes, 1);
    h
}

#[derive(PartialEq, Eq, Debug)]
pub struct Sue {
    pub num: u16,
    pub possessions: MfcsamQtys,
}

impl Sue {
    pub fn parse(line: &str) -> Option<Sue> {
        let line = line.trim();
        if line.is_empty() {
            return None;
        }

        let parser = Parser::default()
            .clear_trailing_punctuation(true)
            .require_at_least(Some(2))
            .fixed_tokens({
                let mut h = HashMap::new();
                h.insert(0, "sue".to_string());
                h
            });

        match parser.parse(line) {
            Err(_) => None,
            Ok(v) => {
                let ref num = v.tokens[0];
                let num = u16::from_str(num);
                if num.is_err() {
                    return None;
                }
                let num = num.unwrap();

                let mut sue = Sue {
                    num: num,
                    possessions: HashMap::new(),
                };
                for chunk in v.tokens.iter().skip(1).collect::<Vec<_>>().chunks(2) {
                    if chunk.len() != 2 {
                        return None;
                    }

                    if let Some(mfc) = get_mfcsam_item(chunk[0]) {
                        if let Ok(qty) = u8::from_str(chunk[1]) {
                            sue.possessions.insert(mfc, qty);
                        }
                    }
                }
                Some(sue)
            }
        }
    }

    pub fn can_be(&self, qtys: &MfcsamQtys) -> bool {
        for (k, v) in &self.possessions {
            if qtys.contains_key(k) && qtys.get(k).unwrap() != v {
                return false;
            }
        }
        true
    }

    pub fn can_be_retro(&self, qtys: &MfcsamQtys) -> bool {
        for (k, v) in &self.possessions {
            if let Some(detected) = qtys.get(k) {
                match k {
                    &Cats => {
                        if !(v > detected) {
                            return false;
                        }
                    }
                    &Trees => {
                        if !(v > detected) {
                            return false;
                        }
                    }
                    &Pomeranians => {
                        if !(v < detected) {
                            return false;
                        }
                    }
                    &Goldfish => {
                        if !(v < detected) {
                            return false;
                        }
                    }
                    _ => {
                        if v != detected {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }
}

pub fn check_sues(items: &MfcsamQtys, lines: &str) -> Vec<Sue> {
    let mut ret = Vec::new();

    let lines = lines.split('\n');
    for line in lines {
        if let Some(sue) = Sue::parse(line) {
            if sue.can_be(&items) {
                ret.push(sue);
            }
        }
    }

    ret
}

pub fn check_sues_retro(items: &MfcsamQtys, lines: &str) -> Vec<Sue> {
    let mut ret = Vec::new();

    let lines = lines.split('\n');
    for line in lines {
        if let Some(sue) = Sue::parse(line) {
            if sue.can_be_retro(&items) {
                ret.push(sue);
            }
        }
    }

    ret
}
