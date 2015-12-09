use std::io;
use std::io::prelude::*;

pub fn get_input(prompt: &str, wait_for_eof: bool) -> io::Result<String> {
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

pub fn get_line_input(prompt: &str) -> io::Result<String> {
    get_input(prompt, false)
}

pub fn get_multiline_input(prompt: &str) -> io::Result<String> {
    get_input(prompt, true)
}
