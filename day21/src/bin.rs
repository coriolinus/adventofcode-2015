extern crate day21lib;
use day21lib::cheapest_winning_loadout;
use day21lib::priciest_losing_loadout;
use day21lib::item_shop;

fn main() {
    let cheapest = cheapest_winning_loadout(&item_shop());
    match cheapest {
        None => println!("Couldn't find any winning loadout, sucker."),
        Some((l, w)) => {
            println!("Cheapest loadout and winner: ");
            println!("  {:?}", l);
            println!("  {:?}", w);
            println!("");
            println!("Loadout cost: {}", l.cost());
        }
    }

    let cheapest = priciest_losing_loadout(&item_shop());
    match cheapest {
        None => println!("Couldn't find any losing loadout, sucker."),
        Some((l, w)) => {
            println!("Priciest loadout and loser: ");
            println!("  {:?}", l);
            println!("  {:?}", w);
            println!("");
            println!("Loadout cost: {}", l.cost());
        }
    }
}
