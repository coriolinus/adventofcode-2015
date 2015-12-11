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

#[macro_use]
extern crate lazy_static;

mod parse;

mod instruction;
use instruction::Instruction;

mod wire;
use wire::Wire;
