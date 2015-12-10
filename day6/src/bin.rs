extern crate util;
use util::get_multiline_input;

extern crate day6lib;
use day6lib::Lights;
use day6lib::Lightable;

fn main() {
    let input = get_multiline_input("Your EOF-terminated lines go here:\n").unwrap();
    let mut lights: Lights<bool> = Lights::new();
    lights.parse_lines(&input);
    let count = lights.count();
    println!("Lit: {}", count)
}
