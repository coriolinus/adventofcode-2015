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


use std::collections::{HashMap, HashSet};

pub mod countdistinct;
use countdistinct::CountDistinct;

pub fn parse_replacements(lines: &str) -> Option<HashMap<String, Vec<String>>> {
    let mut ret = HashMap::new();
    for line in lines.split("\n") {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if line.split("=>").count() != 2 {
            // can't break this line in half
            // who knows what craziness has occurred
            // malformed input, anyway
            return None;
        }

        let mut splitter = line.split("=>");
        let from = splitter.next().unwrap().trim().to_string();
        let to = splitter.next().unwrap().trim().to_string();

        if ret.contains_key(&from) {
            let mut val_vec: &mut Vec<String> = ret.get_mut(&from).unwrap();
            val_vec.push(to);
        } else {
            ret.insert(from, vec![to]);
        }
    }
    Some(ret)
}



/// An Iterator over simple transformations of a given string.
///
/// Given a String to transform from, a String to transform to, and an input, the ChemTransformer
/// iterates over instances of matches of `from` in `input`. Each Item in this sequence is the
/// input, with that particular match of `from` replaced with `to`.
#[derive(PartialEq, Eq, Clone)]
pub struct ChemTransformer {
    from: String,
    to: String,
    chunks: Vec<String>,
    repl_index: usize,
}

impl ChemTransformer {
    pub fn new(trans_from: String, trans_to: String, replace_item: String) -> ChemTransformer {
        ChemTransformer {
            chunks: replace_item.split(&trans_from).map(|s| s.to_string()).collect::<Vec<String>>(),
            from: trans_from,
            to: trans_to,
            repl_index: 0,
        }
    }
}

impl Iterator for ChemTransformer {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        // Situation: we have divided our input string into N chunks. Between each chunk, we insert
        // either our `to` item or replace the `from` item it came from.
        //
        // Trivial example: from = "O", to = "HH", input = "HOH", chunks = ["H", "H"]
        // We should return exactly one result, then None forevermore.
        // Our result: "HHHH"
        //
        // Slightly non-trivial example:
        // from = "H", to = "HO", input = "HOH"
        // chunks = ["", "O", ""]
        // Results: "HOOH", "HOHO"
        if self.repl_index < self.chunks.len() - 1 {
            self.repl_index += 1;
        } else {
            return None;
        }

        let mut ret = "".to_string();
        for (i, chunk) in self.chunks.iter().enumerate() {
            // don't emit filler before the first character
            if i != 0 {
                ret.push_str(if i == self.repl_index {
                    &self.to
                } else {
                    &self.from
                });
            }
            ret.push_str(&chunk);
        }
        Some(ret)
    }
}

#[derive(Clone)]
pub struct TransformEnumerator<'a> {
    transform_iter: std::collections::hash_map::Iter<'a, String, Vec<String>>,
    from: Option<String>,
    tos: Option<std::slice::Iter<'a, String>>,
    input: String,
    ct: Option<ChemTransformer>,
}

impl<'a> TransformEnumerator<'a> {
    pub fn new<'t>(transforms: &'t HashMap<String, Vec<String>>,
                   input: &str)
                   -> TransformEnumerator<'t> {
        TransformEnumerator {
            from: None,
            tos: None,
            ct: None,
            input: input.to_string(),
            transform_iter: transforms.iter(),
        }
    }
}

impl<'a> Iterator for TransformEnumerator<'a> {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        // to begin: if we already have a working ChemTransformer, see if we can just return
        // its next item. If it's run out, we can reset and keep going.
        if self.ct.is_some() {
            let next = self.ct.as_mut().unwrap().next();
            if next.is_some() {
                return next; // Some(str)
            } else {
                // self.ct = None;
                // The above is an unnecessary assignment; we just continue and reset it anyway
                // in the following lines.
            }
        }

        // Our ChemTransformer either ran out or never started, so let's get the materials with
        // which to build the next one.
        if self.from.is_none() {
            let it_next = self.transform_iter.next();
            if it_next.is_none() {
                return None;
            } else {
                self.from = Some(it_next.unwrap().0.clone());
                self.tos = Some(it_next.unwrap().1.iter());
            }
        }
        // once we get here, `self.tos` is never None
        let cur_to = self.tos.as_mut().unwrap().next();
        if cur_to.is_none() {
            // go to the next pair
            self.from = None;
            return self.next();
        }
        let cur_to = cur_to.unwrap();
        self.ct = Some(ChemTransformer::new(self.from.as_mut().unwrap().clone(),
                                            cur_to.to_owned(),
                                            self.input.clone()));
        return self.next();
    }
}

impl<'a> CountDistinct for TransformEnumerator<'a> {}

