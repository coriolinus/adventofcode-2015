use util::get_multiline_input;

use day23::{Instruction, Register, CPU};

fn main() {
    let program = get_multiline_input("Input program, EOF-terminated:").unwrap();
    let insts = Instruction::parse_lines(&program);
    if let Ok(insts) = insts {
        let mut cpu = CPU::from_instructions(insts);
        cpu.run();
        println!("Terminating with register B = '{}'", cpu.get(Register::B));
        println!("Resetting and rerunning with register A = '1'");
        cpu.reset();
        cpu.set(Register::A, 1);
        cpu.run();
        println!("Terminating with register B = '{}'", cpu.get(Register::B));
    } else {
        println!("Could not parse your instructions, sorry;")
    }
}
