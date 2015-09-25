extern crate regex;

use std::io::{self, BufRead};

mod tap;

fn main() {
    let stdin = io::stdin();
    let mut iter = stdin.lock().lines();

    // this is probably bad, bad, bad
    let version_line = &iter.next().unwrap().unwrap();
    println!("{:?}", version_line);
    let version = tap::read_version(&version_line);
    // build a parser on a version?
    println!("TAP Version: {:?}", version);

    for line in iter {
        println!("{}", line.unwrap().trim());
        // feed lines into the parser?
    }
}

