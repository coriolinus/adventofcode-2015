extern crate util;
use util::get_multiline_input;

extern crate day16lib;
use day16lib::{check_sues, mfcsam_result};

fn main() {
    let lines = get_multiline_input("Tell me about your Aunts Sue (and end with EOF):").unwrap();
    let mfcsam = mfcsam_result();

    let sues = check_sues(mfcsam, &lines);
    println!("These Sues could match:");
    for sue in sues {
        println!(" - {:?}", sue);
    }
}
