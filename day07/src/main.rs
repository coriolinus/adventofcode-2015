use util::get_multiline_input;

use day07::parse::Parseable;
use day07::wire::Wire;
use day07::{evaluate, parse_wires};

fn main() {
    let input = get_multiline_input("Your EOF-terminated lines go here:\n").unwrap();
    if let Some(wires) = parse_wires(&input) {
        let sym_table = evaluate(&wires);
        if let Some(val) = sym_table.get("a") {
            println!("Value of wire 'a': {}", val);
            let mut new_wires = wires.clone();
            // eliminate whatever 'b' was before
            new_wires = new_wires
                .iter()
                .filter(|&x| x.get_name() != "b")
                .map(|x| x.to_owned())
                .collect();

            // insert a new 'b' wire containing this computed value
            let wire_string = format!("{} -> b", val);
            new_wires.push(Wire::parse(&wire_string).unwrap());
            let st = evaluate(&new_wires);
            println!("New value of wire 'a': {}", st.get("a").unwrap());
        } else {
            println!("Symbol 'a' did not appear in the final table!");
        }
    } else {
        println!("Could not parse that input");
    }
}
