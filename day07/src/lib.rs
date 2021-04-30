//! # Day 7: Some Assembly Required
//!
//! This year, Santa brought little Bobby Tables a set of wires and bitwise logic gates!
//! Unfortunately, little Bobby is a little under the recommended age range, and he needs help
//! assembling the circuit.
//!
//! Each wire has an identifier (some lowercase letters) and can carry a 16-bit signal (a number
//! from `0` to `65535`). A signal is provided to each wire by a gate, another wire, or some
//! specific value. Each wire can only get a signal from one source, but can provide its signal to
//! multiple destinations. A gate provides no signal until all of its inputs have a signal.
//!
//! The included instructions booklet describes how to connect the parts together: `x AND y -> z`
//! means to connect wires `x` and `y` to an `AND` gate, and then connect its output to wire `z`.
//!
//! # For example:
//!
//! - `123 -> x` means that the signal `123` is provided to wire `x`.
//! - `x AND y -> z` means that the bitwise `AND` of wire `x` and wire `y` is provided to wire `z`.
//! - `p LSHIFT 2 -> q` means that the value from wire `p` is left-shifted by `2` and then provided
//!   to wire `q`.
//! - `NOT e -> f` means that the bitwise complement of the value from wire `e` is provided to wire
//!   `f`.
//! - Other possible gates include `OR` (bitwise OR) and `RSHIFT` (right-shift). If, for some
//!   reason, you'd like to emulate the circuit instead, almost all programming languages (for
//!   example, C, JavaScript, or Python) provide operators for these gates.

use aoclib::parse;
use lalrpop_util::lalrpop_mod;
use std::collections::{HashMap, HashSet};
use std::{path::Path, str::FromStr};
use thiserror::Error;

lalrpop_mod!(parser);

type Signals = HashMap<String, u16>;

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub enum Signal {
    Literal(u16),
    Reference(String),
}

impl Signal {
    fn value(&self, signals: &Signals) -> Option<u16> {
        match self {
            Self::Literal(l) => Some(*l),
            Self::Reference(r) => signals.get(r).copied(),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub enum Instruction {
    Copy(Signal),
    And(Signal, Signal),
    Or(Signal, Signal),
    LShift(Signal, Signal),
    RShift(Signal, Signal),
    Not(Signal),
}

impl Instruction {
    fn value(&self, signals: &Signals) -> Option<u16> {
        match self {
            Self::Copy(x) => x.value(signals),
            Self::And(x, y) => Some(x.value(signals)? & y.value(signals)?),
            Self::Or(x, y) => Some(x.value(signals)? | y.value(signals)?),
            Self::LShift(x, y) => Some(x.value(signals)? << y.value(signals)?),
            Self::RShift(x, y) => Some(x.value(signals)? >> y.value(signals)?),
            Self::Not(x) => Some(!x.value(signals)?),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub struct Wire {
    pub(crate) instruction: Instruction,
    pub(crate) destination: String,
}

impl Wire {
    /// Try to apply this wire's value to the signal table.
    ///
    /// Return `true` when the application was successful.
    fn try_apply(&self, signals: &mut Signals) -> bool {
        if signals.contains_key(&self.destination) {
            return true;
        }
        match self.instruction.value(signals) {
            Some(value) => {
                signals.insert(self.destination.clone(), value);
                true
            }
            None => false,
        }
    }
}

impl FromStr for Wire {
    type Err = lalrpop_util::ParseError<usize, String, &'static str>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parser = parser::WireParser::new();
        parser
            .parse(s)
            .map_err(|err| err.map_token(|t| t.to_string()))
    }
}

pub fn compute_all_signals(mut wires: HashSet<Wire>, mut signals: Signals) -> Signals {
    let mut pending_wires = HashSet::with_capacity(wires.len());
    let mut prev_wires_len = 0;

    while wires.len() != prev_wires_len && !wires.is_empty() {
        prev_wires_len = wires.len();

        for wire in wires.drain() {
            if !wire.try_apply(&mut signals) {
                pending_wires.insert(wire);
            }
        }

        std::mem::swap(&mut wires, &mut pending_wires);
    }

    assert_eq!(wires.len(), 0, "failed to compute a signal for every wire");
    signals
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let wires: HashSet<Wire> = parse(input)?.collect();
    let signals = Signals::with_capacity(wires.len());
    let signals = compute_all_signals(wires, signals);
    println!("value of 'a' wire (pt. 1): {:?}", signals.get("a"));

    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let wires: HashSet<Wire> = parse(input)?.collect();
    let signals = Signals::with_capacity(wires.len());
    let signals = compute_all_signals(wires.clone(), signals);
    let a_value = signals["a"];
    let mut signals = Signals::with_capacity(wires.len());
    signals.insert("b".to_string(), a_value);
    let signals = compute_all_signals(wires, signals);
    println!("value of 'a' wire (pt. 2): {:?}", signals.get("a"));

    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
