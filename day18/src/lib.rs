//! # Day 18: Like a GIF For Your Yard
//!
//! After the million lights incident, the fire code has gotten stricter: now, at most ten thousand
//! lights are allowed. You arrange them in a 100x100 grid.
//!
//! Never one to let you down, Santa again mails you instructions on the ideal lighting
//! configuration. With so few lights, he says, you'll have to resort to animation.
//!
//! Start by setting your lights to the included initial configuration (your puzzle input).
//! A `#` means "on", and a `.` means "off".
//!
//! Then, animate your grid in steps, where each step decides the next configuration based on the current one. Each light's next state (either on or off) depends on its current state and the current states of the eight lights adjacent to it (including diagonals). Lights on the edge of the grid might have fewer than eight neighbors; the missing ones always count as "off".
//!
//! For example, in a simplified 6x6 grid, the light marked A has the neighbors numbered 1 through 8, and the light marked B, which is on an edge, only has the neighbors marked 1 through 5:
//!
//! ```notrust
//! 1B5...
//! 234...
//! ......
//! ..123.
//! ..8A4.
//! ..765.
//! ```
//!
//! The state a light should have next is based on its current state (on or off) plus the number of neighbors that are on:
//!
//! - A light which is on stays on when `2` or `3` neighbors are on, and turns off otherwise.
//! - A light which is off turns on if exactly `3` neighbors are on, and stays off otherwise.
//! All of the lights update simultaneously; they all consider the same current state before moving to the next.
//!
//! Here's a few steps from an example configuration of another 6x6 grid:
//!
//! ```notrust
//! Initial state:
//! .#.#.#
//! ...##.
//! #....#
//! ..#...
//! #.#..#
//! ####..
//!
//! After 1 step:
//! ..##..
//! ..##.#
//! ...##.
//! ......
//! #.....
//! #.##..
//!
//! After 2 steps:
//! ..###.
//! ......
//! ..###.
//! ......
//! .#....
//! .#....
//!
//! After 3 steps:
//! ...#..
//! ......
//! ...#..
//! ..##..
//! ......
//! ......
//!
//! After 4 steps:
//! ......
//! ......
//! ..##..
//! ..##..
//! ......
//! ......
//! ```
//!
//! After 4 steps, this example has four lights on.
//!
//! In your grid of 100x100 lights, given your initial configuration, how many lights are on after 100 steps?

pub struct LightGrid {
    lights: Vec<Vec<bool>>,
}

/// A grid of lights which animates according to rules inspired by Conway's Game of Life
///
/// Internally, this is stored as a `Vec<Vec<bool>>`. Note that addressing is
/// `.lights[y][x]`.
///
/// This implementation is relatively good for densely-populated fixed-edge-length grids, but
/// for sparse or infinite-size grids, a HashMap-based implementation would be better.
impl LightGrid {
    pub fn blank(edge: usize) -> LightGrid {
        LightGrid { lights: vec![vec![false; edge]; edge] }
    }

    pub fn parse_lines(lines: &str) -> Option<LightGrid> {
        let spl = lines.trim().split('\n');
        let edge = spl.clone().count();

        let mut ret = Vec::with_capacity(edge);

        for line in spl {
            let line = line.trim();
            if line.len() != edge {
                return None;
            }

            let mut rl = Vec::with_capacity(edge);
            for ch in line.chars() {
                match ch {
                    '.' => rl.push(false),
                    '#' => rl.push(true),
                    _ => return None,
                }
            }
            ret.push(rl);
        }
        Some(LightGrid { lights: ret })
    }

    pub fn parse_lines_stuck(lines: &str) -> Option<LightGrid> {
        if let Some(mut grid) = LightGrid::parse_lines(lines) {
            let end = grid.lights.len() - 1;
            for (x, y) in vec![(0, 0), (0, end), (end, 0), (end, end)] {
                grid.lights[y][x] = true;
            }
            Some(grid)
        } else {
            None
        }
    }

