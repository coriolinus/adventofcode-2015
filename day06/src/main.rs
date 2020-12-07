use util::get_multiline_input;

use day06::Lightable;
use day06::Lights;

fn main() {
    let input = get_multiline_input("Your EOF-terminated lines go here:\n").unwrap();
    let mut lights: Lights<bool> = Lights::new();
    lights.parse_lines(&input);
    let count = lights.count();
    println!("Lit: {}", count);

    let mut lights: Lights<u64> = Lights::new();
    lights.parse_lines(&input);
    let count = lights.count();
    println!("Brightness: {}", count);
}
