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
    println!("TAP Version: {:?}", version);

    let mut parser = tap::TapParser::new(version);

    for line in iter {
        parser.read_line(&line.unwrap().trim());
    }

    println!("{}", &parser.summarize());
}

