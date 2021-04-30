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

use aoclib::parse;
use std::path::Path;
use std::str::FromStr;
use thiserror::Error;

const RESULT: MfcsamQtys = MfcsamQtys {
    children: Some(3),
    cats: Some(7),
    samoyeds: Some(2),
    pomeranians: Some(3),
    akitas: Some(0),
    vizslas: Some(0),
    goldfish: Some(5),
    trees: Some(3),
    cars: Some(2),
    perfumes: Some(1),
};

#[derive(Debug, PartialEq, Eq, Default, Clone)]
struct MfcsamQtys {
    children: Option<u32>,
    cats: Option<u32>,
    samoyeds: Option<u32>,
    pomeranians: Option<u32>,
    akitas: Option<u32>,
    vizslas: Option<u32>,
    goldfish: Option<u32>,
    trees: Option<u32>,
    cars: Option<u32>,
    perfumes: Option<u32>,
}

impl FromStr for MfcsamQtys {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut qtys = MfcsamQtys::default();

        for item in s.split(',') {
            let item = item.trim();

            let err = || Error::MalformedItem(item.to_string());

            let mut kvs = item.split(':');
            let name = kvs.next().ok_or_else(err)?;
            let qty = kvs.next().ok_or_else(err)?.trim().parse::<u32>()?;
            if kvs.next().is_some() {
                return Err(err());
            }

            match name {
                "children" => qtys.children = Some(qty),
                "cats" => qtys.cats = Some(qty),
                "samoyeds" => qtys.samoyeds = Some(qty),
                "pomeranians" => qtys.pomeranians = Some(qty),
                "akitas" => qtys.akitas = Some(qty),
                "vizslas" => qtys.vizslas = Some(qty),
                "goldfish" => qtys.goldfish = Some(qty),
                "trees" => qtys.trees = Some(qty),
                "cars" => qtys.cars = Some(qty),
                "perfumes" => qtys.perfumes = Some(qty),
                _ => {
                    // just ignore extraneous possessions
                }
            }
        }

        Ok(qtys)
    }
}

impl MfcsamQtys {
    /// `true` when all items specified in `other` are specified here and quantities match.
    ///
    /// I.e. can return `true` if `self.cats == None` and `other.cats == Some(3)`,
    /// but will always return `false` if `self.cats == Some(3)` and `other.cats = None`/
    fn matches(&self, other: &MfcsamQtys) -> bool {
        self.children
            .map(|x| other.children == Some(x))
            .unwrap_or(true)
            && self.cats.map(|x| other.cats == Some(x)).unwrap_or(true)
            && self
                .samoyeds
                .map(|x| other.samoyeds == Some(x))
                .unwrap_or(true)
            && self
                .pomeranians
                .map(|x| other.pomeranians == Some(x))
                .unwrap_or(true)
            && self.akitas.map(|x| other.akitas == Some(x)).unwrap_or(true)
            && self
                .vizslas
                .map(|x| other.vizslas == Some(x))
                .unwrap_or(true)
            && self
                .goldfish
                .map(|x| other.goldfish == Some(x))
                .unwrap_or(true)
            && self.trees.map(|x| other.trees == Some(x)).unwrap_or(true)
            && self.cars.map(|x| other.cars == Some(x)).unwrap_or(true)
            && self
                .perfumes
                .map(|x| other.perfumes == Some(x))
                .unwrap_or(true)
    }

    /// Same semantics as [`Mfcsamqtys::matches`], but with the following adaptations:
    ///
    /// - `self.cats > other.cats`
    /// - `self.trees > other.trees`
    /// - `self.pomeranians > other.pomeranians`
    /// - `self.goldfish < other.goldfish`
    fn matches_retro(&self, other: &MfcsamQtys) -> bool {
        self.children
            .map(|x| other.children == Some(x))
            .unwrap_or(true)
            && self.cats.map(|x| other.cats < Some(x)).unwrap_or(true)
            && self
                .samoyeds
                .map(|x| other.samoyeds == Some(x))
                .unwrap_or(true)
            && self
                .pomeranians
                .map(|x| other.pomeranians > Some(x))
                .unwrap_or(true)
            && self.akitas.map(|x| other.akitas == Some(x)).unwrap_or(true)
            && self
                .vizslas
                .map(|x| other.vizslas == Some(x))
                .unwrap_or(true)
            && self
                .goldfish
                .map(|x| other.goldfish > Some(x))
                .unwrap_or(true)
            && self.trees.map(|x| other.trees < Some(x)).unwrap_or(true)
            && self.cars.map(|x| other.cars == Some(x)).unwrap_or(true)
            && self
                .perfumes
                .map(|x| other.perfumes == Some(x))
                .unwrap_or(true)
    }
}

#[derive(PartialEq, Eq, Debug, parse_display::FromStr)]
#[display("Sue {num}: {possessions}")]
pub struct Sue {
    num: u32,
    possessions: MfcsamQtys,
}

impl Sue {
    fn can_be(&self, qtys: &MfcsamQtys) -> bool {
        self.possessions.matches(qtys)
    }

    fn can_be_retro(&self, qtys: &MfcsamQtys) -> bool {
        self.possessions.matches_retro(qtys)
    }
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let mut found_sue = false;
    for sue in parse::<Sue>(input)? {
        if sue.can_be(&RESULT) {
            println!("matching sue: {}", sue.num);
            found_sue = true;
        }
    }
    if !found_sue {
        println!("no matching sue found");
    }
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let mut found_sue = false;
    for sue in parse::<Sue>(input)? {
        if sue.can_be_retro(&RESULT) {
            println!("matching sue (retro): {}", sue.num);
            found_sue = true;
        }
    }
    if !found_sue {
        println!("no matching sue (retro) found");
    }
    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("malformed item: \"{0}\"")]
    MalformedItem(String),
    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),
}
