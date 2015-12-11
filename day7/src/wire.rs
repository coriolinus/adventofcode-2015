//! # Wire
//! Named variable

use super::parse::Parseable;
use super::parse::Name;

use super::instruction::Instruction;
use super::instruction::parse_instruction;

pub struct Wire {
    name: Name,
    value: Option<u16>,

    instruction: Instruction,
}

impl Parseable for Wire {
    type P = Wire;

    fn parse(input: &str) -> Option<Wire> {
        let input = input.trim().to_lowercase();
        if input.is_empty() {
            return None;
        }

        let mut tokens = input.rsplit(' ');
        let name = tokens.next();
        if name.is_none() {
            return None;
        }
        let name = Name::parse(name.unwrap());
        if name.is_none() {
            return None;
        }
        let name = name.unwrap();

        let arrow = tokens.next();
        if arrow.is_none() {
            return None;
        }
        if arrow.unwrap() != "->" {
            return None;
        }

        let mut inst: Vec<_> = tokens.collect();
        // remember, it's backwards because it was a right iterator
        inst.reverse();

        let inst = parse_instruction(&inst);
        if inst.is_none() {
            return None;
        }
        let inst = inst.unwrap();

        Some(Wire {
            name: name,
            value: None,
            instruction: inst,
        })
    }
}
