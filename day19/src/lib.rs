//! # Day 19: Medicine for Rudolph
//!
//! Rudolph the Red-Nosed Reindeer is sick! His nose isn't shining very brightly, and he needs
//! medicine.
//!
//! Red-Nosed Reindeer biology isn't similar to regular reindeer biology; Rudolph is going to need
//! custom-made medicine. Unfortunately, Red-Nosed Reindeer chemistry isn't similar to regular
//! reindeer chemistry, either.
//!
//! The North Pole is equipped with a Red-Nosed Reindeer nuclear fusion/fission plant, capable of
//! constructing any Red-Nosed Reindeer molecule you need. It works by starting with some input
//! molecule and then doing a series of replacements, one per step, until it has the right molecule.
//!
//! However, the machine has to be calibrated before it can be used. Calibration involves
//! determining the number of molecules that can be generated in one step from a given starting
//! point.
//!
//! For example, imagine a simpler machine that supports only the following replacements:
//!
//! ```notrust
//! H => HO
//! H => OH
//! O => HH
//! ```
//!
//! Given the replacements above and starting with HOH, the following molecules could be generated:
//!
//! - `HOOH` (via `H` => `HO` on the first `H`).
//! - `HOHO` (via `H` => `HO` on the second `H`).
//! - `OHOH` (via `H` => `OH` on the first `H`).
//! - `HOOH` (via `H` => `OH` on the second `H`).
//! - `HHHH` (via `O` => `HH`).
//!
//! So, in the example above, there are 4 distinct molecules (not five, because HOOH appears twice)
//! after one replacement from `HOH`. Santa's favorite molecule, `HOHOHO`, can become 7 distinct
//! molecules (over nine replacements: six from `H`, and three from `O`).
//!
//! The machine replaces without regard for the surrounding characters. For example, given the
//! string `H2O`, the transition `H => OO` would result in `OO2O`.
//!
//! Your puzzle input describes all of the possible replacements and, at the bottom, the medicine
//! molecule for which you need to calibrate the machine. How many distinct molecules can be
//! created after all the different ways you can do one replacement on the medicine molecule?

use std::{
    collections::{HashSet, VecDeque},
    convert::TryFrom,
    path::Path,
    str::FromStr,
};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, parse_display::FromStr, parse_display::Display)]
#[display("{from} => {to}")]
struct Replacement {
    from: String,
    to: String,
}

impl Replacement {
    fn from(&self, reverse: bool) -> &str {
        if reverse {
            &self.to
        } else {
            &self.from
        }
    }

    fn to(&self, reverse: bool) -> &str {
        if reverse {
            &self.from
        } else {
            &self.to
        }
    }
}

#[derive(Debug, Clone, Default)]
struct Input {
    replacements: Vec<Replacement>,
    medicine: String,
}

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut input = Input::default();

        let mut lines: Vec<_> = s.split('\n').collect();
        lines.retain(|line| !line.is_empty());

        if lines.len() > 0 {
            input.medicine = lines[lines.len() - 1].to_string();
            for line in &lines[..lines.len() - 1] {
                input.replacements.push(
                    line.trim()
                        .parse()
                        .map_err(|err| Error::Parse(err, line.trim().to_string()))?,
                );
            }
        }

        // for forward production, the ordering of the list of replacements is irrelevant.
        // for reverse searching, we want to greediliy reduce it as fast as possible, so let's
        // reverse-sort by length of `to`
        input
            .replacements
            .sort_unstable_by_key(|replacement| replacement.to.len());
        input.replacements.reverse();

        Ok(input)
    }
}

impl TryFrom<&Path> for Input {
    type Error = Error;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let data = std::fs::read_to_string(path)?;
        data.parse()
    }
}

impl Input {
    fn replace<'a>(&'a self, initial: &'a str) -> impl 'a + Iterator<Item = String> {
        self.replace_inner(initial, false)
    }

    fn replace_inner<'a>(
        &'a self,
        initial: &'a str,
        reverse: bool,
    ) -> impl 'a + Iterator<Item = String> {
        (0..initial.len())
            .filter(move |&index| initial.is_char_boundary(index))
            .map(move |index| {
                let (prefix, suffix) = initial.split_at(index);
                self.replacements
                    .iter()
                    .filter(move |replacement| suffix.starts_with(&replacement.from(reverse)))
                    .map(move |replacement| {
                        let (_, suffix) = suffix.split_at(replacement.from(reverse).len());
                        format!("{}{}{}", prefix, replacement.to(reverse), suffix)
                    })
            })
            .flatten()
    }

    fn single_step_replacements(&self) -> usize {
        self.replace(&self.medicine).collect::<HashSet<_>>().len()
    }

    // working forwards blows up the machine. Let's work the problem backwards.
    fn count_fabrication_steps(&self) -> Option<usize> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((0, self.medicine.clone()));

        while let Some((prior_steps, product)) = queue.pop_front() {
            if !visited.insert(product.clone()) {
                // `insert` returns false if the set already contained the item
                continue;
            }
            if product == "e" {
                return Some(prior_steps);
            }

            queue.extend(
                self.replace_inner(&product, true)
                    .filter(|product| !visited.contains(product))
                    .map(|product| (prior_steps + 1, product)),
            );
        }

        None
    }
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let input = Input::try_from(input)?;
    let ssr = input.single_step_replacements();
    println!("single step replacements: {}", ssr);
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let input = Input::try_from(input)?;
    let fabrication_steps = input.count_fabrication_steps();
    println!("fabrication steps: {:?}", fabrication_steps);
    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("parsing \"{1}\": {0}")]
    Parse(#[source] parse_display::ParseError, String),
}

#[cfg(test)]
mod test {
    use super::*;

    fn part2(input: &str, expect: usize) {
        let input: Input = input.trim().parse().unwrap();
        let fabrication_steps = input.count_fabrication_steps();
        assert_eq!(fabrication_steps, Some(expect));
    }

    #[test]
    fn part2_example_1() {
        part2(
            "
e => H
e => O
H => HO
H => OH
O => HH
HOH
",
            3,
        )
    }

    #[test]
    fn part2_example_2() {
        part2(
            "
e => H
e => O
H => HO
H => OH
O => HH
HOHOHO
",
            6,
        )
    }
}