    // Returns the count of lights adjacent to the given `x, y` coordinate which are currently
    // turned on.
    //
    // If `x` or `y` is greater than `self.lights.len() - 1`, panic. That coordinate is invalid.
    fn count_adjacent_on(&self, x: usize, y: usize) -> u8 {
        let last = self.lights.len() - 1;

        if x > last || y > last {
            panic!("Invalid coordinate ({}, {}) doesn't fit in edge length {}!",
                   x,
                   y,
                   self.lights.len());
        }

        let mut xs = vec![x];
        if x != 0 {
            xs.push(x - 1);
        }
        if x != last {
            xs.push(x + 1);
        }

        let mut ys = vec![y];
        if y != 0 {
            ys.push(y - 1);
        }
        if y != last {
            ys.push(y + 1);
        }

        let mut ret = 0;
        for cx in &xs {
            for cy in &ys {
                if !((*cx == x) && (*cy == y)) {
                    if self.lights[*cy][*cx] {
                        ret += 1;
                    }
                }
            }
        }

        ret
    }

    pub fn next_state(&self) -> LightGrid {
        let mut ret = LightGrid::blank(self.lights.len());

        for x in 0..self.lights.len() {
            for y in 0..self.lights.len() {

                ret.lights[y][x] = if self.lights[y][x] {
                    // A light which is on stays on when 2 or 3 neighbors are on, and turns off otherwise.
                    match self.count_adjacent_on(x, y) {
                        2...3 => true,
                        _ => false,
                    }
                } else {
                    // A light which is off turns on if exactly 3 neighbors are on, and stays off otherwise.
                    match self.count_adjacent_on(x, y) {
                        3 => true,
                        _ => false,
                    }
                };
            }
        }

        ret
    }

    pub fn next_state_stuck(&self) -> LightGrid {
        let mut grid = self.next_state();

        let end = grid.lights.len() - 1;
        for (x, y) in vec![(0, 0), (0, end), (end, 0), (end, end)] {
            grid.lights[y][x] = true;
        }

        grid
    }

    pub fn count_on(&self) -> u16 {
        let mut ret = 0;
        for x in 0..self.lights.len() {
            for y in 0..self.lights.len() {
                if self.lights[y][x] {
                    ret += 1;
                }
            }
        }
        ret
    }

    pub fn to_string(&self) -> String {
        let mut ret = String::new();
        for y in 0..self.lights.len() {
            for x in 0..self.lights.len() {
                ret.push_str(if self.lights[y][x] {
                    "#"
                } else {
                    "."
                });
            }
            ret.push_str("\n");
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example() -> LightGrid {
        let lines = [".#.#.#", "...##.", "#....#", "..#...", "#.#..#", "####.."];
        let mut ret = String::new();
        for line in lines.iter() {
            ret.push_str(line);
            ret.push_str("\n");
        }
        LightGrid::parse_lines(&ret).unwrap()
    }

    #[test]
    fn test_example() {
        let mut grid = get_example();
        assert_eq!(grid.count_on(), 15);
        println!("");
        println!("Initial State:");
        println!("{}", grid.to_string());

        // 1st step
        grid = grid.next_state();
        println!("After 1st step:");
        println!("{}", grid.to_string());
        assert_eq!(grid.count_on(), 11);

        // 2nd step
        grid = grid.next_state();
        println!("After 2nd step:");
        println!("{}", grid.to_string());
        assert_eq!(grid.count_on(), 8);

        // 3rd step
        grid = grid.next_state();
        println!("After 3rd step:");
        println!("{}", grid.to_string());
        assert_eq!(grid.count_on(), 4);

        // 4th step
        grid = grid.next_state();
        println!("After 4th step:");
        println!("{}", grid.to_string());
        assert_eq!(grid.count_on(), 4);
    }

    #[test]
    fn test_ex_first_row_count() {
        let example = get_example();
        let expected: Vec<u8> = vec![1, 0, 3, 2, 4, 1];
        let found = (0..6)
                        .map(|x| example.count_adjacent_on(x, 0))
                        .collect::<Vec<_>>();
        assert_eq!(found, expected);
    }
}
