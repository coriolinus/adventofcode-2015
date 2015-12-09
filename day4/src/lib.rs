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

use std::sync::mpsc::{channel, Sender};
use std::thread;

const WORK_SIZE: u64 = 1024;
const MIN_ZEROES: usize = 5;

pub fn mine_coin(secret: &str) -> Option<u64> {
    let cpus = num_cpus::get();
    mine_coin_with_cores(secret, cpus)
}

pub fn mine_coin_with_cores(secret: &str, cpus: usize) -> Option<u64> {
    // set up the results channel
    let (result_tx, result_rx) = channel();

    // an iterator handing out units of work which dies before overflow
    let next_work_iter = (0..).map(|x| x * WORK_SIZE).take_while(|x| x < &std::u64::MAX);

    // launch worker threads
    for _ in 0..cpus {
        let secret = secret.to_owned();
        let result_tx = result_tx.clone();
        thread::spawn(move || {
            mine(&secret, result_tx);
        });
    }

    // send work to threads
    for next_work in next_work_iter {
        // consume one child-thread result and separate into the identifier and the data
        let (next_work_tx, result) = result_rx.recv().unwrap();
        if let Some(result) = result {
            // now just return our result
            return Some(result);
        } else {
            // send the next unit of work
            // panicing if the receiver isn't around to take it
            next_work_tx.send(next_work).unwrap();
        }
    }
    None
}

fn mine(secret: &str, result: Sender<(Sender<u64>, Option<u64>)>) {
    let mut md5 = Md5::new();

    // create the transmission channel
    let (next_work_tx, next_work_rx) = channel();
    // send this transmission channel so we can get our first unit of work
    // and panic if the original thread isn't around to receive it
    if !result.send((next_work_tx.clone(), None)).is_err() {
        // skip all this if we're already receiving errors
        loop {
            let begin_at = next_work_rx.recv().unwrap();
            for current in begin_at..(begin_at + WORK_SIZE) {

                let mix = &(secret.clone().to_string() + &current.to_string());

                md5.input_str(mix);
                let digest = md5.result_str();
                md5.reset();

                if result.send((next_work_tx.clone(),
                                {
                             if digest.chars().take(MIN_ZEROES).all(|c| c == '0') {
                                 Some(current)
                             } else {
                                 None
                             }
                         }))
                         .is_err() {
                    break;
                }
            }
        }
    }
}

// extern crate test;
#[cfg(test)]
mod tests {
    use super::*;
    // use test::Bencher;

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

    // Benchmarks disabled due to not compiling in the stable compiler (!)
    // #[bench]
    // fn bench_one_core(b: &mut Bencher) {
    //     b.iter(|| mine_coin_with_cores("coriolinus", 1));
    // }

    // #[bench]
    // fn bench_four_cores(b: &mut Bencher) {
    //     if num_cpus::get() >= 4 {
    //         b.iter(|| mine_coin_with_cores("coriolinus", 4));
    //     }
    // }

    // #[bench]
    // fn bench_all_cores(b: &mut Bencher) {
    //     let cpus = num_cpus::get();
    //     b.iter(|| mine_coin_with_cores("coriolinus", cpus));
    // }
}
