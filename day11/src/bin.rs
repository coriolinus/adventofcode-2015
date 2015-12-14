extern crate util;
use util::get_line_input;

extern crate day11lib;
use day11lib::next_pw;

fn main() {
    let oldpw = get_line_input("Old pw: ").unwrap();
    let oldpw = oldpw.trim();
    let next = next_pw(&oldpw);
    println!("Next: {}", next);
    let next = next_pw(&next);
    println!("Next: {}", next);
}
