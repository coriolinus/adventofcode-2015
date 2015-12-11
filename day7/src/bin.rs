extern crate util;
use util::get_multiline_input;

extern crate day7lib;
use day7lib::{parse_wires, evaluate};

fn main() {
    let input = get_multiline_input("Your EOF-terminated lines go here:\n").unwrap();
    if let Some(wires) = parse_wires(&input) {
        let sym_table = evaluate(&wires);
        if let Some(val) = sym_table.get("a") {
            println!("Value of wire 'a': {}", val);
        } else {
            println!("Symbol 'a' did not appear in the final table!");
        }
    } else {
        println!("Could not parse that input");
    }
}
