extern crate util;
use util::get_multiline_input;

extern crate day19lib;
use day19lib::parse_replacements;
use day19lib::TransformEnumerator;
use day19lib::countdistinct::CountDistinct;
use day19lib::fabricate_steps_count;

fn main() {
    let lines = get_multiline_input("Transforms, each on a line. Two newlines. An input string. \
                                     EOF.")
                    .unwrap();

    let mut separator = lines.split({
        match lines.find("\r\n") {
            Some(_) => "\r\n\r\n",
            None => "\n\n",
        }
    });

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

        println!("");
        println!("Searching for fabrication sequence...");
        if let Some(fs) = fabricate_steps_count(&transforms, input) {
            println!("... found fabrication sequence in {} steps", fs);
        } else {
            println!("... but couldn't find an assembly sequence.")
        }

    } else {
        println!("Couldn't parse your transforms.")
    }
}
