use util::get_multiline_input;

use day18::LightGrid;

const ITERATIONS: u8 = 100;

fn main() {
    let lines = get_multiline_input("Initial Light State:").unwrap();
    let grid = LightGrid::parse_lines(&lines);
    if let Some(mut grid) = grid {
        for _ in 0..ITERATIONS {
            grid = grid.next_state();
        }
        println!(
            "{} lights on after {} iterations",
            grid.count_on(),
            ITERATIONS
        );

        grid = LightGrid::parse_lines_stuck(&lines).unwrap();
        for _ in 0..ITERATIONS {
            grid = grid.next_state_stuck();
        }
        println!(
            "When stuck, {} lights on after {} iterations",
            grid.count_on(),
            ITERATIONS
        );
    } else {
        println!("Couldn't parse the input lines as a LightGrid!")
    }
}
