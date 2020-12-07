//! # Wire
//! Named variable

use super::name::Name;
use super::parse::Parseable;

use super::instruction::Instruction;

#[derive(Clone)]
pub struct Wire {
    pub name: Name,
    instruction: Instruction,
}

impl Wire {
    pub fn get_name(&self) -> &str {
        self.name.get()
    }

    pub fn get_instruction(&self) -> &Instruction {
        &self.instruction
    }
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

        let inst = Instruction::parse(&inst);
        if inst.is_none() {
            return None;
        }
        let inst = inst.unwrap();

        Some(Wire {
            name: name,
            instruction: inst,
        })
    }
}
