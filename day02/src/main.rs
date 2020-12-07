use day02::GiftBox;

use std::io;
use std::io::prelude::*;

fn main() {
    let mut paper = 0;
    let mut ribbon = 0;

    let input: io::Result<String> = get_input(
        "Enter boxes, separated by '\\n', terminated by \
                                               EOF: \n",
    );
    let input = match input {
        Ok(x) => x,
        Err(_) => return,
    };

    for line in input.split("\n") {
        let line = line.trim();
        if line.len() == 0 {
            continue;
        }
        if let Ok(g) = GiftBox::parse(line) {
            paper += g.paper();
            ribbon += g.ribbon();
        } else {
            println!("Failed to parse '{}'; aborting", line);
            return;
        }
    }

    println!("Total paper required: {}", paper);
    println!("Total ribbon required: {}", ribbon);
}

fn get_input(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);

    // flush stdout explicitly so it shows up at the end of the line
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    Ok(input)
}
