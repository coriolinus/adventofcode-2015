extern crate util;
use util::get_multiline_input;

extern crate day6lib;
use day6lib::Lights;

fn main() {
    let input = get_multiline_input("Your EOF-terminated lines go here:\n").unwrap();
    let output = Lights::parse_lines(&input).count();
    println!("Lit: {}", output)
}
