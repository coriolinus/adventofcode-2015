//! --- Day 1: Not Quite Lisp ---
//!
//! Santa was hoping for a white Christmas, but his weather machine's "snow" function is powered by stars, and he's fresh out! To save Christmas, he needs you to collect fifty stars by December 25th.
//!
//! Collect stars by helping Santa solve puzzles. Two puzzles will be made available on each day in the advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!
//!
//! Here's an easy puzzle to warm you up.
//!
//! Santa is trying to deliver presents in a large apartment building, but he can't find the right floor - the directions he got are a little confusing. He starts on the ground floor (floor 0) and then follows the instructions one character at a time.
//!
//! An opening parenthesis, (, means he should go up one floor, and a closing parenthesis, ), means he should go down one floor.
//!
//! The apartment building is very tall, and the basement is very deep; he will never find the top or bottom floors.
//!
//! For example:
//!
//! (()) and ()() both result in floor 0.
//! ((( and (()(()( both result in floor 3.
//! ))((((( also results in floor 3.
//! ()) and ))( both result in floor -1 (the first basement level).
//! ))) and )())()) both result in floor -3.
//! To what floor do the instructions take Santa?
//!
//! # Examples
//!
//! ```
//! assert_eq!( 0, count_parens("(())"    ));
//! assert_eq!( 0, count_parens("()()"    ));
//! assert_eq!( 3, count_parens("((()))"  ));
//! assert_eq!( 3, count_parens("(()(()(" ));
//! assert_eq!( 3, count_parens("))(((((" ));
//! assert_eq!(-1, count_parens("())"     ));
//! assert_eq!(-1, count_parens("))()"    ));
//! assert_eq!(-3, count_parens(")))"     ));
//! assert_eq!(-3, count_parens(")())())" ));
//! ```

use std::io;
use std::io::Write;

fn main() {
    let floor = count_parens(&get_input());
    println!("Floor: {}", floor);
}

fn get_input() -> String {
    print!("Input the parenthetical expression now: ");

    // flush stdout explicitly so it shows up at the end of the line
    io::stdout().flush().ok().expect("Could not flush stdout");

    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);

    input
}

/// Returns <number of open parens> - <number of close parens> in the given string
fn count_parens(input: &str) -> i32 {
    input.chars().fold(0, |sum, ch| if ch == '(' {sum + 1} else if ch == ')' {sum - 1} else {sum})
}
