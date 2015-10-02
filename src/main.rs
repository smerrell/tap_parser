#[cfg(test)]
extern crate hamcrest;

extern crate regex;
extern crate term;

mod tap;

use std::io::{self, BufRead};
use tap::{TapHarness};

// colorized output, eventually
// flag to disable colorized output
fn main() {
    let stdin = io::stdin();
    let iter = stdin.lock().lines();
    let mut parser = TapHarness::new();
    let mut term = term::stdout().unwrap();

    for line_res in iter {
        let line = line_res.unwrap();
        let result = parser.read_line(&line.trim());

        match result {
            Some(res) => {
                // will need to figure out skipped and incomplete tests
                let outcome = if res.passed { "✓ Passed" } else { "𐄂 Failed" };
                if res.passed {
                    term.fg(term::color::GREEN).unwrap();
                } else {
                    term.fg(term::color::RED).unwrap();
                }
                writeln!(term, "{} - {}", outcome, res.name).unwrap();
            }
            None => {}
        }
        term.reset().unwrap();
    }

    println!("{}", &parser.summarize());
}

