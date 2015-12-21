extern crate util;
use util::get_multiline_input;

extern crate day19lib;
use day19lib::parse_replacements;
use day19lib::TransformEnumerator;
use day19lib::countdistinct::CountDistinct;

fn main() {
    let lines = get_multiline_input("Transforms, each on a line. Two newlines. An input string. \
                                     EOF.")
                    .unwrap();

    let mut separator = lines.split("\n\n");
    if !separator.clone().count() >= 2 {
        separator = lines.split("\r\n\r\n")
    }
    let mut sn = separator.next();
    if sn.is_none() {
        println!("Couldn't split the input properly!");
        return;
    }
    let transform_lines = sn.unwrap().trim();
    sn = separator.next();
    if sn.is_none() {
        println!("Couldn't find the input!");
        println!("");
        println!("Lines so far: {:?}", transform_lines);
        return;
    }
    let input = sn.unwrap().trim();

    if let Some(transforms) = parse_replacements(transform_lines) {
        let te = TransformEnumerator::new(&transforms, input);
        println!("Found {} distinct replacements", te.count_distinct());
    } else {
        println!("Couldn't parse your transforms.")
    }
}
