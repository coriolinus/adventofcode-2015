//! # Day 20: Infinite Elves and Infinite Houses
//!
//! To keep the Elves busy, Santa has them deliver some presents by hand, door-to-door. He sends
//! them down a street with infinite houses numbered sequentially: 1, 2, 3, 4, 5, and so on.
//!
//! Each Elf is assigned a number, too, and delivers presents to houses based on that number:
//!
//!  - The first Elf (number 1) delivers presents to every house: 1, 2, 3, 4, 5, ....
//!  - The second Elf (number 2) delivers presents to every second house: 2, 4, 6, 8, 10, ....
//!  - Elf number 3 delivers presents to every third house: 3, 6, 9, 12, 15, ....
//!
//! There are infinitely many Elves, numbered starting with 1. Each Elf delivers presents equal to
//! ten times his or her number at each house.
//!
//! So, the first nine houses on the street end up like this:
//!
//! ```notrust
//! House 1 got 10 presents.
//! House 2 got 30 presents.
//! House 3 got 40 presents.
//! House 4 got 70 presents.
//! House 5 got 60 presents.
//! House 6 got 120 presents.
//! House 7 got 80 presents.
//! House 8 got 150 presents.
//! House 9 got 130 presents.
//! ```
//!
//! The first house gets `10` presents: it is visited only by Elf 1, which delivers `1 * 10 = 10`
//!  presents. The fourth house gets `70` presents, because it is visited by Elves 1, 2, and 4, for
//!  a total of `10 + 20 + 40 = 70` presents.
//!
//! What is the lowest house number of the house to get at least as many presents as the number in
//! your puzzle input?

use aoclib::parse;
use permutohedron::heap_recursive;

use std::{collections::HashSet, convert::TryInto, path::Path};

/// Usized floor of the square root of the input number
pub fn usqrt(num: u64) -> u64 {
    (num as f64).sqrt().floor() as u64
}

pub struct SieveOfErasthenes {
    pub primes: Vec<u64>,
    through: u64,
}

impl Default for SieveOfErasthenes {
    fn default() -> Self {
        Self::new()
    }
}

impl SieveOfErasthenes {
    pub fn new() -> SieveOfErasthenes {
        SieveOfErasthenes {
            primes: vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47],
            through: 47,
        }
    }

    /// Return a list of unique prime factors of `num`
    pub fn prime_factors(&mut self, num: u64) -> Vec<u64> {
        match num {
            0..=1 => Vec::new(),
            _ => {
                let sqrt_fl = usqrt(num);
                if sqrt_fl > self.through {
                    self.calc_through(sqrt_fl);
                }

                self.primes
                    .iter()
                    .take_while(|&p| p * p <= num)
                    .filter(|&p| num % p == 0)
                    .cloned()
                    .collect()
            }
        }
    }

    /// Return a list of all prime factors of `num`
    pub fn factorize_prime(&mut self, num: u64) -> Vec<u64> {
        match num {
            0 => Vec::new(),
            1 => vec![1],
            _ => {
                // initialize the return
                let mut ret = vec![];
                let mut quot = num;
                for p in self.prime_factors(num) {
                    while quot % p == 0 {
                        ret.push(p);
                        quot /= p;
                    }
                }
                ret
            }
        }
    }

    /// return a list of all factors of `num`
    pub fn factorize(&mut self, num: u64) -> Vec<u64> {
        let mut prime_factors = self.factorize_prime(num);
        let mut ret = HashSet::new();
        ret.extend(&prime_factors);

        let pl = prime_factors.len();
        heap_recursive(&mut prime_factors, |factor_ordering| {
            for how_many in 2..pl {
                ret.insert(factor_ordering.iter().take(how_many).product());
            }
        });

        ret.insert(1);
        let complements = ret.clone();
        for c in complements {
            ret.insert(num / c);
        }

        let mut r = ret.iter().cloned().collect::<Vec<u64>>();
        r.sort_unstable();
        r
    }

    /// calculate all primes <= num
    pub fn calc_through(&mut self, num: u64) {
        if num <= self.through {
            return;
        }

        for through in (self.through + 1)..(num + 1) {
            let mut potential_prime = true;
            for p in self.primes.iter().take_while(|&p| p * p <= through) {
                if through % p == 0 {
                    potential_prime = false;
                    break;
                }
            }
            if potential_prime {
                self.primes.push(through);
            }
        }
        self.through = num;
    }
}

