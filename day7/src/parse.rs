//! Tools for parsing the given instructions into a data structure mapping them.




use std::collections::HashSet;


lazy_static! {
    static ref LETTERS: HashSet<char> = {
        let mut letters = HashSet::new();
        for letter in "abcdefghijklmnopqrstuvwxyz".chars() {
            letters.insert(letter);
        }
        letters
    };

    static ref NUMBERS: HashSet<char> = {
        let mut numbers = HashSet::new();
        for number in "1234567890".chars() {
            numbers.insert(number);
        }
        numbers
    };
}

fn is_just_letters(s: &str) -> bool {
    for c in s.chars() {
        if !LETTERS.contains(&c) {
            return false;
        }
    }
    true
}

fn is_just_numbers(s: &str) -> bool {
    for c in s.chars() {
        if !NUMBERS.contains(&c) {
            return false;
        }
    }
    true
}

pub enum Instruction {
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
    // Unary prefix
    Not {
        x: Evaluable,
    },
    // Nonary (implied)
    Store {
        x: Evaluable,
    },
}

pub fn parse_instruction(inst: &Vec<&str>) -> Option<Instruction> {
    match inst.len() {
        1 => parse_nonary_instruction(inst[0]), // nonary instruction (direct assignment)
        2 => parse_unary_instruction(inst[0], inst[1]), // unary instruction
        3 => parse_binary_instruction(inst[0], inst[1], inst[2]), // binary instruction
        _ => None,
    }
}

fn parse_nonary_instruction(x: &str) -> Option<Instruction> {
    // the only nonary instruction is Store, so that's easy
    let ev = parse_evaluable(x);
    if ev.is_none() {
        None
    } else {
        Some(Instruction::Store { x: ev.unwrap() })
    }
}

fn parse_unary_instruction(inst: &str, val: &str) -> Option<Instruction> {
    // the only unary instruction is "Not"
    if inst != "not" {
        return None;
    }
    let ev = parse_evaluable(val);
    if ev.is_none() {
        None
    } else {
        Some(Instruction::Not { x: ev.unwrap() })
    }
}

fn parse_binary_instruction(x: &str, inst: &str, y: &str) -> Option<Instruction> {
    // there are four binary instructions: and, or, lshift, rshift.
    // before we match against them, though, it's cleaner to make sure the other args
    //   are all evaluable as well.

    let x = parse_evaluable(x);
    if x.is_none() {
        return None;
    }
    let x = x.unwrap();

    let y = parse_evaluable(y);
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

fn parse_evaluable(ev: &str) -> Option<Evaluable> {
    if is_just_letters(ev) {
        let n = Name::new(ev);
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

enum Evaluable {
    Num(u16),
    Name(Name),
}

pub struct Name {
    val: String,
}

impl Name {
    pub fn new(v: &str) -> Option<Name> {
        if is_just_letters(v) {
            Some(Name { val: v.to_string() })
        } else {
            None
        }
    }

    pub fn get(&self) -> &str {
        &self.val
    }
}
