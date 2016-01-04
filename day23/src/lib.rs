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
    pub fn get(&self, r: Register) -> u64 {
        self.registers[r.val()]
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

    pub fn reset(&mut self) {
        self.ip = 0;
        self.registers = [0, 0];
    }

    pub fn load(&mut self, instructions: Vec<Instruction>) {
        self.instructions = instructions;
        self.reset();
    }
}
