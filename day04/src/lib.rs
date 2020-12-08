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

use aoc2015::parse;

use crypto::digest::Digest;
use crypto::md5::Md5;
use rayon::prelude::*;
use std::path::Path;
use thiserror::Error;

pub fn mine_coin(secret: &str, leading_zeros: usize) -> Option<u64> {
    (0..=u64::MAX)
        .into_par_iter()
        .map(|suffix| {
            let mut md5 = Md5::new();
            md5.input_str(secret);
            md5.input_str(&suffix.to_string());
            (suffix, md5.result_str())
        })
        .find_first(|(_suffix, digest)| {
            digest.chars().take_while(|&ch| ch == '0').count() >= leading_zeros
        })
        .map(|(suffix, _digest)| suffix)
}

pub fn part1(input: &Path) -> Result<(), Error> {
    for (row, secret) in parse::<String>(input)?.enumerate() {
        let coin = mine_coin(&secret, 5).ok_or(Error::NoCoin(secret))?;
        println!("row {} coin (5 leading): {}", row, coin);
    }
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    for (row, secret) in parse::<String>(input)?.enumerate() {
        let coin = mine_coin(&secret, 6).ok_or(Error::NoCoin(secret))?;
        println!("row {} coin (6 leading): {}", row, coin);
    }
    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("no coin found for prefix \"{0}\"")]
    NoCoin(String),
}

/// these tests are by default ignored when built with debug, and not ignored when built with release
#[cfg(test)]
mod tests {
    use super::*;
    // use test::Bencher;

    fn test_known(secret: &str, expected: u64, leading_zeros: usize) {
        let coin = mine_coin(secret, leading_zeros).unwrap();
        assert_eq!(coin, expected);
    }

    #[test]
    #[cfg_attr(debug_assertions, ignore)]
    fn test_first_example() {
        test_known("abcdef", 609043, 5);
    }

    #[test]
    #[cfg_attr(debug_assertions, ignore)]
    fn test_second_example() {
        test_known("pqrstuv", 1048970, 5);
    }

    #[test]
    #[cfg_attr(debug_assertions, ignore)]
    fn test_with_six() {
        test_known(
            "bgvyzdsv",
            1038736,
            6,
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
