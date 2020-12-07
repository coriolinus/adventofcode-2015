//! # Day 23: Opening the Turing Lock
//!
//! Little Jane Marie just got her very first computer for Christmas from some unknown benefactor.
//! It comes with instructions and an example program, but the computer itself seems to be
//! malfunctioning. She's curious what the program does, and would like you to help her run it.
//!
//! The manual explains that the computer supports two registers and six instructions (truly, it
//! goes on to remind the reader, a state-of-the-art technology). The registers are named `a` and
//! `b`, can hold any non-negative integer, and begin with a value of `0`. The instructions are as
//! follows:
//!
//! - `hlf r` sets register `r` to half its current value, then continues with the next instruction.
//! - `tpl r` sets register `r` to triple its current value, then continues with the next instruction.
//! - `inc r` increments register `r`, adding `1` to it, then continues with the next instruction.
//! - `jmp offset` is a jump; it continues with the instruction `offset` away relative to itself.
//! - `jie r, offset` is like `jmp`, but only jumps if register `r` is even ("jump if even").
//! - `jio r, offset` is like `jmp`, but only jumps if register `r` is 1 ("jump if one", not odd).
//!
//! All three jump instructions work with an offset relative to that instruction. The offset is
//! always written with a prefix `+` or `-` to indicate the direction of the jump (forward or
//! backward, respectively). For example, `jmp +1` would simply continue with the next instruction,
//! while `jmp +0` would continuously jump back to itself forever.
//!
//! The program exits when it tries to run an instruction beyond the ones defined.
//!
//! For example, this program sets `a` to `2`, because the jio instruction causes it to skip the
//! `tpl` instruction:
//!
//! ```notrust
//! inc a
//! jio a, +2
//! tpl a
//! inc a
//! ```

use util::parse::{ParseError, Parser};

/// The registers are named `a` and `b`, and can hold any non-negative integer
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Register {
    A,
    B,
}

impl Register {
    fn val(&self) -> usize {
        match self {
            &Register::A => 0,
            &Register::B => 1,
        }
    }
}

pub type Offset = isize;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Instruction {
    /// `hlf r` sets register `r` to half its current value, then continues with the next instruction.
    Hlf(Register),
    /// `tpl r` sets register `r` to triple its current value, then continues with the next instruction.
    Tpl(Register),
    /// `inc r` increments register `r`, adding `1` to it, then continues with the next instruction.
    Inc(Register),
    /// `jmp offset` is a jump; it continues with the instruction `offset` away relative to itself.
    Jmp(Offset),
    /// `jie r, offset` is like `jmp`, but only jumps if register `r` is even ("jump if even").
    Jie(Register, Offset),
    /// `jio r, offset` is like `jmp`, but only jumps if register `r` is 1 ("jump if one", not odd).
    Jio(Register, Offset),
}

impl Instruction {
    fn parse_reg(s: &str) -> Result<Register, ParseError> {
        match s {
            "a" => Ok(Register::A),
            "b" => Ok(Register::B),
            _ => {
                println!("Invalid register name: {}", s);
                Err(ParseError::ConsumerError)
            }
        }
    }

    fn parse_offset(s: &str) -> Result<Offset, ParseError> {
        let o = Offset::from_str_radix(s, 10);
        if let Ok(n) = o {
            Ok(n)
        } else {
            println!("Couldn't interpret {} as Offset", s);
            Err(ParseError::ConsumerError)
        }
    }

    fn parse_reg_and_offset(s: Vec<String>) -> Result<(Register, Offset), ParseError> {
        if s.len() != 3 {
            println!("Wrong number of tokens passed to parse_reg_and_offset");
            Err(ParseError::ConsumerError)
        } else {
            let r = Instruction::parse_reg(&s[1])?;
            let o = Instruction::parse_offset(&s[2])?;
            Ok((r, o))
        }
    }

    pub fn parse(s: &str) -> Result<Instruction, ParseError> {
        let result = Parser::default()
            .require_at_least(Some(2))
            .require_fewer_than(Some(4))
            .clear_trailing_punctuation(true)
            .parse(s)?;

        match result.tokens.first().unwrap_or(&String::new()).as_ref() {
            "hlf" => Ok(Instruction::Hlf(Instruction::parse_reg(&result.tokens[1])?)),
            "tpl" => Ok(Instruction::Tpl(Instruction::parse_reg(&result.tokens[1])?)),
            "inc" => Ok(Instruction::Inc(Instruction::parse_reg(&result.tokens[1])?)),
            "jmp" => Ok(Instruction::Jmp(Instruction::parse_offset(
                &result.tokens[1],
            )?)),
            "jie" => {
                let (r, o) = Instruction::parse_reg_and_offset(result.tokens)?;
                Ok(Instruction::Jie(r, o))
            }
            "jio" => {
                let (r, o) = Instruction::parse_reg_and_offset(result.tokens)?;
                Ok(Instruction::Jio(r, o))
            }
            _ => {
                println!("Choked on invalid token");
                Err(ParseError::ConsumerError)
            }
        }
    }

