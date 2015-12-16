extern crate util;
use util::get_multiline_input;

extern crate day16lib;
use day16lib::{check_sues, check_sues_retro, mfcsam_result};

fn main() {
    let lines = get_multiline_input("Tell me about your Aunts Sue (and end with EOF):").unwrap();
    let mfcsam = mfcsam_result();

    let sues = check_sues(&mfcsam, &lines);
    println!("These Sues could match:");
    for sue in sues {
        println!(" - {:?}", sue);
    }

    println!("");
    let sues = check_sues_retro(&mfcsam, &lines);
    println!("These Sues could match (retro):");
    for sue in sues {
        println!(" - {:?}", sue);
    }
}
