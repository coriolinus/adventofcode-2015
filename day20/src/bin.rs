extern crate util;
use util::get_line_input;

extern crate day20lib;
use day20lib::{SieveOfErasthenes, presents_at, usqrt};

fn main() {
    let presents_s = get_line_input("Find the first house with this many presents or more: ")
                         .unwrap();

    if let Ok(presents) = usize::from_str_radix(&presents_s.trim(), 10) {
        let mut sieve = SieveOfErasthenes::new();
        println!("Finding prime numbers...");
        println!("");
        sieve.calc_through(usqrt(presents));

        println!("Little birdie says 665280.");
        println!(" .. Found {} presents there",
                 presents_at(&mut sieve, 665280));
        println!(" .. by factorizing {} to {:?}",
                 665280,
                 sieve.factorize(665280));
        println!("");


        for house in 10.. {
            let pat = presents_at(&mut sieve, house);
            if pat >= presents {
                println!("Found {} presents at house {},", pat, house);
                println!(" .. {} presents at the previous house,",
                         presents_at(&mut sieve, house - 1));
                println!(" .. {} presents at the previous house,",
                         presents_at(&mut sieve, house - 2));
                println!(" .. by factorizing {} to {:?}",
                         house,
                         sieve.factorize(house));
                break;
            }
        }
    } else {
        println!("Couldn't parse your input, sorry")
    }
}
