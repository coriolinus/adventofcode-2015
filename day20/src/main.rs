use util::get_line_input;

use day20::{first_house_with_n_presents, first_house_with_n_presents_limited};

fn main() {
    let presents_s =
        get_line_input("Find the first house with this many presents or more: ").unwrap();

    if let Ok(presents) = usize::from_str_radix(&presents_s.trim(), 10) {
        println!("Dispatching elves...");
        let n = first_house_with_n_presents(presents);
        println!("First house with {} presents: {}", presents, n);

        println!("");
        println!("Dispatching lazy elves...");
        let n = first_house_with_n_presents_limited(presents);
        println!("First house with enough presents with lazy elves: {}", n);
    } else {
        println!("Couldn't parse your input, sorry")
    }
}
