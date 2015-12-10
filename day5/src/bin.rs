extern crate util;
use util::get_multiline_input;

extern crate day5lib;
use day5lib::count_nice;
use day5lib::count_nice2;

fn main() {
    let input = get_multiline_input("Your EOF-terminated lines go here:\n").unwrap();
    let nice = count_nice(&input);
    println!("Nice: {}", nice);
    let nice2 = count_nice2(&input);
    println!("Nice2: {}", nice2);
}
