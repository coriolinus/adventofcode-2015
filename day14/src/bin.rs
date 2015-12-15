
extern crate util;
use util::get_multiline_input;

extern crate day14lib;
use day14lib::Reindeer;

const RACE_LEN: usize = 2503;

fn main() {
    let lines = get_multiline_input("Reindeer definitions here:").unwrap();
    let mut rs = Reindeer::parse_lines(&lines).unwrap();
    Reindeer::fast_forward(&mut rs, RACE_LEN);
    let winner = Reindeer::farthest(&rs).unwrap();
    println!("Winner (old scoring): {:?}", winner);

    Reindeer::reset_all(&mut rs);
    let (winner, pts) = Reindeer::new_race(&mut rs, RACE_LEN).unwrap();
    println!("Winner (new scoring): {:?}", winner);
    println!(" ...with {} points", pts);
}
