use util::get_line_input;

use day04::mine_coin;

fn main() {
    if let Ok(input) = get_line_input("Enter your secret key here: ") {
        let input = input.trim();
        if let Some(result) = mine_coin(&input) {
            println!("Coin: {}", result);
        } else {
            println!("No coin found")
        }
    }
}
