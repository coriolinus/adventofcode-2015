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

use std::{ops::AddAssign, path::Path};

type Pointer = i32;

/// The registers are named `a` and `b`, and can hold any non-negative integer
#[derive(PartialEq, Eq, Clone, Copy, Debug, parse_display::Display, parse_display::FromStr)]
#[display(style = "snake_case")]
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

#[derive(PartialEq, Eq, Clone, Copy, Debug, parse_display::Display, parse_display::FromStr)]
pub enum Direction {
    #[display("+")]
    Forward,
    #[display("-")]
    Back,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, parse_display::Display, parse_display::FromStr)]
#[display("{direction}{distance}")]
#[from_str(regex = r"(?P<direction>.)(?P<distance>\d+)")]
pub struct Offset {
    direction: Direction,
    distance: Pointer,
}

impl AddAssign<Offset> for Pointer {
    fn add_assign(&mut self, rhs: Offset) {
        match rhs.direction {
            Direction::Forward => *self += rhs.distance,
            Direction::Back => *self -= rhs.distance,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, parse_display::Display, parse_display::FromStr)]
#[display(style = "snake_case")]
pub enum Instruction {
    /// `hlf r` sets register `r` to half its current value, then continues with the next instruction.
    #[display("{} {0}")]
    Hlf(Register),
    /// `tpl r` sets register `r` to triple its current value, then continues with the next instruction.
    #[display("{} {0}")]
    Tpl(Register),
    /// `inc r` increments register `r`, adding `1` to it, then continues with the next instruction.
    #[display("{} {0}")]
    Inc(Register),
    /// `jmp offset` is a jump; it continues with the instruction `offset` away relative to itself.
    #[display("{} {0}")]
    Jmp(Offset),
    /// `jie r, offset` is like `jmp`, but only jumps if register `r` is even ("jump if even").
    #[display("{} {0}, {1}")]
    Jie(Register, Offset),
    /// `jio r, offset` is like `jmp`, but only jumps if register `r` is 1 ("jump if one", not odd).
    #[display("{} {0}, {1}")]
    Jio(Register, Offset),
}

#[derive(Default)]
pub struct Cpu {
    registers: [u64; 2],
    instructions: Vec<Instruction>,
    ip: Pointer,
}

impl Cpu {
    pub fn from_instructions(instructions: Vec<Instruction>) -> Cpu {
        Cpu {
            instructions,
            ..Cpu::default()
        }
    }

    pub fn get(&self, r: Register) -> u64 {
        self.registers[r.val()]
    }

    pub fn set(&mut self, r: Register, v: u64) {
        self.registers[r.val()] = v;
    }

    /// `hlf r` sets register `r` to half its current value, then continues with the next instruction.
    fn hlf(&mut self, r: Register) {
        self.registers[r.val()] = self.registers[r.val()] / 2;
        self.ip += 1;
    }

    /// `tpl r` sets register `r` to triple its current value, then continues with the next instruction.
    fn tpl(&mut self, r: Register) {
        self.registers[r.val()] = self.registers[r.val()] * 3;
        self.ip += 1;
    }

    /// `inc r` increments register `r`, adding `1` to it, then continues with the next instruction.
    fn inc(&mut self, r: Register) {
        self.registers[r.val()] = self.registers[r.val()] + 1;
        self.ip += 1;
    }

    /// `jmp offset` is a jump; it continues with the instruction `offset` away relative to itself.
    fn jmp(&mut self, offset: Offset) {
        self.ip += offset;
    }

    /// `jie r, offset` is like `jmp`, but only jumps if register `r` is even ("jump if even").
    fn jie(&mut self, r: Register, offset: Offset) {
        if self.get(r) % 2 == 0 {
            self.ip += offset;
        } else {
            self.ip += 1;
        }
    }

    /// `jio r, offset` is like `jmp`, but only jumps if register `r` is 1 ("jump if one", not odd).
    fn jio(&mut self, r: Register, offset: Offset) {
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
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let instructions = aoclib::parse(input)?.collect();
    let mut cpu = Cpu::from_instructions(instructions);
    cpu.run();
    println!("Terminating with register B = '{}'", cpu.get(Register::B));
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let instructions = aoclib::parse(input)?.collect();
    let mut cpu = Cpu::from_instructions(instructions);
    cpu.set(Register::A, 1);
    cpu.run();
    println!("Terminating with register B = '{}'", cpu.get(Register::B));
    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"
inc a
jio a, +2
tpl a
inc a
"#;

    #[test]
    fn test_example_parses() {
        let insts: Vec<Instruction> = aoclib::input::parse_str(EXAMPLE.trim()).unwrap().collect();
        println!("Instructions: {:?}", insts);
        assert_eq!(insts.len(), 4);
    }

    #[test]
    fn test_example() {
        let insts: Vec<Instruction> = aoclib::input::parse_str(EXAMPLE.trim()).unwrap().collect();
        let mut cpu = Cpu::from_instructions(insts);
        cpu.run();
        assert_eq!(cpu.get(Register::A), 2);
    }
}
