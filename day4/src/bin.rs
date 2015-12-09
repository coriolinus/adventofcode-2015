extern crate util;
use util::get_line_input;

fn main() {
    if let Ok(input) = get_line_input("Enter your secret key here: ") {
        println!("What a secret! {}", input);
    }
}
