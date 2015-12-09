extern crate day3lib;
use day3lib::follow_santa;

use std::io;
use std::io::prelude::*;

fn main() {
    if let Ok(input) = get_input("Enter Santa's path now: ", false) {
        let cc = follow_santa(input);
        println!("Houses visited: {}", cc.trail.len());
    } else {
        println!("Failed to parse; try again later")
    }
}

fn get_input(prompt: &str, wait_for_eof: bool) -> io::Result<String> {
    print!("{}", prompt);

    // flush stdout explicitly so it shows up at the end of the line
    try!(io::stdout().flush());

    let mut input = String::new();
    if wait_for_eof {
        try!(io::stdin().read_to_string(&mut input));
    } else {
        try!(io::stdin().read_line(&mut input));
    }

    Ok(input) // wrap the output string in Ok to match our signature
}
