use day03::follow_n_santas;
use day03::follow_santa;
use day03::unique_houses;

use std::io;
use std::io::prelude::*;

fn main() {
    if let Ok(input) = get_input("Enter Santa's path now: ", false) {
        let cc = follow_santa(&input);
        println!("Houses visited (1 santa): {}", cc.trail.len());
        let uh = unique_houses(&follow_n_santas(&input, 2));
        println!("Houses visited (2 santas): {}", uh);
    } else {
        println!("Failed to parse; try again later")
    }
}

fn get_input(prompt: &str, wait_for_eof: bool) -> io::Result<String> {
    print!("{}", prompt);

    // flush stdout explicitly so it shows up at the end of the line
    io::stdout().flush()?;

    let mut input = String::new();
    if wait_for_eof {
        io::stdin().read_to_string(&mut input)?;
    } else {
        io::stdin().read_line(&mut input)?;
    }

    Ok(input) // wrap the output string in Ok to match our signature
}
