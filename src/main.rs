#[cfg(test)]
extern crate hamcrest;

extern crate regex;
extern crate term;

mod tap;

use std::io::{self, BufRead};
use tap::{TapHarness,TestState};

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
                let outcome = match res.state {
                    TestState::Passed => "✓",
                    TestState::Failed => "𐄂",
                    TestState::Skipped => "—",
                    TestState::Incomplete => "—",
                };

                match res.state {
                    TestState::Passed => term.fg(term::color::GREEN).unwrap(),
                    TestState::Failed => term.fg(term::color::RED).unwrap(),
                    TestState::Skipped => term.fg(term::color::YELLOW).unwrap(),
                    TestState::Incomplete => term.fg(term::color::YELLOW).unwrap(),
                };

                writeln!(term, "{} {}", outcome, res.name).unwrap();
            }
            None => {}
        }
        term.reset().unwrap();
    }

    println!("{}", &parser.summarize());
}

