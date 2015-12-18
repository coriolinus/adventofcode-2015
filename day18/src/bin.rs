extern crate util;
use util::get_multiline_input;

extern crate day18lib;
use day18lib::LightGrid;

const ITERATIONS: u8 = 100;

fn main() {
    let lines = get_multiline_input("Initial Light State:").unwrap();
    let grid = LightGrid::parse_lines(&lines);
    if let Some(mut grid) = grid {
        for _ in 0..ITERATIONS {
            grid = grid.next_state();
        }
        println!("{} lights on after {} iterations",
                 grid.count_on(),
                 ITERATIONS);
    } else {
        println!("Couldn't parse the input lines as a LightGrid!")
    }

}