/// Generate a target string by applying a sequence of string transformations to the single
/// character `e`. Allowable string transformations are given in the parameter `transforms`.
/// Target to generate given in the parameter `target`.
///
/// Returns `Vec<String>`, containing all mutations on the way to the target
pub fn fabricate(transforms: &HashMap<String, Vec<String>>, target: &str) -> Option<Vec< String>> {
    // to_examine: a list of tuples:
    // (next, history)
    // where next is simply the next thing to try,
    // and history is how we got there: a list of strings.
    let mut to_examine : Vec<(String, Vec<String>)> = vec![("e".to_string(), Vec::new())];
    let mut tried = HashSet::new();

    while to_examine.len() > 0 {
        let (ex, mut history) = to_examine.remove(0);
        history.push(ex.clone());
        if ex == target {
            return Some(history);
        }
        if tried.insert(ex.clone()) {
            // `.insert()` returns true if the value was not already present
            for mutation in TransformEnumerator::new(transforms, &ex).filter(|m| !tried.contains(m)) {
                to_examine.push((mutation, history.clone()));
            }
        }
    }
    None
}

/// Generate a target string by applying a sequence of string transformations to the single
/// character `e`. Allowable string transformations are given in the parameter `transforms`.
/// Target to generate given in the parameter `target`.
///
/// Returns `usize`, the number of steps after `e` to generate the target
pub fn fabricate_steps_count(transforms: &HashMap<String, Vec<String>>, target: &str) -> Option<usize> {
    // to_examine: a list of tuples:
    // (next, history)
    // where next is simply the next thing to try,
    // and history is how we got there: a list of strings.
    let mut to_examine : Vec<(String, usize)> = vec![("e".to_string(), 0)];
    let mut tried = HashSet::new();

    while to_examine.len() > 0 {
        let (ex, mut history) = to_examine.remove(0);
        if ex == target {
            return Some(history);
        }
        history += 1;
        if tried.insert(ex.clone()) {
            // `.insert()` returns true if the value was not already present
            for mutation in TransformEnumerator::new(transforms, &ex).filter(|m| !tried.contains(m)) {
                // this depends on the fact that no mutation shortens the overall length of the string.
                // That may not always be the case! Be careful with that!
                if mutation.len() <= target.len() {
                to_examine.push((mutation, history));
            }}
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::*;
    use super::countdistinct::CountDistinct;

    #[test]
    fn test_chem_transformer() {
        let tests = vec![("O", "HH", "HOH", vec!["HHHH"]),
                         ("H", "HO", "HOH", vec!["HOOH", "HOHO"])];

        for (from, to, input, expect) in tests {
            let from = from.to_string();
            let to = to.to_string();
            let input = input.to_string();
            let expect = expect.iter().map(|s| s.to_string()).collect::<Vec<String>>();

            let ct = ChemTransformer::new(from, to, input);
            assert_eq!(ct.clone().count(), expect.len());
            for (result, exp) in ct.zip(expect) {
                assert_eq!(result, exp);
            }
        }
    }

    #[test]
    fn test_transform_enumerator() {
        let rep = get_default_replacements();
        let input = "HOH";
        let te = TransformEnumerator::new(&rep, input);

        assert_eq!(te.clone().count(), 5);
        assert_eq!(te.clone().count_distinct(), 4);
    }

    fn get_default_replacements() -> HashMap<String, Vec<String>> {
        let mut lines = "".to_string();
        lines.push_str("H => HO\n");
        lines.push_str("H => OH\n");
        lines.push_str("O => HH\n");

        parse_replacements(&lines).unwrap()
    }

    #[test]
    fn test_parse_replacements() {
        let rep = get_default_replacements();

        assert!(rep.get("H").is_some());
        assert!(rep.get("O").is_some());

        assert_eq!(rep.get("H").unwrap().len(), 2);
        assert_eq!(rep.get("O").unwrap().len(), 1);
    }

    #[test]
    fn test_part_2_examples() {
        let mut lines = "".to_string();
        lines.push_str("e => H\n");
        lines.push_str("e => O\n");
        lines.push_str("H => HO\n");
        lines.push_str("H => OH\n");
        lines.push_str("O => HH\n");

        let transforms = parse_replacements(&lines).unwrap();

        println!("Fabricating 'e'...");
        assert_eq!(fabricate(&transforms, "e").unwrap(), vec!["e"]);

        println!("Fabricating 'HOH'...");
        assert_eq!(fabricate(&transforms, "HOH").unwrap(), vec!["e", "O", "HH", "HOH"]);

        println!("Fabricating 'HOHOHO'...");
        assert_eq!(fabricate(&transforms, "HOHOHO").unwrap().len(), 7);
    }

    #[test]
    fn test_fabricate_steps_count() {
        let mut lines = "".to_string();
        lines.push_str("e => H\n");
        lines.push_str("e => O\n");
        lines.push_str("H => HO\n");
        lines.push_str("H => OH\n");
        lines.push_str("O => HH\n");

        let transforms = parse_replacements(&lines).unwrap();

        println!("Fabricating 'e'...");
        assert_eq!(fabricate_steps_count(&transforms, "e").unwrap(), 0);

        println!("Fabricating 'HOH'...");
        assert_eq!(fabricate_steps_count(&transforms, "HOH").unwrap(), 3);

        println!("Fabricating 'HOHOHO'...");
        assert_eq!(fabricate_steps_count(&transforms, "HOHOHO").unwrap(), 6);
    }
}
