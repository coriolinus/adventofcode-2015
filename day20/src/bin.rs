extern crate util;
use util::get_line_input;

extern crate day20lib;
use day20lib::first_house_with_n_presents;

fn main() {
    let presents_s = get_line_input("Find the first house with this many presents or more: ")
                         .unwrap();

    if let Ok(presents) = usize::from_str_radix(&presents_s.trim(), 10) {
        println!("Dispatching elves...");
        let n = first_house_with_n_presents(presents);
        println!("First house with {} presents: {}", presents, n);
    } else {
        println!("Couldn't parse your input, sorry")
    }
}
