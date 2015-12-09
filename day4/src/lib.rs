//! # Day 4: The Ideal Stocking Stuffer
//!
//! Santa needs help mining some AdventCoins (very similar to bitcoins) to use as gifts for all the
//! economically forward-thinking little girls and boys.
//!
//! To do this, he needs to find MD5 hashes which, in hexadecimal, start with at least five zeroes.
//! The input to the MD5 hash is some secret key (your puzzle input, given below) followed by a
//! number in decimal. To mine AdventCoins, you must find Santa the lowest positive number (no
//! leading zeroes: 1, 2, 3, ...) that produces such a hash.
//!
//! For example:
//!
//! - If your secret key is abcdef, the answer is `609043`, because the MD5 hash of `abcdef609043`
//!   starts with five zeroes (`000001dbbfa...`), and it is the lowest such number to do so.
//! - If your secret key is `pqrstuv`, the lowest number it combines with to make an MD5 hash
//!   starting with five zeroes is `1048970`; that is, the MD5 hash of `pqrstuv1048970` looks like
//!   `000006136ef....`

extern crate num_cpus;
extern crate crypto;
use crypto::md5::Md5;
use crypto::digest::Digest;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

const WORK_SIZE: u64 = 1024;
const MIN_ZEROES: usize = 5;

pub fn mine_coin(secret: &str) -> Option<u64> {
    // TODO: multiprocessing!
    // let cpus = num_cpus::get();
    // for _ in 0..cpus {
    // }

    // in the meantime, here's a single-threaded thing to demonstrate the concept:

    let mut next_work = 0;
    let finished = AtomicBool::new(false);

    while next_work < (std::u64::MAX - WORK_SIZE) {
        let mining = miner(secret, &next_work, &finished);
        next_work += WORK_SIZE;
        if mining.is_some() {
            return mining;
        }
    }
    None
}

fn miner(secret: &str, begin_at: &u64, finished: &AtomicBool) -> Option<u64> {
    let mut md5 = Md5::new();
    for current in *begin_at..(*begin_at + WORK_SIZE) {
        if finished.load(Ordering::Acquire) {
            return None;
        }

        let mix = &(secret.clone().to_string() + &current.to_string());

        md5.input_str(mix);
        let digest = md5.result_str();
        md5.reset();

        if digest.chars().take(MIN_ZEROES).all(|c| c == '0') {
            finished.store(true, Ordering::Release);
            return Some(current);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_known(secret: &str, expected: u64) {
        let coin = mine_coin(secret);
        match coin {
            Some(val) => assert_eq!(val, expected),
            None => panic!("Failed to find known coin value"),
        }
    }

    #[test]
    #[ignore]
    fn test_examples() {
        test_known("abcdef", 609043);
        test_known("pqrstuv", 1048970);
    }
}