pub fn presents_at(sieve: &mut SieveOfErasthenes, house: u64) -> u64 {
    let mut factors = HashSet::new();
    factors.extend(sieve.factorize(house));
    factors.iter().fold(0, |acc, item| acc + (10 * item))
}

pub fn first_house_with_n_presents(n: u64) -> Result<u64, Error> {
    // the brute force of memory way!
    let n = n.try_into()?;
    let stop = (n / 10) + 1;
    // we have an upper bound for the answer: even if nobody else stops there, elf `n/10` will
    // stop by and drop off that many right away
    let mut houses = vec![0; stop];
    for elf in 1..stop {
        for j in (1..).map(|jj| jj * elf).take_while(|jj| jj < &stop) {
            houses[j] += elf * 10;
        }
    }
    for (i, h) in houses.iter().enumerate() {
        if h >= &n {
            return Ok(i.try_into()?);
        }
    }
    Ok(0)
}

pub fn first_house_with_n_presents_limited(n: u64) -> Result<u64, Error> {
    // the brute force of memory way!
    let n = n.try_into()?;
    let stop = (n / 10) + 1;
    // we have an upper bound for the answer: even if nobody else stops there, elf `n/10` will
    // stop by and drop off that many right away
    let mut houses = vec![0; stop];
    for elf in 1..stop {
        for j in (1..)
            .map(|jj| jj * elf)
            .take_while(|jj| jj < &stop)
            .take(50)
        {
            houses[j] += elf * 11;
        }
    }
    for (i, h) in houses.iter().enumerate() {
        if h >= &n {
            return Ok(i.try_into()?);
        }
    }
    Ok(0)
}

pub fn part1(input: &Path) -> Result<(), Error> {
    for presents in parse::<u64>(input)? {
        println!(
            "First house with {} presents: {}",
            presents,
            first_house_with_n_presents(presents)?,
        );
    }
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    for presents in parse::<u64>(input)? {
        println!(
            "First house with {} presents with lazy elves: {}",
            presents,
            first_house_with_n_presents_limited(presents)?,
        );
    }
    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("value cannot fit into `usize` on this architecture")]
    Conversion(#[from] std::num::TryFromIntError),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sieve() {
        let expected = vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149,
        ];

        let mut e = SieveOfErasthenes::new();
        e.calc_through(150);
        assert_eq!(e.primes, expected);
    }

    #[test]
    fn test_presents() {
        let mut sieve = SieveOfErasthenes::new();
        let expected = vec![10, 30, 40, 70, 60, 120, 80, 150, 130];
        for (house, expect) in (1..).zip(expected) {
            println!("Expecting: House {} got {} presents", house, expect);
            println!("  Factors of {}: {:?}", house, sieve.factorize(house));
            println!("  Calculated presents: {}", presents_at(&mut sieve, house));
            assert_eq!(presents_at(&mut sieve, house), expect);
        }
    }

    #[test]
    fn test_factorize() {
        let mut sieve = SieveOfErasthenes::new();
        let expected = vec![
            1, 2, 3, 4, 5, 6, 8, 9, 10, 12, 15, 18, 20, 24, 30, 36, 40, 45, 60, 72, 90, 120, 180,
            360,
        ];
        assert_eq!(sieve.factorize(360), expected);
    }

    #[test]
    fn test_first_house_with_n() {
        for (input, output) in vec![(25, 2), (50, 4), (100, 6), (150, 8)] {
            assert_eq!(first_house_with_n_presents(input).unwrap(), output);
        }
    }
}
