#[cfg(test)]
extern crate hamcrest;

extern crate regex;

mod tap;

use std::io::{self, BufRead};
use tap::{TapHarness};

// colorized output, eventually
// flag to disable colorized output
fn main() {
    let stdin = io::stdin();
    let iter = stdin.lock().lines();
    let mut parser = TapHarness::new();

    for line_res in iter {
        let line = line_res.unwrap();
        let result = parser.read_line(&line.trim());

        match result {
            Some(res) => {
                // will need to figure out skipped and incomplete tests
                let outcome = if res.passed { "✓ Passed" } else { "𐄂 Failed" };
                println!("{} - {}", outcome, res.name);
            }
            None => {}
        }
    }

    println!("{}", &parser.summarize());
}

