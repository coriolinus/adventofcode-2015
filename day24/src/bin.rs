use std::str::FromStr;

extern crate util;
use util::get_multiline_input;

extern crate day24lib;
use day24lib::{Package, SleighConfigurations};

fn main() {
    let lines = get_multiline_input("Enter your packages here, one per line, EOF-term'd:").unwrap();

    let mut packages = Vec::new();
    for line in lines.split("\n") {
        let line = line.trim();
        if !line.is_empty() {
            if let Ok(r) = Package::from_str(line) {
                packages.push(r);
            } else {
                println!("Could not parse \"{}\" as Package; aborting", line);
                packages = Vec::new();
                break;
            }
        }
    }

    if packages.len() > 0 {
        if let Some(best) = SleighConfigurations::best(packages.clone(), false) {
            println!("Best config: {:?}", best);
            println!(" with QE: {:?}", best.foot_qe());
        } else {
            println!("Could not determine an appropriate loading for the following packages: {:?}",
                     packages);
        }
        if let Some(best) = SleighConfigurations::best(packages.clone(), true) {
            println!("Best config (with trunk): {:?}", best);
            println!(" with QE: {:?}", best.foot_qe());
        } else {
            println!("Could not determine an appropriate loading for the following packages: {:?}",
                     packages);
        }
    }
}
