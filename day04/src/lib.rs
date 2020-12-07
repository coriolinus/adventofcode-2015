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

use crypto::digest::Digest;
use crypto::md5::Md5;

use std::sync::mpsc::{channel, Sender};
use std::thread;

const WORK_SIZE: u64 = 1024;
const DEFAULT_MIN_ZEROES: usize = 5;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct CoinMiningConfig {
    leading_zeros: usize,
    cpus: usize,
}

fn overthink_cpus() -> usize {
    let nc = num_cpus::get();
    match nc {
        4 => 2,
        8 => 4,
        12 => 6,
        16 => 8,
        _ => nc,
    }
}

impl Default for CoinMiningConfig {
    fn default() -> CoinMiningConfig {
        CoinMiningConfig {
            leading_zeros: DEFAULT_MIN_ZEROES,
            cpus: overthink_cpus(),
        }
    }
}

impl CoinMiningConfig {
    pub fn new(leading_zeros: usize, cpus: usize) -> CoinMiningConfig {
        CoinMiningConfig {
            leading_zeros: leading_zeros,
            cpus: cpus,
        }
    }

    pub fn leading_zeros(&self, leading_zeros: usize) -> CoinMiningConfig {
        CoinMiningConfig {
            leading_zeros: leading_zeros,
            ..*self
        }
    }

    pub fn cpus(&self, cpus: usize) -> CoinMiningConfig {
        CoinMiningConfig {
            cpus: cpus,
            ..*self
        }
    }
}

pub fn mine_coin(secret: &str) -> Option<u64> {
    mine_coin_with_conf(secret, CoinMiningConfig::default())
}

pub fn mine_coin_with_conf(secret: &str, conf: CoinMiningConfig) -> Option<u64> {
    // set up the results channel
    let (result_tx, result_rx) = channel();

    // an iterator handing out units of work which dies before overflow
    let next_work_iter = (0..)
        .map(|x| x * WORK_SIZE)
        .take_while(|x| x < &std::u64::MAX);

    // launch worker threads
    for _ in 0..conf.cpus {
        let secret = secret.to_owned();
        let result_tx = result_tx.clone();
        thread::spawn(move || {
            mine(&secret, conf.leading_zeros, result_tx);
        });
    }

    // send work to threads
    for next_work in next_work_iter {
        // consume one child-thread result and separate into the identifier and the data
        let (next_work_tx, result) = result_rx.recv().unwrap();
        if let Some(result) = result {
            // now just return our result
            // return Some(result);
            //
            // The above doesn't just work in all cases. It often does, but occasionally a thread
            // with a mucn higher unit of work will return successfully before the thread with
            // the correct answer.
            //
            // The solution to this implies a slowdown, unfortunately. We have to collect up all
            // the threads' results and find which of them is minimal, then return that.
            let mut results = vec![Some(result)];

            // collect all remaining threads
            for _ in 1..conf.cpus {
                let (_, result) = result_rx.recv().unwrap();
                results.push(result);
            }

            return results
                .iter()
                .filter(|&x| x.is_some())
                .map(|&x| x.unwrap())
                .min();
        } else {
            // send the next unit of work
            // panicing if the receiver isn't around to take it
            next_work_tx.send(next_work).unwrap();
        }
    }
    None
}

fn mine(secret: &str, leading_zeros: usize, result: Sender<(Sender<u64>, Option<u64>)>) {
    let mut md5 = Md5::new();

    // create the transmission channel
    let (next_work_tx, next_work_rx) = channel();
    // send this transmission channel so we can get our first unit of work
    // and panic if the original thread isn't around to receive it
    if !result.send((next_work_tx.clone(), None)).is_err() {
        // skip all this if we're already receiving errors
        loop {
            let begin_at = next_work_rx.recv().unwrap();

            let mut this_work_result = None;

            for current in begin_at..(begin_at + WORK_SIZE) {
                md5.input_str(secret);
                md5.input_str(&current.to_string());
                let digest = md5.result_str();
                md5.reset();

                if digest.chars().take(leading_zeros).all(|c| c == '0') {
                    this_work_result = Some(current);
                    break;
                }
            }

            if result
                .send((next_work_tx.clone(), this_work_result))
                .is_err()
            {
                break;
            }
        }
    }
}

//
#[cfg(test)]
mod tests {
    use super::*;
    // use test::Bencher;

    fn test_known(secret: &str, expected: u64) {
        test_known_with_conf(secret, expected, CoinMiningConfig::default())
    }

    fn test_known_with_conf(secret: &str, expected: u64, conf: CoinMiningConfig) {
        let coin = mine_coin_with_conf(secret, conf);
        match coin {
            Some(val) => assert_eq!(val, expected),
            None => panic!("Failed to find known coin value"),
        }
    }

    #[test]
    #[ignore]
    fn test_first_example() {
        test_known("abcdef", 609043);
    }

    #[test]
    #[ignore]
    fn test_second_example() {
        test_known("pqrstuv", 1048970);
    }

    #[test]
    #[ignore]
    fn test_with_six() {
        test_known_with_conf(
            "bgvyzdsv",
            1038736,
            CoinMiningConfig::default().leading_zeros(6),
        );
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
