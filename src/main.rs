#![feature(drain)]

#[cfg(test)]
extern crate hamcrest;

extern crate regex;
extern crate term;

mod tap;

use std::io::{self, BufRead};
use tap::{TapHarness,TestState};

// flag to disable colorized output
fn main() {
    let stdin = io::stdin();
    let stdin_lines = stdin.lock().lines();
    let mut term = term::stdout().unwrap();
    let mut harness = TapHarness::new();

    for line_res in stdin_lines {
        let line = line_res.unwrap();
        let result = harness.read_line(&line.trim());

        result.map(|res| {
            let mut term_color = term::color::YELLOW;
            let outcome = match res.state {
                TestState::Passed => {
                    term_color = term::color::GREEN;
                    "✓"
                },
                TestState::Failed => {
                    term_color = term::color::RED;
                    "𐄂"
                },
                _ => "—",
            };

            term.fg(term_color).unwrap();
            writeln!(term, "{} {}", outcome, res.name).unwrap();
            if let Some(diagnostic) = res.diagnostics {
                term.reset().unwrap();
                for message in diagnostic {
                    println!("  {}", message);
                }
            }
        });

        term.reset().unwrap();
    }

    println!("{}", &harness.summarize());
}

