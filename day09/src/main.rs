use util::get_multiline_input;

use day09::Routes;

fn main() {
    let lines = get_multiline_input("Enter routes, then EOF: ").unwrap();
    let routes = Routes::parse_routes(&lines);
    let shortest = routes.find_shortest();
    println!("Shortest route: {:?}", shortest);
    let longest = routes.find_longest();
    println!("Longest route: {:?}", longest);
}
