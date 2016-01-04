extern crate util;
use util::get_multiline_input;

extern crate day23lib;
use day23lib::{Instruction, CPU, Register};

fn main() {
    let program = get_multiline_input("Input program, EOF-terminated:").unwrap();
    let insts = Instruction::parse_lines(&program);
    if let Ok(insts) = insts {
        let mut cpu = CPU::from_instructions(insts);
        cpu.run();
        println!("Terminating with register B = '{}'", cpu.get(Register::B));
    } else {
        println!("Could not parse your instructions, sorry;")
    }
}
