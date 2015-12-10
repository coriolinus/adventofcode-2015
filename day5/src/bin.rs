extern crate util;
use util::get_multiline_input;

extern crate day5lib;
use day5lib::count_nice;

fn main() {
    let nice = count_nice(&get_multiline_input("Your EOF-terminated lines go here:\n").unwrap());
    println!("Nice: {}", nice);
}
