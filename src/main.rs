extern crate regex;

use std::io::{self, BufRead};

mod tap;

// colorized output, eventually
// flag to disable colorized output
fn main() {
    let stdin = io::stdin();
    let mut iter = stdin.lock().lines();

    // this is probably bad, bad, bad
    let version_line = &iter.next().unwrap().unwrap();
    let version = tap::read_version(&version_line);

    let mut parser = tap::TapParser::new(version);

    for line_res in iter {
        let line = line_res.unwrap();
        parser.read_line(&line.trim());
    }

    println!("{}", &parser.summarize());
}

