use std::str::FromStr;

extern crate util;
use util::get_multiline_input;

extern crate day17lib;
use day17lib::EggnogFiller;

const EGGNOG: u8 = 150;

fn main() {
    let lines = get_multiline_input("Container size, one container per line, EOF to end:").unwrap();
    let mut containers = Vec::new();
    for line in lines.split('\n') {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if let Ok(container) = u8::from_str(line) {
            containers.push(container);
        }
    }

    let filler = EggnogFiller::new(EGGNOG, containers);
    let combo_count = filler.clone().count();
    println!("Possible combinations: {}", combo_count);

    let min_ctrs = filler.clone().map(|c| c.len()).min().unwrap();
    let ways_min = filler.clone().filter(|c| c.len() == min_ctrs).count();

    println!("..with {} ways to use only {} containers.",
             ways_min,
             min_ctrs);
}