    pub fn parse_lines(instructions: &str) -> Result<Vec<Instruction>, ParseError> {
        let mut ret = Vec::new();
        for line in instructions.split("\n") {
            if !line.trim().is_empty() {
                if let Ok(inst) = Instruction::parse(line) {
                    ret.push(inst);
                } else {
                    println!("Couldn't parse non-empty line: '{}'", line.trim());
                    return Err(ParseError::ConsumerError);
                }
            }
        }
        Ok(ret)
    }
}

pub struct CPU {
    registers: [u64; 2],
    instructions: Vec<Instruction>,
    ip: isize,
}

impl Default for CPU {
    fn default() -> CPU {
        CPU {
            registers: [0, 0],
            instructions: Vec::new(),
            ip: 0,
        }
    }
}

impl CPU {
    pub fn from_instructions(instructions: Vec<Instruction>) -> CPU {
        let mut ret = CPU::default();
        ret.load(instructions);
        ret
    }

    pub fn get(&self, r: Register) -> u64 {
        self.registers[r.val()]
    }

    pub fn set(&mut self, r: Register, v: u64) {
        self.registers[r.val()] = v;
    }

    /// `hlf r` sets register `r` to half its current value, then continues with the next instruction.
    pub fn hlf(&mut self, r: Register) {
        self.registers[r.val()] = self.registers[r.val()] / 2;
        self.ip += 1;
    }

    /// `tpl r` sets register `r` to triple its current value, then continues with the next instruction.
    pub fn tpl(&mut self, r: Register) {
        self.registers[r.val()] = self.registers[r.val()] * 3;
        self.ip += 1;
    }

    /// `inc r` increments register `r`, adding `1` to it, then continues with the next instruction.
    pub fn inc(&mut self, r: Register) {
        self.registers[r.val()] = self.registers[r.val()] + 1;
        self.ip += 1;
    }

    /// `jmp offset` is a jump; it continues with the instruction `offset` away relative to itself.
    pub fn jmp(&mut self, offset: Offset) {
        self.ip += offset;
    }

    /// `jie r, offset` is like `jmp`, but only jumps if register `r` is even ("jump if even").
    pub fn jie(&mut self, r: Register, offset: Offset) {
        if self.get(r) % 2 == 0 {
            self.ip += offset;
        } else {
            self.ip += 1;
        }
    }

    /// `jio r, offset` is like `jmp`, but only jumps if register `r` is 1 ("jump if one", not odd).
    pub fn jio(&mut self, r: Register, offset: Offset) {
        if self.get(r) == 1 {
            self.ip += offset;
        } else {
            self.ip += 1;
        }
    }

    /// Run the program until the instruction pointer goes beyond the range of the instruction set
    pub fn run(&mut self) {
        while self.ip >= 0 && (self.ip as usize) < self.instructions.len() {
            match self.instructions[self.ip as usize] {
                Instruction::Hlf(r) => self.hlf(r),
                Instruction::Tpl(r) => self.tpl(r),
                Instruction::Inc(r) => self.inc(r),
                Instruction::Jmp(o) => self.jmp(o),
                Instruction::Jie(r, o) => self.jie(r, o),
                Instruction::Jio(r, o) => self.jio(r, o),
            }
        }
    }

    /// Reset the computer to the initial state without modifying the instruction set
    pub fn reset(&mut self) {
        self.ip = 0;
        self.registers = [0, 0];
    }

    /// Load a new program into the computer and configure it to start.
    ///
    /// Note that the program is widely referred to as the instruction set.
    pub fn load(&mut self, instructions: Vec<Instruction>) {
        self.instructions = instructions;
        self.reset();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_lines() -> String {
        let mut ret = String::new();
        ret.push_str("inc a\n");
        ret.push_str("jio a, +2\n");
        ret.push_str("tpl a\n");
        ret.push_str("inc a\n");
        ret
    }

    #[test]
    fn test_example_parses() {
        let insts = Instruction::parse_lines(&get_example_lines());
        println!("Instructions: {:?}", insts);
        assert!(insts.is_ok());
    }

    #[test]
    fn test_example() {
        let insts = Instruction::parse_lines(&get_example_lines()).unwrap();
        let mut cpu = CPU::from_instructions(insts);
        cpu.run();
        assert_eq!(cpu.get(Register::A), 2);
    }
}
