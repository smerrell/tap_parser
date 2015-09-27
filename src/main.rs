extern crate regex;

use std::io::{self, BufRead};

mod tap;

// colorized output, eventually
// flag to disable colorized output
fn main() {
    let stdin = io::stdin();
    let iter = stdin.lock().lines();
    let mut parser = tap::TapHarness::new(tap::TapVersion::Thirteen);

    for line_res in iter {
        let line = line_res.unwrap();
        parser.read_line(&line.trim());
        println!("{}", &line.trim());
    }

    println!("{}", &parser.summarize());
}

