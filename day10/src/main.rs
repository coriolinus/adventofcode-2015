use util::get_line_input;

use day10::look_and_say;

fn main() {
    let mut line = get_line_input("Look and Say seed: ")
        .unwrap()
        .trim()
        .to_string();
    for _ in 0..40 {
        line = look_and_say(&line);
    }
    println!("L&S len after 40: {}", line.len());
    for _ in 0..10 {
        line = look_and_say(&line);
    }
    println!("L&S len after 50: {}", line.len());
}
