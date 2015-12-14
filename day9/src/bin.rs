extern crate util;
use util::get_multiline_input;

extern crate day9lib;
use day9lib::Routes;

fn main() {
    let lines = get_multiline_input("Enter routes, then EOF: ").unwrap();
    let shortest = Routes::parse_routes(&lines).find_shortest();
    println!("Shortest route: {:?}", shortest)
}
