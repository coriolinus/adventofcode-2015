//! # Instruction
//! What do do in this mini-computer


use super::evaluable::Evaluable;
use super::parse::Parseable;

#[derive(Clone)]
pub enum Instruction {
    // Nonary (implied)
    Store(Evaluable),

    // Unary prefix
    Not(Evaluable),

    // Binary infix
    And {
        x: Evaluable,
        y: Evaluable,
    },
    Or {
        x: Evaluable,
        y: Evaluable,
    },
    Lshift {
        x: Evaluable,
        y: Evaluable,
    },
    Rshift {
        x: Evaluable,
        y: Evaluable,
    },
}

pub enum InstructionType {
    Nonary,
    Unary,
    Binary,
}

impl Instruction {
    pub fn parse(inst: &Vec<&str>) -> Option<Instruction> {
        match inst.len() {
            1 => parse_nonary_instruction(inst[0]), // nonary instruction (direct assignment)
            2 => parse_unary_instruction(inst[0], inst[1]), // unary instruction
            3 => parse_binary_instruction(inst[0], inst[1], inst[2]), // binary instruction
            _ => None,
        }
    }

    pub fn get_type(&self) -> InstructionType {
        match &self {
            &&Instruction::Store(_) => InstructionType::Nonary,
            &&Instruction::Not(_) => InstructionType::Unary,
            &&Instruction::And{ref x,ref y} => InstructionType::Binary,
            &&Instruction::Or{ref x,ref y} => InstructionType::Binary,
            &&Instruction::Lshift{ref x,ref y} => InstructionType::Binary,
            &&Instruction::Rshift{ref x,ref y} => InstructionType::Binary,
        }
    }
}

fn parse_nonary_instruction(x: &str) -> Option<Instruction> {
    // the only nonary instruction is Store, so that's easy
    let ev = Evaluable::parse(x);
    if ev.is_none() {
        None
    } else {
        Some(Instruction::Store(ev.unwrap()))
    }
}

fn parse_unary_instruction(inst: &str, val: &str) -> Option<Instruction> {
    // the only unary instruction is "Not"
    if inst != "not" {
        return None;
    }
    let ev = Evaluable::parse(val);
    if ev.is_none() {
        None
    } else {
        Some(Instruction::Not(ev.unwrap()))
    }
}

fn parse_binary_instruction(x: &str, inst: &str, y: &str) -> Option<Instruction> {
    // there are four binary instructions: and, or, lshift, rshift.
    // before we match against them, though, it's cleaner to make sure the other args
    //   are all evaluable as well.

    let x = Evaluable::parse(x);
    if x.is_none() {
        return None;
    }
    let x = x.unwrap();

    let y = Evaluable::parse(y);
    if y.is_none() {
        return None;
    }
    let y = y.unwrap();

    match inst {
        "and" => Some(Instruction::And { x: x, y: y }),
        "or" => Some(Instruction::Or { x: x, y: y }),
        "lshift" => Some(Instruction::Lshift { x: x, y: y }),
        "rshift" => Some(Instruction::Rshift { x: x, y: y }),
        _ => None,
    }
}
