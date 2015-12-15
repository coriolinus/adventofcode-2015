extern crate util;
use util::get_multiline_input;

extern crate day14lib;
use day14lib::Reindeer;

const FIRST_RACE_LEN: usize = 2503;

fn main() {
    let lines = get_multiline_input("Reindeer definitions here:").unwrap();
    let mut rs = Reindeer::parse_lines(&lines).unwrap();
    Reindeer::fast_forward(&mut rs, FIRST_RACE_LEN);
    let winner = Reindeer::farthest(&rs).unwrap();
    println!("Winner: {:?}", winner);
}
