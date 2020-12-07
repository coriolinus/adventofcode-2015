use day21::cheapest_winning_loadout;
use day21::item_shop;
use day21::priciest_losing_loadout;

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
